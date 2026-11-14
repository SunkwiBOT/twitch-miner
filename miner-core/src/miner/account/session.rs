use super::*;

impl<T: TwitchApi> AccountMiner<T> {
    // ! Twitch, a proxy or Cloudflare can refuse a request while the token is
    // still good: only the validation endpoint decides that a sign-in is needed.
    pub(super) async fn session_refused(&mut self) {
        if self.status == AccountStatus::LoginRequired {
            return;
        }

        let checked_lately = self
            .session_valid_at
            .is_some_and(|at| at.elapsed() < REFUSAL_RECHECK_GAP);

        let checked = if checked_lately {
            Ok(())
        } else {
            self.validate_session().await
        };

        match checked {
            Ok(()) => {
                let refused = Error::Status(StatusCode::UNAUTHORIZED);

                if self.reported.contains(&refused.code()) {
                    tracing::debug!(account = %self.login, "{refused}; the session is valid");
                } else {
                    tracing::info!(account = %self.login, "{refused}; the session is valid");
                }

                self.set_problem(refused.code());
            }
            Err(Error::Unauthorized) => self.require_login().await,
            Err(other) => self.report_problem(other),
        }
    }

    pub(super) async fn require_login(&mut self) {
        self.set_status(
            AccountStatus::LoginRequired,
            Some(ErrorCode::SessionExpired),
        );
        self.login_checked_at = Some(tokio::time::Instant::now());

        let event = self.emit(
            EventKind::SessionExpired,
            "Twitch session expired, sign in again",
        );

        self.publish(event);
        self.stop_all_slots("sign-in required").await;
        self.close_sockets().await;
    }

    pub(super) async fn close_sockets(&mut self) {
        if let Some(mut pubsub) = self.pubsub.take() {
            pubsub.close().await;
        }

        if let Some(mut chat) = self.chat.take() {
            chat.close().await;
        }
    }

    pub(super) async fn session_back(&mut self) -> bool {
        if self
            .login_checked_at
            .is_some_and(|at| at.elapsed() < VALIDATE_INTERVAL)
        {
            return false;
        }

        self.login_checked_at = Some(tokio::time::Instant::now());

        if let Err(error) = self.validate_session().await {
            tracing::debug!(account = %self.login, %error, "the session is still refused");

            return false;
        }

        self.set_status(AccountStatus::Starting, None);

        let event = self.emit(EventKind::Info, "Twitch session valid again");

        self.publish(event);

        true
    }

    pub(super) async fn ensure_session(&mut self) -> Result<()> {
        if self
            .session_valid_at
            .is_some_and(|at| at.elapsed() < VALIDATE_INTERVAL)
        {
            return Ok(());
        }

        self.validate_session().await
    }

    pub(super) async fn validate_session(&mut self) -> Result<()> {
        match self.client.validate().await? {
            Some(validation) if validation.user_id == self.id => {
                self.session_valid_at = Some(tokio::time::Instant::now());

                Ok(())
            }
            Some(_) => Err(Error::Unauthorized),
            None => self.refresh_session().await,
        }
    }

    pub(super) async fn refresh_session(&mut self) -> Result<()> {
        let Some(refresh) = self.credential.tokens.refresh_token.clone() else {
            return Err(Error::Unauthorized);
        };
        let mut tokens = self.client.refresh_tokens(&refresh).await?;

        if tokens.refresh_token.is_none() {
            tokens.refresh_token = Some(refresh);
        }

        self.credential.tokens = tokens;
        self.context
            .store
            .save_credential(&self.id, &self.credential, &self.context.vault)?;
        self.session_valid_at = Some(tokio::time::Instant::now());

        let event = self.emit(EventKind::Info, "Twitch session renewed");

        self.publish(event);

        self.close_sockets().await;

        Ok(())
    }
}
