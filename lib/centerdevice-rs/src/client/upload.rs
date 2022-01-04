use crate::{
    client::{self, upload::internal::DocumentMetadata, AuthorizedClient, GeneralErrHandler},
    errors::{ErrorKind, Result},
};

use failure::Fail;
use hex;
use log::debug;
use mime::*;
use mime_multipart::{write_multipart, FilePart, Node, Part};
use reqwest::{blocking::Response, header, StatusCode};
use serde::{self, Deserialize};
use std::{borrow::Cow, path::Path};

#[derive(Debug)]
pub struct Upload<'a> {
    path: &'a Path,
    mime_type: Mime,
    filename: Cow<'a, str>,
    size: u64,
    title: Option<&'a str>,
    author: Option<&'a str>,
    tags: &'a [&'a str],
    collections: &'a [&'a str],
}

impl<'a> Upload<'a> {
    pub fn new(path: &'a Path, mime_type: Mime) -> Result<Upload<'a>> {
        let metadata = path.metadata().map_err(|e| {
            e.context(ErrorKind::FailedToPrepareHttpRequest(
                "reading file metadata".to_string(),
            ))
        })?;
        let filename = path
            .file_name()
            .ok_or_else(|| ErrorKind::FailedToPrepareHttpRequest("getting filename from path".to_string()))?
            .to_string_lossy();

        Ok(Upload {
            path,
            mime_type,
            filename,
            size: metadata.len(),
            title: None,
            author: None,
            tags: &[],
            collections: &[],
        })
    }

    pub fn with_title(path: &'a Path, mime_type: Mime, title: &'a str) -> Result<Upload<'a>> {
        Upload::new(path, mime_type).map(|u| u.title(title))
    }

    pub fn title(self, title: &'a str) -> Upload<'a> {
        Upload {
            title: Some(title),
            ..self
        }
    }

    pub fn author(self, author: &'a str) -> Upload<'a> {
        Upload {
            author: Some(author),
            ..self
        }
    }

    pub fn tags(self, tags: &'a [&str]) -> Upload<'a> {
        Upload { tags, ..self }
    }

    pub fn collections(self, collections: &'a [&str]) -> Upload<'a> {
        Upload { collections, ..self }
    }
}

pub(crate) mod internal {
    use serde::Serialize;

    #[derive(Serialize, Debug)]
    pub struct DocumentMetadata<'a> {
        metadata: Metadata<'a>,
    }

    #[derive(Serialize, Debug)]
    struct Metadata<'a> {
        document: Document<'a>,
        #[serde(skip_serializing_if = "Option::is_none")]
        actions: Option<Actions<'a>>,
    }

    #[derive(Serialize, Debug)]
    struct Document<'a> {
        filename: &'a str,
        size: u64,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<&'a str>,
        #[serde(skip_serializing_if = "Option::is_none")]
        author: Option<&'a str>,
    }

    #[derive(Serialize, Debug)]
    struct Actions<'a> {
        #[serde(skip_serializing_if = "Option::is_none", rename(serialize = "add-tag"))]
        tags: Option<&'a [&'a str]>,
        #[serde(skip_serializing_if = "Option::is_none", rename(serialize = "add-to-collection"))]
        collections: Option<&'a [&'a str]>,
    }

    impl<'a> DocumentMetadata<'a> {
        pub fn from_upload(u: &'a super::Upload<'a>) -> Self {
            let document = Document {
                filename: u.filename.as_ref(),
                size: u.size,
                title: u.title,
                author: u.author,
            };
            let actions = Actions {
                tags: Some(u.tags),
                collections: Some(u.collections),
            };
            let metadata = Metadata {
                document,
                actions: Some(actions),
            };

            DocumentMetadata { metadata }
        }
    }
}

#[derive(Debug, Deserialize)]
struct Id {
    id: client::ID,
}

pub fn upload_file(authorized_client: &AuthorizedClient, upload: Upload) -> Result<client::ID> {
    let url = format!("https://api.{}/v2/documents", authorized_client.base_url);

    let document_metadata = internal::DocumentMetadata::from_upload(&upload);

    // FIXME: Loads all the things into memory.
    // cf. https://github.com/seanmonstar/reqwest/issues/365
    // cf. https://github.com/seanmonstar/reqwest/issues/262
    let mut body: Vec<u8> = Vec::new();
    let nodes = create_multipart(&document_metadata, &upload)
        .map_err(|e| e.context(ErrorKind::FailedToPrepareHttpRequest("creating multipart".to_string())))?;
    let boundary = generate_boundary(&upload.filename.as_bytes());
    let content_type: Mime = mime!(Multipart / FormData; Boundary = (boundary));
    let _ = write_multipart(&mut body, &boundary.into_bytes(), &nodes)
        .map_err(|e| e.context(ErrorKind::FailedToPrepareHttpRequest("multipart".to_string())))?;

    let request = authorized_client
        .http_client
        .post(&url)
        .bearer_auth(&authorized_client.token.access_token)
        .header(header::CONTENT_TYPE, content_type.to_string().as_bytes())
        .header(
            header::ACCEPT,
            mime!(Application / Json; Charset = Utf8).to_string().as_bytes(),
        )
        .body(body);
    debug!("Request: '{:#?}'", request);

    let response: Response = request
        .send()
        .map_err(|e| e.context(ErrorKind::HttpRequestFailed))?
        .general_err_handler(&[StatusCode::CREATED])?;
    debug!("Response: '{:#?}'", response);

    let status = response.status();
    let result: Id = response.json().map_err(|e| {
        e.context(ErrorKind::FailedToProcessHttpResponse(
            status,
            "reading body".to_string(),
        ))
    })?;

    Ok(result.id)
}

fn create_multipart(metadata: &DocumentMetadata, upload: &Upload) -> Result<Vec<Node>> {
    // TODO: Upgrade to another version of mime_multifrom or replace because it uses hyper 0.10
    // headers and mime 0.2
    use hyper_old::header::{ContentDisposition, ContentType, DispositionParam, DispositionType, Headers};

    let mut nodes: Vec<Node> = Vec::with_capacity(2);

    let metadata_json = serde_json::to_string(metadata).map_err(|e| {
        e.context(ErrorKind::FailedToPrepareHttpRequest(
            "serializing doc-metadata json".to_string(),
        ))
    })?;
    debug!("Metadata: {:#?}", metadata_json);

    let mut h = Headers::new();
    h.set(ContentType(mime!(Application / Json)));
    h.set(ContentDisposition {
        disposition: DispositionType::Ext("form-data".to_string()),
        parameters: vec![DispositionParam::Ext("name".to_string(), "metadata".to_string())],
    });
    nodes.push(Node::Part(Part {
        headers: h,
        body: metadata_json.into_bytes(),
    }));

    let mut h = Headers::new();
    h.set(ContentType(upload.mime_type.clone()));
    h.set(ContentDisposition {
        disposition: DispositionType::Ext("form-data".to_string()),
        parameters: vec![
            DispositionParam::Ext("name".to_string(), "document".to_string()),
            DispositionParam::Ext("filename".to_string(), upload.filename.to_string()),
        ],
    });
    nodes.push(Node::File(FilePart::new(h, upload.path)));

    Ok(nodes)
}

// CenterDevice / Jersey does not accept special characters in boundary; thus, we build it
// ourselves.
fn generate_boundary(seed: &[u8]) -> String {
    let sha = ring::digest::digest(&ring::digest::SHA256, seed);
    let sha_str = hex::encode(sha.as_ref());
    format!("Boundary_{}", sha_str)
}
