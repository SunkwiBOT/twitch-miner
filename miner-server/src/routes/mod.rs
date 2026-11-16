mod assets;
mod auth;
mod failure;
mod push;

use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, LazyLock, Once};
use std::time::Duration;

use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::extract::ws::{close_code, CloseFrame, Message, Utf8Bytes, WebSocket, WebSocketUpgrade};
use axum::extract::{ConnectInfo, FromRequest, FromRequestParts, Request, State};
use axum::http::request::Parts;
use axum::http::{header, HeaderMap, HeaderName, HeaderValue, StatusCode, Uri};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use miner_core::api::{
    ApiRoute, AuthResponse, AuthStatus, BadgesResponse, CatalogResponse, ChannelBannersRequest,
    ChannelBannersResponse, ChannelQuery, ChannelResponse, ChannelsLookupRequest,
    ChannelsLookupResponse, Command, DashboardLoginRequest, FollowsQuery, FollowsResponse,
    HistoryQuery, HistoryResponse, LoginFlowResponse, LogsQuery, LogsReportResponse, LogsResponse,
    NotificationTestResponse, ProxyTestRequest, ProxyTestResponse, RedeemRequest, RedeemResponse,
    RefreshResponse, RequestsQuery, RequestsResponse, SearchQuery, SearchResponse, ServerMessage,
    SnapshotResponse, SpamBalanceResponse, SpamChannelQuery, SpamChannelResponse, UpdateCheck,
    UpdateCheckResponse,
};
use miner_core::error::{ApiError, ErrorCode};
use miner_core::Manager;
use serde::de::DeserializeOwned;
use sha2::{Digest, Sha256};
use subtle::ConstantTimeEq;
use tower_http::set_header::SetResponseHeaderLayer;

use crate::access::Access;
use crate::attempts::{SignIn, SignInAttempts};
use crate::feed::SnapshotFeed;
use crate::sessions::{SessionLease, Sessions, SESSION_MAX_AGE};
use crate::updates::Releases;

pub use assets::ui_build;
use assets::*;
use auth::*;
use failure::*;
use push::*;

const BODY_LIMIT_BYTES: usize = 256 * 1024;
// ! The desktop window's policy (`tauri.conf.json`) without its `ipc:` sources, plus the push channel's websocket and the framing and form rules a window never needs: the two change together.
const CONTENT_SECURITY_POLICY: &str = "default-src 'self'; img-src 'self' data: https://static-cdn.jtvnw.net https://*.jtvnw.net; style-src 'self' 'unsafe-inline'; font-src 'self'; connect-src 'self' {connect}; object-src 'none'; base-uri 'self'; form-action 'self'; frame-ancestors 'none'";
const SECURE_WEBSOCKET_SCHEME: &str = "wss";
const STRICT_TRANSPORT_SECURITY: &str = "max-age=31536000";
const API_CACHE_CONTROL: &str = "no-store";
const PERMISSIONS_POLICY: &str = "camera=(), microphone=(), geolocation=(), payment=(), usb=()";
const PERMISSIONS_POLICY_HEADER: HeaderName = HeaderName::from_static("permissions-policy");

pub struct AppState {
    pub manager: Arc<Manager>,
    pub snapshots: Arc<SnapshotFeed>,
    pub access: Access,
    pub sessions: Sessions,
    pub attempts: SignInAttempts,
    pub releases: Releases,
}

pub type Shared = Arc<AppState>;

// ! A local dashboard has no fixed address: its websocket is the one of the address the page was opened at.
fn content_security_policy(access: &Access, host: Option<&str>) -> HeaderValue {
    let connect = match access {
        Access::Local { .. } => host
            .filter(|host| {
                !host.is_empty()
                    && host
                        .bytes()
                        .all(|b| b.is_ascii_alphanumeric() || b".-:[]".contains(&b))
            })
            .map(|host| format!("ws://{host} wss://{host}"))
            .unwrap_or_default(),
        Access::Public { url, .. } => {
            let mut websocket = url.clone();
            let _ = websocket.set_scheme(SECURE_WEBSOCKET_SCHEME);

            websocket.origin().ascii_serialization()
        }
    };

    HeaderValue::from_str(&CONTENT_SECURITY_POLICY.replace("{connect}", &connect))
        .expect("the public address and a checked host are valid header values")
}

async fn security_policy(State(state): State<Shared>, request: Request, next: Next) -> Response {
    let host = request
        .headers()
        .get(header::HOST)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    let mut response = next.run(request).await;

    response.headers_mut().insert(
        header::CONTENT_SECURITY_POLICY,
        content_security_policy(&state.access, host.as_deref()),
    );

    response
}

pub fn router(state: Shared) -> Router {
    let public = state.access.is_public();
    let protected = Router::new()
        .route(ApiRoute::Activity.path(), post(activity))
        .route(ApiRoute::Snapshot.path(), get(snapshot))
        .route(ApiRoute::Command.path(), post(command))
        .route(ApiRoute::Catalog.path(), get(catalog))
        .route(ApiRoute::CatalogRefresh.path(), post(refresh_catalog))
        .route(ApiRoute::BadgesRefresh.path(), post(refresh_badges))
        .route(ApiRoute::UpdatesCheck.path(), post(check_for_update))
        .route(ApiRoute::LoginFlow.path(), post(start_login))
        .route(ApiRoute::Channel.path(), get(lookup_channel))
        .route(ApiRoute::ChannelsLookup.path(), post(lookup_channels))
        .route(ApiRoute::Search.path(), get(search))
        .route(ApiRoute::ChannelBanners.path(), post(channel_banners))
        .route(ApiRoute::Follows.path(), get(follows))
        .route(ApiRoute::NotificationsTest.path(), post(test_notifications))
        .route(ApiRoute::ProxyTest.path(), post(test_proxy))
        .route(ApiRoute::History.path(), get(history))
        .route(ApiRoute::Badges.path(), get(badges))
        .route(ApiRoute::Logs.path(), get(logs))
        .route(ApiRoute::LogsReport.path(), get(logs_report))
        .route(ApiRoute::Requests.path(), get(requests))
        .route(ApiRoute::RequestsReport.path(), get(requests_report))
        .route(ApiRoute::Events.path(), get(events))
        .route(ApiRoute::SpamChannel.path(), get(spam_channel))
        .route(ApiRoute::SpamBalance.path(), get(spam_balance))
        .route(ApiRoute::SpamRedeem.path(), post(redeem))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            require_session,
        ));

    let router = Router::new()
        .route(
            ApiRoute::Auth.path(),
            get(auth_status).post(login).delete(logout),
        )
        .merge(protected)
        .route_layer(SetResponseHeaderLayer::overriding(
            header::CACHE_CONTROL,
            HeaderValue::from_static(API_CACHE_CONTROL),
        ))
        .fallback(static_file)
        .layer(middleware::from_fn_with_state(state.clone(), admit))
        // ! Added after the admission check, which only wraps the routes above:
        // Docker's health check calls it over plain HTTP from inside the container.
        .route(
            ApiRoute::Health.path(),
            get(|| async { StatusCode::NO_CONTENT }),
        )
        .layer(axum::extract::DefaultBodyLimit::max(BODY_LIMIT_BYTES))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            header::REFERRER_POLICY,
            HeaderValue::from_static("no-referrer"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("x-robots-tag"),
            HeaderValue::from_static("noindex, nofollow"),
        ))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            security_policy,
        ))
        .layer(SetResponseHeaderLayer::overriding(
            PERMISSIONS_POLICY_HEADER,
            HeaderValue::from_static(PERMISSIONS_POLICY),
        ))
        .layer(tower_http::compression::CompressionLayer::new())
        .with_state(state);

    if public {
        return router.layer(SetResponseHeaderLayer::overriding(
            header::STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_static(STRICT_TRANSPORT_SECURITY),
        ));
    }

    router
}

async fn snapshot(State(state): State<Shared>) -> Result<Json<SnapshotResponse>, Failure> {
    let mut snapshot = state.manager.snapshot().await?;

    snapshot.build = ui_build();

    Ok(Json(SnapshotResponse { snapshot }))
}

async fn command(
    State(state): State<Shared>,
    Body(command): Body<Command>,
) -> Result<StatusCode, Failure> {
    state.manager.execute(command).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn start_login(State(state): State<Shared>) -> Result<Json<LoginFlowResponse>, Failure> {
    Ok(Json(LoginFlowResponse {
        login: state.manager.start_login().await?,
    }))
}

async fn lookup_channel(
    State(state): State<Shared>,
    Params(query): Params<ChannelQuery>,
) -> Result<Json<ChannelResponse>, Failure> {
    Ok(Json(ChannelResponse {
        channel: state.manager.lookup_channel(&query.login).await?,
    }))
}

async fn channel_banners(
    State(state): State<Shared>,
    Body(request): Body<ChannelBannersRequest>,
) -> Result<Json<ChannelBannersResponse>, Failure> {
    Ok(Json(state.manager.channel_banners(&request.logins).await?))
}

async fn search(
    State(state): State<Shared>,
    Params(query): Params<SearchQuery>,
) -> Result<Json<SearchResponse>, Failure> {
    Ok(Json(state.manager.search(&query.query).await?))
}

async fn lookup_channels(
    State(state): State<Shared>,
    Body(request): Body<ChannelsLookupRequest>,
) -> Result<Json<ChannelsLookupResponse>, Failure> {
    Ok(Json(state.manager.lookup_channels(&request.logins).await?))
}

async fn follows(
    State(state): State<Shared>,
    Params(query): Params<FollowsQuery>,
) -> Result<Json<FollowsResponse>, Failure> {
    Ok(Json(FollowsResponse {
        channels: state.manager.followed_channels(&query.account_id).await?,
    }))
}

async fn spam_channel(
    State(state): State<Shared>,
    Params(query): Params<SpamChannelQuery>,
) -> Result<Json<SpamChannelResponse>, Failure> {
    Ok(Json(SpamChannelResponse {
        channel: state
            .manager
            .spam_channel(&query.account_id, &query.login)
            .await?,
    }))
}

async fn spam_balance(
    State(state): State<Shared>,
    Params(query): Params<SpamChannelQuery>,
) -> Result<Json<SpamBalanceResponse>, Failure> {
    Ok(Json(SpamBalanceResponse {
        balance: state
            .manager
            .spam_balance(&query.account_id, &query.login)
            .await?,
    }))
}

// ! Answered once the burst is accepted: its result comes over the push
// channel, as a burst can outlast a proxy's read timeout.
async fn redeem(
    State(state): State<Shared>,
    Body(request): Body<RedeemRequest>,
) -> Result<(StatusCode, Json<RedeemResponse>), Failure> {
    Ok((
        StatusCode::ACCEPTED,
        Json(RedeemResponse {
            burst_id: state.manager.redeem(request).await?,
        }),
    ))
}

async fn test_notifications(State(state): State<Shared>) -> Json<NotificationTestResponse> {
    Json(NotificationTestResponse {
        targets: state.manager.test_notification_targets().await,
    })
}

async fn test_proxy(
    State(state): State<Shared>,
    Body(request): Body<ProxyTestRequest>,
) -> Result<Json<ProxyTestResponse>, Failure> {
    Ok(Json(state.manager.test_proxy(&request.url).await?))
}

async fn history(
    State(state): State<Shared>,
    Params(query): Params<HistoryQuery>,
) -> Result<Json<HistoryResponse>, Failure> {
    Ok(Json(state.manager.history_page(&query)?))
}

async fn refresh_catalog(State(state): State<Shared>) -> Result<Json<RefreshResponse>, Failure> {
    Ok(Json(RefreshResponse {
        refresh: state.manager.refresh_catalog().await?,
    }))
}

async fn refresh_badges(State(state): State<Shared>) -> Result<Json<RefreshResponse>, Failure> {
    Ok(Json(RefreshResponse {
        refresh: state.manager.refresh_badges().await?,
    }))
}

async fn check_for_update(
    State(state): State<Shared>,
) -> Result<Json<UpdateCheckResponse>, Failure> {
    let available = state.releases.check(&state.manager).await?;

    Ok(Json(UpdateCheckResponse {
        check: UpdateCheck { available },
    }))
}

async fn badges(State(state): State<Shared>) -> Json<BadgesResponse> {
    Json(state.manager.badge_list().await)
}

async fn catalog(State(state): State<Shared>) -> Json<CatalogResponse> {
    Json(state.manager.catalog_list().await)
}

async fn logs(State(state): State<Shared>, Params(query): Params<LogsQuery>) -> Json<LogsResponse> {
    Json(state.manager.logs_page(&query))
}

async fn logs_report(
    State(state): State<Shared>,
    Params(query): Params<LogsQuery>,
) -> Json<LogsReportResponse> {
    Json(state.manager.logs_report(&query))
}

async fn requests(
    State(state): State<Shared>,
    Params(query): Params<RequestsQuery>,
) -> Result<Json<RequestsResponse>, Failure> {
    Ok(Json(state.manager.requests_page(query).await?))
}

async fn requests_report(
    State(state): State<Shared>,
    Params(query): Params<RequestsQuery>,
) -> Result<Json<LogsReportResponse>, Failure> {
    Ok(Json(state.manager.requests_report(query).await?))
}
