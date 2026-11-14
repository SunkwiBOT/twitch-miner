use url::Url;

use crate::error::{Error, Result};

// ! The gzip body is unpacked while it streams, so the bound also holds for a
// small download that unpacks into a huge one.
pub async fn read_bounded(mut response: reqwest::Response, max: usize) -> Result<Vec<u8>> {
    if response
        .content_length()
        .is_some_and(|length| length > max as u64)
    {
        return Err(too_large());
    }

    let mut body = Vec::new();

    while let Some(chunk) = response.chunk().await? {
        if body.len() + chunk.len() > max {
            return Err(too_large());
        }

        body.extend_from_slice(&chunk);
    }

    Ok(body)
}

pub fn too_large() -> Error {
    Error::InvalidResponse("answer too large".into())
}

pub fn trusted_host(url: &Url, suffixes: &[&str]) -> bool {
    url.scheme() == "https"
        && url.username().is_empty()
        && url.password().is_none()
        && url.port().is_none()
        && url.host_str().is_some_and(|host| {
            suffixes.iter().any(|suffix| {
                host.strip_suffix(suffix)
                    .is_some_and(|start| start.is_empty() || start.ends_with('.'))
            })
        })
}
