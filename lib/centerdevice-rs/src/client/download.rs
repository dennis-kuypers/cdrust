use crate::{
    client::{AuthorizedClient, GeneralErrHandler},
    errors::{Error, ErrorKind, Result},
    WithProgress,
};

use failure::Fail;
use log::debug;
use reqwest::{blocking::Response, header, StatusCode};
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    string::ToString,
};

#[derive(Debug)]
pub struct Download<'a> {
    document_id: &'a str,
    dir: &'a Path,
    filename: Option<&'a Path>,
}

impl<'a> Download<'a> {
    pub fn new(document_id: &'a str, dir: &'a Path) -> Download<'a> {
        Download {
            document_id,
            dir,
            filename: None,
        }
    }

    pub fn filename(self, filename: &'a Path) -> Download<'a> {
        Download {
            filename: Some(filename),
            ..self
        }
    }
}
struct ProgressWriter<'a, P: ?Sized, W> {
    progress: Option<&'a mut P>,
    inner: W,
}

impl<'a, P: WithProgress + ?Sized, W: Write> Write for ProgressWriter<'a, P, W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let amount = self.inner.write(buf)?;
        if let Some(ref mut p) = self.progress {
            p.progress(amount);
        }
        Ok(amount)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}

pub fn download_file(authorized_client: &AuthorizedClient, download: Download) -> Result<u64> {
    do_download(authorized_client, download, None::<&mut dyn WithProgress>)
}

pub fn download_file_with_progress<T: WithProgress + ?Sized>(
    authorized_client: &AuthorizedClient,
    download: Download,
    progress: &mut T,
) -> Result<u64> {
    do_download(authorized_client, download, Some(progress))
}

fn do_download<T: WithProgress + ?Sized>(
    authorized_client: &AuthorizedClient,
    download: Download,
    mut progress: Option<&mut T>,
) -> Result<u64> {
    let url = format!(
        "https://api.{}/v2/document/{}",
        authorized_client.base_url, download.document_id
    );

    let request = authorized_client
        .http_client
        .get(&url)
        .bearer_auth(&authorized_client.token.access_token);
    debug!("Request: '{:#?}'", request);

    let mut response = request
        .send()
        .map_err(|e| e.context(ErrorKind::HttpRequestFailed))?
        .general_err_handler(&[StatusCode::OK])?;
    debug!("Response: '{:#?}'", response);

    let status_code = response.status();
    let content_length = get_content_length(&response)?;
    let filename = if let Some(f_path) = download.filename {
        PathBuf::from(f_path)
    } else {
        let f_content_disposition = get_filename(&response)?;
        PathBuf::from(f_content_disposition)
    };
    debug!("Filename: {:#?}", filename);

    let mut file_path = PathBuf::from(&download.dir);
    file_path.push(filename);

    let file = File::create(file_path.as_path()).map_err(|e| {
        e.context(ErrorKind::FailedToProcessHttpResponse(
            status_code,
            "creating file".to_string(),
        ))
    })?;

    let mut writer = {
        if let Some(ref mut p) = progress {
            p.setup(content_length as usize);
        }
        let inner = BufWriter::new(file);
        ProgressWriter { progress, inner }
    };

    let len = response.copy_to(&mut writer).map_err(|e| {
        e.context(ErrorKind::FailedToProcessHttpResponse(
            status_code,
            "reading body".to_string(),
        ))
    })?;
    assert_eq!(content_length, len);

    if let Some(ref mut p) = writer.progress {
        p.finish();
    }

    Ok(len)
}

fn get_filename(response: &Response) -> Result<String> {
    // TODO: Upgrade to another version of mime_multifrom or replace because it uses hyper 0.10
    // headers and mime 0.2
    use hyperx::header::{ContentDisposition, DispositionParam, Header};
    use std::str;

    let status_code = response.status();

    let header = response
        .headers()
        .get(header::CONTENT_DISPOSITION)
        .ok_or_else(|| ErrorKind::FailedToProcessHttpResponse(status_code, "content disposition header".to_string()))?;
    let content_disposition: ContentDisposition = ContentDisposition::parse_header(&header).map_err(|e| {
        e.context(ErrorKind::FailedToProcessHttpResponse(
            status_code,
            "parsing content disposition header".to_string(),
        ))
    })?;

    let mut filename = None;
    for cp in &content_disposition.parameters {
        if let DispositionParam::Filename(_, _, ref f) = *cp {
            let decoded = str::from_utf8(f).map_err(|e| {
                e.context(ErrorKind::FailedToProcessHttpResponse(
                    status_code,
                    "parsing content disposition filename".to_string(),
                ))
            })?;
            filename = Some(decoded);
            break;
        }
    }
    filename
        .ok_or_else(|| {
            Error::from(ErrorKind::FailedToProcessHttpResponse(
                status_code,
                "content disposition header filename not found".to_string(),
            ))
        })
        .map(ToString::to_string)
}

fn get_content_length(response: &Response) -> Result<u64> {
    let status_code = response.status();
    let content_length = response
        .headers()
        .get(header::CONTENT_LENGTH)
        .ok_or_else(|| ErrorKind::FailedToProcessHttpResponse(status_code, "content length header".to_string()))?
        .to_str()
        .map_err(|e| {
            e.context(ErrorKind::FailedToProcessHttpResponse(
                status_code,
                "parsing content length header".to_string(),
            ))
        })?
        .parse::<u64>()
        .map_err(|e| {
            e.context(ErrorKind::FailedToProcessHttpResponse(
                status_code,
                "parsing content length".to_string(),
            ))
        })?;
    Ok(content_length)
}
