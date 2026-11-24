use reqwest::header::{HeaderValue, AUTHORIZATION};
use reqwest::StatusCode;
use serde_json::{json, Value};
use url::Url;

use super::{category_url, cdn, label_for, picture, retry_after_header, Answer, Notice, Shared};
use crate::config::WebhookSettings;
use crate::error::{Error, Result};
use crate::events::MinerEvent;
use crate::requests::RequestKind;

// ! The user's own address can name them: the requests list shows no target.
const TARGET: &str = "";

pub(super) async fn send(
    shared: &Shared,
    settings: &WebhookSettings,
    notice: &Notice,
) -> Result<Answer> {
    let url = target(&settings.url)?;
    let authorization = authorization(&settings.authorization)?;
    let event = &notice.event;
    let mut request =
        shared
            .client
            .post(url)
            .json(&body(event, notice.lead.as_deref(), notice.language));

    if let Some(authorization) = authorization {
        request = request.header(AUTHORIZATION, authorization);
    }

    let response = shared
        .requests
        .start(RequestKind::Webhook, TARGET)
        .account(Some(event.account_id.as_str()).filter(|id| !id.is_empty()))
        .send(request)
        .await?;
    let status = response.status();
    let retry_after = if status == StatusCode::TOO_MANY_REQUESTS {
        retry_after_header(&response)
    } else {
        None
    };

    Ok(Answer {
        status,
        retry_after,
        pause: None,
    })
}

pub(super) fn authorization(value: &str) -> Result<Option<HeaderValue>> {
    let value = value.trim();

    if value.is_empty() {
        return Ok(None);
    }

    let mut header = HeaderValue::from_str(value).map_err(|_| {
        Error::Notification("webhook Authorization is not a valid header value".into())
    })?;

    header.set_sensitive(true);

    Ok(Some(header))
}

pub(super) fn target(address: &str) -> Result<Url> {
    Url::parse(address.trim())
        .ok()
        .filter(|url| matches!(url.scheme(), "https" | "http") && url.has_host())
        .ok_or_else(|| Error::Notification("webhook URL is not a valid http(s) address".into()))
}

fn body(event: &MinerEvent, lead: Option<&str>, language: &str) -> Value {
    json!({
        "app": {
            "name": crate::APP_NAME,
            "version": crate::VERSION,
            "url": crate::REPOSITORY_URL,
        },
        "event": event,
        "label": label_for(language, event.kind),
        "lead": lead,
        "context": event.context,
        "image_url": cdn(picture(&event.context)),
        "category_url": category_url(&event.context),
    })
}
