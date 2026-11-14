use super::*;

#[derive(rust_embed::Embed)]
#[folder = "$CARGO_MANIFEST_DIR/../ui/dist/"]
struct Assets;

static UI_BUILD: LazyLock<Option<String>> = LazyLock::new(|| {
    Assets::get("index.html").map(|file| hex::encode(&file.metadata.sha256_hash()[..8]))
});

pub fn ui_build() -> Option<String> {
    UI_BUILD.clone()
}

pub(super) async fn static_file(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');

    if path.starts_with("api/") {
        return Failure::from(ErrorCode::NotFound).into_response();
    }

    let path = if path.is_empty() { "index.html" } else { path };

    let (path, content) = match Assets::get(path) {
        Some(content) => (path, content),
        None => match Assets::get("index.html") {
            Some(content) => ("index.html", content),
            None => return Failure::from(ErrorCode::NotFound).into_response(),
        },
    };

    let mime = mime_guess::from_path(path).first_or_octet_stream();
    let cache = if path.starts_with("assets/") {
        "public, max-age=31536000, immutable"
    } else {
        "no-cache"
    };

    (
        [
            (header::CONTENT_TYPE, mime.as_ref().to_owned()),
            (header::CACHE_CONTROL, cache.to_owned()),
        ],
        content.data.into_owned(),
    )
        .into_response()
}
