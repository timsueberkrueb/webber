use std::error::Error;
use std::fmt;

use url::Url;

mod net;
mod parser;
mod resolver;

use self::net::{download_manifest, ManifestContent};
pub use self::parser::*;

use crate::fetchable::*;
use crate::resolvable::*;

impl Fetchable for Manifest<Unresolved> {
    type Out<R: ResolveType> = Manifest<R>;
    type Error = FetchManifestError;

    fn fetch(url: &Url) -> Result<Manifest<Unresolved>, FetchManifestError> {
        let ManifestContent(s) =
            download_manifest(url).map_err(FetchManifestError::NetworkError)?;

        Manifest::parse(&s)
            .map_err(|e| e.to_string())
            .map_err(FetchManifestError::ParseError)
    }
}

#[derive(Debug)]
pub enum FetchManifestError {
    NetworkError(String),
    ParseError(String),
}

impl fmt::Display for FetchManifestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FetchManifestError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            FetchManifestError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl Error for FetchManifestError {}
