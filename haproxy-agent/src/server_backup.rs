use anyhow::{anyhow, Result};
use derive_new::new;
use futures::StreamExt;
use std::net::SocketAddr;
use std::time::Duration;
use thiserror::Error;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::Sender;
use tokio_stream::wrappers::ReceiverStream;
use tracing::*;

#[derive(new)]
pub struct Server {
    systemd: cd_systemd::SystemdConnection,
    max_clients: usize,
    timeout: Duration,
    listen_addr: SocketAddr,
}

impl Server {
    pub fn run(&self) {
        let rt = tokio::runtime::Runtime::new().expect("Failed to initialize async runtime");

        rt.block_on(async {
            let (sender, receiver) = tokio::sync::mpsc::channel(5);

            tokio::spawn(self.listen(sender));

            ReceiverStream::new(receiver)
                .for_each_concurrent(self.max_clients, |mut client| async move {
                    let handle_client = self.handle_client(&mut client.0, &client.1);
                    if let Err(_) = tokio::time::timeout(self.timeout, handle_client).await {
                        debug!("Client {:?} timed out", &client.1);
                    }
                })
                .await;
        });
    }

    #[instrument(skip(self))]
    async fn listen(&self, sender: Sender<(TcpStream, SocketAddr)>) -> anyhow::Result<()> {
        let listener = TcpListener::bind(self.listen_addr).await.map_err(|e| anyhow!(e))?;
        info!(
            "listening on {}, max_connections={}, timeout={}ms",
            self.listen_addr,
            self.max_clients,
            self.timeout.as_millis()
        );
        loop {
            let client = listener.accept().await?;
            sender.send(client).await?;
        }
    }

    #[instrument(skip(self, stream))]
    async fn handle_client(
        &self,
        stream: &mut tokio::net::TcpStream,
        #[allow(unused_variables)] addr: &std::net::SocketAddr, // used by #[instrument]
    ) -> Result<(), HandleClientError> {
        let (read, mut write) = stream.split();
        let read = BufReader::new(read);

        let mut service_name = String::with_capacity(1024);
        let bytes_read = read
            .take(1024) // SEC: prevent client from keeping connection alive forever by sending non-CR garbage
            .read_line(&mut service_name)
            .await?;

        if bytes_read == 0 {
            return Err(HandleClientError::ClientDisconnected);
        }

        if service_name.pop() != Some('\n') {
            return Err(HandleClientError::ClientSentInvalidData);
        }

        use cd_haproxy_agent::State;
        use cd_systemd::ActiveState;

        let service_name: cd_systemd::UnitName = service_name
            .parse()
            .map_err(|_| HandleClientError::ClientSentInvalidData)?;

        let service_status = self.systemd.service_get_state(&service_name)?;

        debug!("service {} is {}", service_name, service_status);

        let state = match service_status {
            // stopped & not yet ready
            ActiveState::Inactive => State::Stopped(None),
            ActiveState::Activating => State::Stopped(None),
            // ready
            ActiveState::Active => State::Ready,
            ActiveState::Reloading => State::Ready,
            // draining
            ActiveState::Deactivating => State::Drain,
            // failure
            ActiveState::Failed => State::Fail(None),
        };

        use cd_haproxy_agent::AgentResponse;
        let response = AgentResponse {
            state: Some(state),
            ..AgentResponse::default()
        }
        .to_string();

        trace!("response: {:?}", response);

        write.write_all(response.as_bytes()).await?;

        Ok(())
    }
}

#[derive(Error, Debug)]
enum HandleClientError {
    #[error("Client disconnected before sending data")]
    ClientDisconnected,

    #[error("Client sent data that does not appear to be a service name followed by a newline character")]
    ClientSentInvalidData,

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    SystemdError(#[from] cd_systemd::SystemdError),
}
