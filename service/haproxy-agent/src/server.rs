use anyhow::{anyhow, Result};
use futures::StreamExt;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::rc::Rc;
use std::time::Duration;
use thiserror::Error;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{Receiver, Sender};
use tokio_stream::wrappers::ReceiverStream;
use tracing::*;

type Client = (TcpStream, SocketAddr);

pub fn serve(systemd: cd_systemd::SystemdConnection, params: ServerParameters) {
    let rt = tokio::runtime::Runtime::new().expect("Failed to initialize async runtime");

    let (sender, receiver) = tokio::sync::mpsc::channel(1);

    rt.block_on(async {
        tokio::spawn(listen(params.listen_addr, sender));

        handle(systemd, receiver, params.client_timeout, params.max_clients).await
    });
}

#[instrument(skip(sender))]
async fn listen(listen_addr: SocketAddr, sender: Sender<(TcpStream, SocketAddr)>) -> anyhow::Result<()> {
    let listener = TcpListener::bind(listen_addr).await.map_err(|e| anyhow!(e))?;
    info!("started listening on {}", listen_addr);

    loop {
        let client = listener.accept().await?;
        sender.send(client).await?;
    }
}

async fn handle(
    systemd: cd_systemd::SystemdConnection,
    clients: Receiver<Client>,
    timeout: Duration,
    max_clients: usize,
) {
    let systemd = Rc::new(systemd);
    ReceiverStream::new(clients)
        .for_each_concurrent(max_clients, |client| async {
            let span = tracing::debug_span!("client", client = ?client.1);
            let systemd = systemd.clone();
            async move {
                let handle_client = handle_client(&systemd, client);
                let timeout_result = tokio::time::timeout(timeout, handle_client).await;
                match timeout_result {
                    Err(_) => debug!("Client timed out"),
                    Ok(Err(e)) => warn!("Client failed: {}", e),
                    Ok(Ok(())) => {} // success
                }
            }
            .instrument(span)
            .await
        })
        .await;
}

async fn handle_client(systemd: &cd_systemd::SystemdConnection, client: Client) -> Result<(), HandleClientError> {
    let mut stream = client.0;
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

    let service_status = systemd.service_get_state(&service_name)?;

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

pub struct ServerParameters {
    pub listen_addr: SocketAddr,
    pub client_timeout: Duration,
    pub max_clients: usize,
}

impl Default for ServerParameters {
    fn default() -> Self {
        Self {
            listen_addr: SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 13370)),
            client_timeout: Duration::from_secs(2),
            max_clients: 1024,
        }
    }
}
