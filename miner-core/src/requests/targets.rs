use url::Url;

const SEGMENT_MAX_CHARS: usize = 32;
const MASKED_SEGMENT: &str = "…";

// ! Host and path only: playback and beacon addresses carry signatures in the query, a long segment is a token.
pub fn url_target(url: &Url) -> String {
    let path: Vec<&str> = url
        .path_segments()
        .into_iter()
        .flatten()
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            if segment.chars().count() > SEGMENT_MAX_CHARS {
                MASKED_SEGMENT
            } else {
                segment
            }
        })
        .collect();
    let host = url.host_str().unwrap_or_default();

    if path.is_empty() {
        return host.to_owned();
    }

    format!("{host}/{}", path.join("/"))
}

pub fn host_target(url: &Url) -> String {
    url.host_str().unwrap_or_default().to_owned()
}

pub fn address_target(address: &str) -> String {
    Url::parse(address)
        .map(|url| url_target(&url))
        .unwrap_or_default()
}
