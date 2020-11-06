use azure_core::errors::AzureError;
use azure_core::{TokenCredential, TokenResponse};
use chrono::Utc;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AccessToken, AuthType, AuthUrl, Scope, TokenUrl,
};
use std::{borrow::Cow, str, time::Duration};
use url::Url;

/// Provides options to configure how the Identity library makes authentication
/// requests to Azure Active Directory.
#[derive(Clone, Debug, PartialEq)]
pub struct TokenCredentialOptions {
    /// The authority host to use for authentication requests.  The default is
    /// "https://login.microsoftonline.com".
    pub authority_host: Option<Cow<'static, str>>,
}

const DEFAULT_TOKEN_CREDENTIAL_OPTIONS: TokenCredentialOptions = TokenCredentialOptions {
    authority_host: Some(Cow::Borrowed(authority_hosts::AZURE_PUBLIC_CLOUD)),
};

impl Default for TokenCredentialOptions {
    fn default() -> Self {
        DEFAULT_TOKEN_CREDENTIAL_OPTIONS
    }
}

impl TokenCredentialOptions {
    pub fn authority_host(&self) -> &str {
        match &self.authority_host {
            Some(authority_host) => authority_host,
            None => authority_hosts::AZURE_PUBLIC_CLOUD,
        }
    }
}

/// A list of known Azure authority hosts
pub mod authority_hosts {
    /// China-based Azure Authority Host
    pub const AZURE_CHINA: &str = "https://login.chinacloudapi.cn";
    /// Germany-based Azure Authority Host
    pub const AZURE_GERMANY: &str = "https://login.microsoftonline.de";
    /// US Government Azure Authority Host
    pub const AZURE_GOVERNMENT: &str = "https://login.microsoftonline.us";
    /// Public Cloud Azure Authority Host
    pub const AZURE_PUBLIC_CLOUD: &str = "https://login.microsoftonline.com";
}

pub mod tenant_ids {
    pub const TENANT_ID_COMMON: &str = "common";
    /// Active Directory Federated Services
    pub const TENANT_ID_ADFS: &str = "adfs";
}

/// Enables authentication to Azure Active Directory using a client secret that was generated for an App Registration.
///
/// More information on how to configure a client secret can be found here:
/// https://docs.microsoft.com/en-us/azure/active-directory/develop/quickstart-configure-app-access-web-apis#add-credentials-to-your-web-application
pub struct ClientSecretCredential {
    tenant_id: String,
    client_id: oauth2::ClientId,
    client_secret: Option<oauth2::ClientSecret>,
    options: TokenCredentialOptions,
}

impl ClientSecretCredential {
    pub fn new(
        tenant_id: String,
        client_id: String,
        client_secret: String,
        options: TokenCredentialOptions,
    ) -> ClientSecretCredential {
        ClientSecretCredential {
            tenant_id,
            client_id: oauth2::ClientId::new(client_id),
            client_secret: Some(oauth2::ClientSecret::new(client_secret)),
            options,
        }
    }

    fn options(&self) -> &TokenCredentialOptions {
        &self.options
    }
}

#[async_trait::async_trait]
impl TokenCredential for ClientSecretCredential {
    async fn get_token(&self, resource: &str) -> Result<TokenResponse, AzureError> {
        let options = self.options();
        let authority_host = options.authority_host();

        let token_url = TokenUrl::from_url(
            Url::parse(&format!(
                "{}/{}/oauth2/v2.0/token",
                authority_host, self.tenant_id
            ))
            .map_err(|_| {
                AzureError::GenericErrorWithText(format!(
                    "Failed to construct token endpoint with tenant id {}",
                    self.tenant_id,
                ))
            })?,
        );

        let auth_url = AuthUrl::from_url(
            Url::parse(&format!(
                "{}/{}/oauth2/v2.0/authorize",
                authority_host, self.tenant_id
            ))
            .map_err(|_| {
                AzureError::GenericErrorWithText(format!(
                    "Failed to construct authorize endpoint with tenant id {}",
                    self.tenant_id,
                ))
            })?,
        );

        let client = BasicClient::new(
            self.client_id.clone(),
            self.client_secret.clone(),
            auth_url,
            Some(token_url),
        )
        .set_auth_type(AuthType::RequestBody);

        let token_result = client
            .exchange_client_credentials()
            .add_scope(Scope::new(format!("{}.default", resource)))
            .request_async(async_http_client)
            .await
            .map(|r| {
                use oauth2::TokenResponse as _;
                TokenResponse::new(
                    AccessToken::new(r.access_token().secret().to_owned()),
                    Utc::now()
                        + chrono::Duration::from_std(
                            r.expires_in().unwrap_or(Duration::from_secs(0)),
                        )
                        .unwrap(),
                )
            })
            .map_err(|e| match e {
                oauth2::RequestTokenError::ServerResponse(s) => AzureError::GenericErrorWithText(
                    s.error_description()
                        .unwrap_or(&"Server error without description".to_string())
                        .to_owned(),
                ),
                _ => AzureError::GenericErrorWithText("OAuth2 error".to_string()),
            })?;

        Ok(token_result)
    }
}
