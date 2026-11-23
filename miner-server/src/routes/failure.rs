use super::*;

pub(super) fn status_of(code: ErrorCode) -> StatusCode {
    match code {
        ErrorCode::Unauthorized | ErrorCode::WrongPassword => StatusCode::UNAUTHORIZED,
        ErrorCode::WrongPassphrase
        | ErrorCode::HttpsRequired
        | ErrorCode::ForwardedForRequired
        | ErrorCode::ProxyNotLocal
        | ErrorCode::CrossSiteRequest
        | ErrorCode::UpdateSignatureRefused
        | ErrorCode::OpenUrlRefused => StatusCode::FORBIDDEN,
        ErrorCode::InvalidRequest => StatusCode::BAD_REQUEST,
        ErrorCode::NotFound
        | ErrorCode::UnknownAccount
        | ErrorCode::UnknownChannel
        | ErrorCode::UnknownReward => StatusCode::NOT_FOUND,
        ErrorCode::VaultLocked => StatusCode::LOCKED,
        ErrorCode::PassphraseRequired
        | ErrorCode::RewardNotRedeemable
        | ErrorCode::BurstRunning
        | ErrorCode::DeviceCodeExpired => StatusCode::CONFLICT,
        ErrorCode::InvalidProxy | ErrorCode::InvalidStreamerLogin | ErrorCode::InvalidToken => {
            StatusCode::UNPROCESSABLE_ENTITY
        }
        // ! Never 502 or 504: Cloudflare puts its own page in their place.
        ErrorCode::NetworkError
        | ErrorCode::TwitchUnavailable
        | ErrorCode::TwitchRejected
        | ErrorCode::InvalidResponse
        | ErrorCode::SessionExpired
        | ErrorCode::NotificationFailed
        | ErrorCode::BeaconBlocked => StatusCode::FAILED_DEPENDENCY,
        ErrorCode::CatalogRateLimited
        | ErrorCode::TooManyAttempts
        | ErrorCode::ActionRateLimited => StatusCode::TOO_MANY_REQUESTS,
        ErrorCode::NotSupported => StatusCode::NOT_IMPLEMENTED,
        ErrorCode::StorageError
        | ErrorCode::NewerData
        | ErrorCode::InvalidVault
        | ErrorCode::UpdateInstallFailed
        | ErrorCode::ServerUnreachable
        | ErrorCode::OpenUrlFailed
        | ErrorCode::UnknownError => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub struct Failure(ApiError);

impl From<ErrorCode> for Failure {
    fn from(code: ErrorCode) -> Self {
        Self(ApiError::new(code))
    }
}

impl From<miner_core::Error> for Failure {
    fn from(error: miner_core::Error) -> Self {
        Self(ApiError::from(error))
    }
}

impl IntoResponse for Failure {
    fn into_response(self) -> Response {
        (status_of(self.0.code), Json(self.0)).into_response()
    }
}

pub struct Body<T>(pub T);

impl<S, T> FromRequest<S> for Body<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Failure;

    async fn from_request(request: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<T>::from_request(request, state).await {
            Ok(Json(value)) => Ok(Self(value)),
            Err(rejection) => Err(invalid_request(&rejection)),
        }
    }
}

pub struct Params<T>(pub T);

impl<S, T> FromRequestParts<S> for Params<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Failure;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Query::<T>::from_request_parts(parts, state).await {
            Ok(axum::extract::Query(value)) => Ok(Self(value)),
            Err(rejection) => Err(invalid_query(&rejection)),
        }
    }
}

fn invalid_request(rejection: &JsonRejection) -> Failure {
    tracing::debug!("request body refused: {rejection}");
    ErrorCode::InvalidRequest.into()
}

fn invalid_query(rejection: &QueryRejection) -> Failure {
    tracing::debug!("query refused: {rejection}");
    ErrorCode::InvalidRequest.into()
}
