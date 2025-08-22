use crate::error::{Result, WhoopError};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub const AUTH_URL: &str = "https://api.prod.whoop.com/oauth/oauth2/auth";
pub const TOKEN_URL: &str = "https://api.prod.whoop.com/oauth/oauth2/token";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: Option<i64>,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenRequest {
    pub grant_type: String,
    pub code: Option<String>,
    pub redirect_uri: Option<String>,
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Scope {
    ReadRecovery,
    ReadCycles,
    ReadWorkout,
    ReadSleep,
    ReadProfile,
    ReadBodyMeasurement,
}

impl Scope {
    /// Converts the scope to its string representation for the API.
    pub fn as_str(&self) -> &str {
        match self {
            Scope::ReadRecovery => "read:recovery",
            Scope::ReadCycles => "read:cycles",
            Scope::ReadWorkout => "read:workout",
            Scope::ReadSleep => "read:sleep",
            Scope::ReadProfile => "read:profile",
            Scope::ReadBodyMeasurement => "read:body_measurement",
        }
    }

    /// Parses a scope from its string representation.
    /// Returns None if the string doesn't match any known scope.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "read:recovery" => Some(Scope::ReadRecovery),
            "read:cycles" => Some(Scope::ReadCycles),
            "read:workout" => Some(Scope::ReadWorkout),
            "read:sleep" => Some(Scope::ReadSleep),
            "read:profile" => Some(Scope::ReadProfile),
            "read:body_measurement" => Some(Scope::ReadBodyMeasurement),
            _ => None,
        }
    }
}

pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scopes: HashSet<Scope>,
}

impl OAuthConfig {
    /// Creates a new OAuth config with the required credentials.
    /// Start here, then add scopes with `with_scope()` or `with_all_scopes()`.
    pub fn new(client_id: String, client_secret: String, redirect_uri: String) -> Self {
        Self {
            client_id,
            client_secret,
            redirect_uri,
            scopes: HashSet::new(),
        }
    }

    /// Adds a single scope to the OAuth request.
    /// Chain multiple calls to add more scopes.
    pub fn with_scope(mut self, scope: Scope) -> Self {
        self.scopes.insert(scope);
        self
    }

    /// Adds all available scopes for full access.
    /// Useful for development or when you need complete access.
    pub fn with_all_scopes(mut self) -> Self {
        self.scopes.insert(Scope::ReadRecovery);
        self.scopes.insert(Scope::ReadCycles);
        self.scopes.insert(Scope::ReadWorkout);
        self.scopes.insert(Scope::ReadSleep);
        self.scopes.insert(Scope::ReadProfile);
        self.scopes.insert(Scope::ReadBodyMeasurement);
        self
    }

    /// Builds the URL where users authorize your app.
    /// Redirect users here to start the OAuth flow.
    pub fn get_authorization_url(&self) -> String {
        let scopes_str = self
            .scopes
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(" ");

        format!(
            "{}?response_type=code&client_id={}&redirect_uri={}&scope={}",
            AUTH_URL,
            urlencoding::encode(&self.client_id),
            urlencoding::encode(&self.redirect_uri),
            urlencoding::encode(&scopes_str)
        )
    }

    /// Exchanges an authorization code for access and refresh tokens.
    /// Call this after the user authorizes and you get the code from the callback.
    pub async fn exchange_code(&self, code: String) -> Result<TokenResponse> {
        let client = reqwest::Client::new();

        let params = TokenRequest {
            grant_type: "authorization_code".to_string(),
            code: Some(code),
            redirect_uri: Some(self.redirect_uri.clone()),
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            refresh_token: None,
        };

        let response = client.post(TOKEN_URL).form(&params).send().await?;

        if response.status().is_success() {
            Ok(response.json::<TokenResponse>().await?)
        } else {
            let msg = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(WhoopError::AuthenticationError(msg))
        }
    }

    /// Gets a new access token using a refresh token.
    /// Use when the access token expires (usually after an hour).
    pub async fn refresh_token(&self, refresh_token: String) -> Result<TokenResponse> {
        let client = reqwest::Client::new();

        let params = TokenRequest {
            grant_type: "refresh_token".to_string(),
            code: None,
            redirect_uri: None,
            client_id: self.client_id.clone(),
            client_secret: self.client_secret.clone(),
            refresh_token: Some(refresh_token),
        };

        let response = client.post(TOKEN_URL).form(&params).send().await?;

        if response.status().is_success() {
            Ok(response.json::<TokenResponse>().await?)
        } else {
            let msg = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(WhoopError::AuthenticationError(msg))
        }
    }
}
