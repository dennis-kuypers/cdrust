use crate::prelude::*;
use futures::StreamExt;
use reqwest::Client as ReqwestClient;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opts {
    pub host: String,
}

pub async fn health() -> anyhow::Result<()> {
    let client = ReqwestClient::builder();
    // TODO: Add possibility to add trusted cert
    // client = client.add_root_certificate(certificate);
    let client = client.build().context("Failed constructing HttpClient")?;

    let base_domain = "abc";

    let endpoints = ENDPOINTS.iter().map(|name| {
        let url = format!("https://{}.{}/healthcheck", name, base_domain);
        query_health(&client, name, url)
    });

    let checks: Vec<anyhow::Result<HealthCheck>> = futures::stream::iter(endpoints).buffer_unordered(4).collect().await;
    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HealthCheck {
    pub name: String,
    pub result: HealthCheckResult,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum HealthCheckResult {
    Ok(HealthCheckResponse),
    Failed(String),
}

type HealthCheckResponse = std::collections::HashMap<String, HealthSample>;

#[derive(Debug, Deserialize, Serialize)]
pub struct HealthSample {
    #[serde(rename = "timeStamp")]
    pub time_stamp: Option<i64>,
    #[serde(rename = "samplingTime")]
    pub stampling_time: Option<usize>,
    #[serde(rename = "value")]
    pub healthy: bool,
}

async fn query_health(client: &ReqwestClient, name: &'static str, url: String) -> anyhow::Result<HealthCheck> {
    trace!("Quering health for {}", url);
    //
    // let mut headers = reqwest::header::HeaderMap::new();
    // headers.insert(CONNECTION, "close".parse().unwrap());
    //
    // client
    //     .get(url)
    //     .headers(headers)
    //     .send()
    //     .map_err(|e| {
    //         Error::with_chain(
    //             e,
    //             ErrorKind::FailedQueryHeatlhCheck("failed to request health check from server".to_owned()),
    //         )
    //     })
    //     .and_then(|response| {
    //         trace!("Received response with status = {}.", response.status());
    //         let res = if response.status() == StatusCode::OK {
    //             Ok(response)
    //         } else {
    //             let reason = format!("of unexpected status code {} != 200", response.status());
    //             Err(Error::from_kind(ErrorKind::FailedQueryHeatlhCheck(reason)))
    //         };
    //         result(res)
    //     })
    //     .and_then(|response| {
    //         let body = response.into_body();
    //         body.concat2()
    //             .map_err(|e| Error::with_chain(e, ErrorKind::FailedQueryHeatlhCheck("failed to read body".to_owned())))
    //     })
    //     .and_then(|body| {
    //         let body = String::from_utf8_lossy(&body).to_string();
    //         trace!("Parsing body {:?}", &body);
    //         let res = serde_json::from_slice::<HealthCheckResponse>(&body.as_bytes()).map_err(|e| {
    //             Error::with_chain(
    //                 e,
    //                 ErrorKind::FailedQueryHeatlhCheck("failed to parse response".to_owned()),
    //             )
    //         });
    //         result(res)
    //     })
    //     .map(move |checks| HealthCheck {
    //         name: name.to_string(),
    //         result: HealthCheckResult::Ok(checks),
    //     })
    //     .or_else(move |e| {
    //         let reason = format!("{}", e);
    //         Ok(HealthCheck {
    //             name: name.to_string(),
    //             result: HealthCheckResult::Failed(reason),
    //         })
    //     })

    todo!()
}

pub const ENDPOINTS: &[&str] = &["admin", "api", "app", "auth", "public", "sales", "upload"];
