use super::*;

pub(super) const WRONG_PASSWORD_DELAY: Duration = Duration::from_millis(500);

static PLAIN_HTTP_WARNING: Once = Once::new();
static UNKNOWN_VISITOR_WARNING: Once = Once::new();
static DIRECT_VISITOR_WARNING: Once = Once::new();
static CROSS_SITE_NOTICE: Once = Once::new();
static UNKNOWN_PROXY_WARNING: Once = Once::new();

#[derive(Clone, Copy)]
pub(super) struct ClientAddress {
    visitor: IpAddr,
    connection: IpAddr,
}

pub(super) fn cookie_session(headers: &HeaderMap, name: &str) -> Option<String> {
    let prefix = format!("{name}=");
    headers
        .get_all(header::COOKIE)
        .iter()
        .filter_map(|value| value.to_str().ok())
        .flat_map(|value| value.split(';'))
        .map(str::trim)
        .find_map(|pair| pair.strip_prefix(&prefix).map(str::to_owned))
}

pub(super) fn bearer(headers: &HeaderMap) -> Option<&str> {
    headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
}

// ! Hashed first: two values of one length, so the time taken tells nothing,
// not even the password's length.
pub(super) fn password_matches(expected: &str, given: &str) -> bool {
    let expected: [u8; 32] = Sha256::digest(expected.as_bytes()).into();
    let given: [u8; 32] = Sha256::digest(given.as_bytes()).into();

    expected.ct_eq(&given).into()
}

enum Verdict {
    Allowed(Option<SessionLease>),
    Refused,
    LockedOut(Duration),
}

async fn authorize(state: &AppState, headers: &HeaderMap, client: ClientAddress) -> Verdict {
    // ! A bearer password is a sign-in try like the form's, or scripts would
    // be a way around the lockout.
    if let Some(given) = bearer(headers) {
        let matches = password_matches(state.access.password(), given);

        return match state
            .attempts
            .attempt(client.visitor, client.connection, matches)
            .await
        {
            SignIn::Accepted => Verdict::Allowed(None),
            SignIn::Refused => {
                tokio::time::sleep(WRONG_PASSWORD_DELAY).await;

                Verdict::Refused
            }
            SignIn::LockedOut { retry_after } => Verdict::LockedOut(retry_after),
        };
    }

    let Some(session) = cookie_session(headers, state.access.cookie_name()) else {
        return Verdict::Refused;
    };

    let idle_limit = state.manager.global_settings().await.auto_lock.idle_limit();

    match state.sessions.find(&session, idle_limit).await {
        Some(lease) => Verdict::Allowed(Some(lease)),
        None => Verdict::Refused,
    }
}

fn locked_out(retry_after: Duration) -> Response {
    let seconds = retry_after.as_millis().div_ceil(1000);

    (
        StatusCode::TOO_MANY_REQUESTS,
        [(header::RETRY_AFTER, seconds.to_string())],
        Json(ApiError::new(ErrorCode::TooManyAttempts)),
    )
        .into_response()
}

pub(super) async fn admit(
    State(state): State<Shared>,
    ConnectInfo(socket): ConnectInfo<SocketAddr>,
    mut request: Request,
    next: Next,
) -> Response {
    if state.access.behind_unknown_proxy(request.headers()) {
        UNKNOWN_PROXY_WARNING.call_once(|| {
            tracing::warn!(
                "a request came through a proxy (X-Forwarded-*) but MINER_PUBLIC_URL is not set: sign-in tries are counted on the proxy's address, so one visitor's wrong passwords lock everyone out; set MINER_PUBLIC_URL to the dashboard's https:// address"
            );
        });
    }

    let connection = socket.ip();
    match state.access.admit(request.headers(), connection) {
        Ok(_)
            if !state
                .access
                .allows_write(request.method(), request.headers()) =>
        {
            CROSS_SITE_NOTICE.call_once(|| {
                tracing::info!("refused a write sent by a page of another site or port");
            });

            Failure::from(ErrorCode::CrossSiteRequest).into_response()
        }
        Ok(visitor) => {
            request.extensions_mut().insert(ClientAddress {
                visitor,
                connection,
            });

            next.run(request).await
        }
        Err(code) => {
            warn_once(code);

            Failure::from(code).into_response()
        }
    }
}

fn warn_once(refusal: ErrorCode) {
    match refusal {
        ErrorCode::HttpsRequired => PLAIN_HTTP_WARNING.call_once(|| {
            tracing::warn!(
                "refused a request that did not arrive over HTTPS: open the dashboard at MINER_PUBLIC_URL, through a proxy that sends X-Forwarded-Proto"
            );
        }),
        ErrorCode::ProxyNotLocal => DIRECT_VISITOR_WARNING.call_once(|| {
            tracing::warn!(
                "refused a request from a public address that reached the port directly: with MINER_PUBLIC_URL, only the HTTPS proxy on this machine or its local network may connect"
            );
        }),
        _ => UNKNOWN_VISITOR_WARNING.call_once(|| {
            tracing::warn!(
                "refused a request without the visitor's address: the HTTPS proxy must send X-Forwarded-For (and MINER_TRUSTED_HOPS must not exceed the proxies that add to it)"
            );
        }),
    }
}

// ! Runs before any body is read, so an anonymous request never gets parsed.
pub(super) async fn require_session(
    State(state): State<Shared>,
    Extension(client): Extension<ClientAddress>,
    mut request: Request,
    next: Next,
) -> Response {
    match authorize(&state, request.headers(), client).await {
        Verdict::Allowed(lease) => {
            if let Some(lease) = lease {
                request.extensions_mut().insert(lease);
            }

            next.run(request).await
        }
        Verdict::Refused => Failure::from(ErrorCode::Unauthorized).into_response(),
        Verdict::LockedOut(retry_after) => locked_out(retry_after),
    }
}

pub(super) async fn auth_status(
    State(state): State<Shared>,
    Extension(client): Extension<ClientAddress>,
    headers: HeaderMap,
) -> Json<AuthResponse> {
    let verdict = authorize(&state, &headers, client).await;

    Json(AuthResponse {
        auth: AuthStatus {
            authorized: matches!(verdict, Verdict::Allowed(_)),
            public: state.access.is_public(),
        },
    })
}

pub(super) async fn login(
    State(state): State<Shared>,
    Extension(client): Extension<ClientAddress>,
    headers: HeaderMap,
    Body(body): Body<DashboardLoginRequest>,
) -> Result<Response, Failure> {
    let matches = password_matches(state.access.password(), &body.password);

    match state
        .attempts
        .attempt(client.visitor, client.connection, matches)
        .await
    {
        SignIn::Accepted => {}
        SignIn::Refused => {
            tokio::time::sleep(WRONG_PASSWORD_DELAY).await;

            return Err(ErrorCode::WrongPassword.into());
        }
        SignIn::LockedOut { retry_after } => return Ok(locked_out(retry_after)),
    }

    if let Some(previous) = cookie_session(&headers, state.access.cookie_name()) {
        state.sessions.revoke(&previous).await;
    }

    let session = state.sessions.create().await;
    let cookie = state.access.session_cookie(&session, SESSION_MAX_AGE);

    Ok((StatusCode::NO_CONTENT, [(header::SET_COOKIE, cookie)]).into_response())
}

pub(super) async fn logout(State(state): State<Shared>, headers: HeaderMap) -> Response {
    if let Some(session) = cookie_session(&headers, state.access.cookie_name()) {
        state.sessions.revoke(&session).await;
    }

    let cleared = state.access.session_cookie("", Duration::ZERO);

    (StatusCode::NO_CONTENT, [(header::SET_COOKIE, cleared)]).into_response()
}

pub(super) async fn activity(lease: Option<Extension<SessionLease>>) -> StatusCode {
    if let Some(Extension(lease)) = lease {
        lease.touch();
    }

    StatusCode::NO_CONTENT
}
