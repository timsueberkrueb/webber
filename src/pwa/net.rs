use url::Url;

use reqwest::blocking as reqwest;

pub(super) struct ManifestContent(pub String);

pub(super) fn download_manifest(url: &Url) -> Result<ManifestContent, String> {
    let body = reqwest::get(url.as_ref())
        .map_err(|err| err.to_string())?
        .text()
        .map_err(|err| err.to_string())?;

    Ok(ManifestContent(body))
}
