use crate::auth::{OAuthConfig, TokenResponse};
use crate::error::{Result, WhoopError};
use crate::models::*;
use reqwest::{Client, Method, RequestBuilder, StatusCode};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

const BASE_URL: &str = "https://api.prod.whoop.com/developer";

pub struct WhoopClient {
    client: Client,
    auth: Auth,
}

enum Auth {
    AccessToken(String),
    OAuth {
        config: OAuthConfig,
        token: Arc<Mutex<TokenResponse>>,
    },
}

impl WhoopClient {
    /// Creates a client with a simple access token.
    /// Use this if you already have a token from somewhere else.
    pub fn new(access_token: String) -> Self {
        let client = Client::new();
        Self {
            client,
            auth: Auth::AccessToken(access_token),
        }
    }

    /// Creates a client that can manage OAuth tokens.
    /// Use when you've already done the OAuth dance.
    pub fn new_with_oauth(config: OAuthConfig, token: TokenResponse) -> Self {
        let client = Client::new();
        Self {
            client,
            auth: Auth::OAuth {
                config,
                token: Arc::new(Mutex::new(token)),
            },
        }
    }

    /// Creates a client directly from an authorization code.
    /// Handles the token exchange for you.
    pub async fn from_authorization_code(config: OAuthConfig, code: String) -> Result<Self> {
        let token = config.exchange_code(code).await?;
        Ok(Self::new_with_oauth(config, token))
    }

    fn get_access_token(&self) -> String {
        match &self.auth {
            Auth::AccessToken(token) => token.clone(),
            Auth::OAuth { token, .. } => {
                let token_lock = token.lock().unwrap();
                token_lock.access_token.clone()
            }
        }
    }

    /// Refreshes an expired OAuth token.
    /// Only works if you're using OAuth (does nothing for static tokens).
    pub async fn refresh_token(&mut self) -> Result<()> {
        match &self.auth {
            Auth::AccessToken(_) => Ok(()),
            Auth::OAuth { config, token } => {
                let refresh_token = {
                    let token_lock = token.lock().unwrap();
                    token_lock.refresh_token.clone().ok_or_else(|| {
                        WhoopError::AuthenticationError("No refresh token available".to_string())
                    })?
                };

                let new_token = config.refresh_token(refresh_token).await?;

                let mut token_lock = token.lock().unwrap();
                *token_lock = new_token;
                Ok(())
            }
        }
    }

    fn request(&self, method: Method, path: &str) -> RequestBuilder {
        let url = format!("{}{}", BASE_URL, path);
        self.client
            .request(method, url)
            .bearer_auth(self.get_access_token())
    }

    async fn execute<T: DeserializeOwned>(&self, request: RequestBuilder) -> Result<T> {
        let response = request.send().await?;
        let status = response.status();

        if status.is_success() {
            let data = response.json::<T>().await?;
            Ok(data)
        } else {
            let message = response.text().await.ok();
            Err(WhoopError::from_status(status, message))
        }
    }

    async fn execute_no_content(&self, request: RequestBuilder) -> Result<()> {
        let response = request.send().await?;
        let status = response.status();

        if status == StatusCode::NO_CONTENT {
            Ok(())
        } else {
            let message = response.text().await.ok();
            Err(WhoopError::from_status(status, message))
        }
    }

    // Cycle endpoints

    pub async fn get_cycle_by_id(&self, cycle_id: i64) -> Result<Cycle> {
        let path = format!("/v2/cycle/{}", cycle_id);
        let request = self.request(Method::GET, &path);
        self.execute(request).await
    }

    pub async fn get_cycle_collection(
        &self,
        params: Option<CycleQueryParams>,
    ) -> Result<PaginatedCycleResponse> {
        let mut request = self.request(Method::GET, "/v2/cycle");

        if let Some(p) = params {
            request = request.query(&p);
        }

        self.execute(request).await
    }

    pub async fn get_sleep_for_cycle(&self, cycle_id: i64) -> Result<Sleep> {
        let path = format!("/v2/cycle/{}/sleep", cycle_id);
        let request = self.request(Method::GET, &path);
        self.execute(request).await
    }

    pub async fn get_recovery_for_cycle(&self, cycle_id: i64) -> Result<Recovery> {
        let path = format!("/v2/cycle/{}/recovery", cycle_id);
        let request = self.request(Method::GET, &path);
        self.execute(request).await
    }

    // Recovery endpoints

    pub async fn get_recovery_collection(
        &self,
        params: Option<RecoveryQueryParams>,
    ) -> Result<RecoveryCollection> {
        let mut request = self.request(Method::GET, "/v2/recovery");

        if let Some(p) = params {
            request = request.query(&p);
        }

        self.execute(request).await
    }

    // Sleep endpoints

    pub async fn get_sleep_by_id(&self, sleep_id: Uuid) -> Result<Sleep> {
        let path = format!("/v2/activity/sleep/{}", sleep_id);
        let request = self.request(Method::GET, &path);
        self.execute(request).await
    }

    pub async fn get_sleep_collection(
        &self,
        params: Option<SleepQueryParams>,
    ) -> Result<PaginatedSleepResponse> {
        let mut request = self.request(Method::GET, "/v2/activity/sleep");

        if let Some(p) = params {
            request = request.query(&p);
        }

        self.execute(request).await
    }

    // User endpoints

    pub async fn get_body_measurement(&self) -> Result<UserBodyMeasurement> {
        let request = self.request(Method::GET, "/v2/user/measurement/body");
        self.execute(request).await
    }

    pub async fn get_profile_basic(&self) -> Result<UserBasicProfile> {
        let request = self.request(Method::GET, "/v2/user/profile/basic");
        self.execute(request).await
    }

    pub async fn revoke_oauth_access(&self) -> Result<()> {
        let request = self.request(Method::DELETE, "/v2/user/access");
        self.execute_no_content(request).await
    }

    // Workout endpoints

    pub async fn get_workout_by_id(&self, workout_id: Uuid) -> Result<WorkoutV2> {
        let path = format!("/v2/activity/workout/{}", workout_id);
        let request = self.request(Method::GET, &path);
        self.execute(request).await
    }

    pub async fn get_workout_collection(
        &self,
        params: Option<WorkoutQueryParams>,
    ) -> Result<WorkoutCollection> {
        let mut request = self.request(Method::GET, "/v2/activity/workout");

        if let Some(p) = params {
            request = request.query(&p);
        }

        self.execute(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = WhoopClient::new("test_token".to_string());
        assert_eq!(client.get_access_token(), "test_token");
    }
}
