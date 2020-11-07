#![doc = "generated by AutoRust 0.1.0"]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::models::*;
use reqwest::StatusCode;
use snafu::{ResultExt, Snafu};
pub mod registries {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn check_name_availability(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        registry_name_check_request: &RegistryNameCheckRequest,
    ) -> std::result::Result<RegistryNameStatus, check_name_availability::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.ContainerRegistry/checkNameAvailability",
            &operation_config.base_path, subscription_id
        );
        let mut req_builder = client.post(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(check_name_availability::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(registry_name_check_request);
        let req = req_builder.build().context(check_name_availability::BuildRequestError)?;
        let rsp = client.execute(req).await.context(check_name_availability::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(check_name_availability::ResponseBytesError)?;
                let rsp_value: RegistryNameStatus =
                    serde_json::from_slice(&body).context(check_name_availability::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(check_name_availability::ResponseBytesError)?;
                check_name_availability::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod check_name_availability {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn get(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        registry_name: &str,
    ) -> std::result::Result<Registry, get::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}",
            &operation_config.base_path, subscription_id, resource_group_name, registry_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(get::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(get::BuildRequestError)?;
        let rsp = client.execute(req).await.context(get::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                let rsp_value: Registry = serde_json::from_slice(&body).context(get::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(get::ResponseBytesError)?;
                get::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod get {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn create(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        registry_name: &str,
        registry_create_parameters: &RegistryCreateParameters,
    ) -> std::result::Result<create::Response, create::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}",
            &operation_config.base_path, subscription_id, resource_group_name, registry_name
        );
        let mut req_builder = client.put(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(create::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(registry_create_parameters);
        let req = req_builder.build().context(create::BuildRequestError)?;
        let rsp = client.execute(req).await.context(create::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(create::ResponseBytesError)?;
                let rsp_value: Registry = serde_json::from_slice(&body).context(create::DeserializeError { body })?;
                Ok(create::Response::Ok200(rsp_value))
            }
            StatusCode::ACCEPTED => Ok(create::Response::Accepted202),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(create::ResponseBytesError)?;
                create::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod create {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200(Registry),
            Accepted202,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn update(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        registry_name: &str,
        registry_update_parameters: &RegistryUpdateParameters,
    ) -> std::result::Result<Registry, update::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}",
            &operation_config.base_path, subscription_id, resource_group_name, registry_name
        );
        let mut req_builder = client.patch(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(update::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(registry_update_parameters);
        let req = req_builder.build().context(update::BuildRequestError)?;
        let rsp = client.execute(req).await.context(update::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(update::ResponseBytesError)?;
                let rsp_value: Registry = serde_json::from_slice(&body).context(update::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(update::ResponseBytesError)?;
                update::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod update {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn delete(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        registry_name: &str,
    ) -> std::result::Result<delete::Response, delete::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}",
            &operation_config.base_path, subscription_id, resource_group_name, registry_name
        );
        let mut req_builder = client.delete(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(delete::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(delete::BuildRequestError)?;
        let rsp = client.execute(req).await.context(delete::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => Ok(delete::Response::Ok200),
            StatusCode::NO_CONTENT => Ok(delete::Response::NoContent204),
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(delete::ResponseBytesError)?;
                delete::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod delete {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug)]
        pub enum Response {
            Ok200,
            NoContent204,
        }
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn list_by_resource_group(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
    ) -> std::result::Result<RegistryListResult, list_by_resource_group::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries",
            &operation_config.base_path, subscription_id, resource_group_name
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(list_by_resource_group::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(list_by_resource_group::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_by_resource_group::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_resource_group::ResponseBytesError)?;
                let rsp_value: RegistryListResult =
                    serde_json::from_slice(&body).context(list_by_resource_group::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_by_resource_group::ResponseBytesError)?;
                list_by_resource_group::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod list_by_resource_group {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn list(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
    ) -> std::result::Result<RegistryListResult, list::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/providers/Microsoft.ContainerRegistry/registries",
            &operation_config.base_path, subscription_id
        );
        let mut req_builder = client.get(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(list::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(list::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                let rsp_value: RegistryListResult = serde_json::from_slice(&body).context(list::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                list::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod list {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn list_credentials(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        registry_name: &str,
    ) -> std::result::Result<RegistryListCredentialsResult, list_credentials::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/listCredentials",
            &operation_config.base_path, subscription_id, resource_group_name, registry_name
        );
        let mut req_builder = client.post(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(list_credentials::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.header(reqwest::header::CONTENT_LENGTH, 0);
        let req = req_builder.build().context(list_credentials::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list_credentials::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_credentials::ResponseBytesError)?;
                let rsp_value: RegistryListCredentialsResult =
                    serde_json::from_slice(&body).context(list_credentials::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list_credentials::ResponseBytesError)?;
                list_credentials::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod list_credentials {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
    pub async fn regenerate_credential(
        operation_config: &crate::OperationConfig,
        subscription_id: &str,
        resource_group_name: &str,
        registry_name: &str,
        regenerate_credential_parameters: &RegenerateCredentialParameters,
    ) -> std::result::Result<RegistryListCredentialsResult, regenerate_credential::Error> {
        let client = &operation_config.client;
        let uri_str = &format!(
            "{}/subscriptions/{}/resourceGroups/{}/providers/Microsoft.ContainerRegistry/registries/{}/regenerateCredential",
            &operation_config.base_path, subscription_id, resource_group_name, registry_name
        );
        let mut req_builder = client.post(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(regenerate_credential::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        req_builder = req_builder.json(regenerate_credential_parameters);
        let req = req_builder.build().context(regenerate_credential::BuildRequestError)?;
        let rsp = client.execute(req).await.context(regenerate_credential::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(regenerate_credential::ResponseBytesError)?;
                let rsp_value: RegistryListCredentialsResult =
                    serde_json::from_slice(&body).context(regenerate_credential::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(regenerate_credential::ResponseBytesError)?;
                regenerate_credential::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod regenerate_credential {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
}
pub mod operations {
    use crate::models::*;
    use reqwest::StatusCode;
    use snafu::{ResultExt, Snafu};
    pub async fn list(operation_config: &crate::OperationConfig) -> std::result::Result<OperationListResult, list::Error> {
        let client = &operation_config.client;
        let uri_str = &format!("{}/providers/Microsoft.ContainerRegistry/operations", &operation_config.base_path,);
        let mut req_builder = client.get(uri_str);
        if let Some(token_credential) = &operation_config.token_credential {
            let token_response = token_credential
                .get_token(&operation_config.token_credential_resource)
                .await
                .context(list::GetTokenError)?;
            req_builder = req_builder.bearer_auth(token_response.token.secret());
        }
        req_builder = req_builder.query(&[("api-version", &operation_config.api_version)]);
        let req = req_builder.build().context(list::BuildRequestError)?;
        let rsp = client.execute(req).await.context(list::ExecuteRequestError)?;
        match rsp.status() {
            StatusCode::OK => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                let rsp_value: OperationListResult = serde_json::from_slice(&body).context(list::DeserializeError { body })?;
                Ok(rsp_value)
            }
            status_code => {
                let body: bytes::Bytes = rsp.bytes().await.context(list::ResponseBytesError)?;
                list::UnexpectedResponse { status_code, body: body }.fail()
            }
        }
    }
    pub mod list {
        use crate::{models, models::*};
        use reqwest::StatusCode;
        use snafu::Snafu;
        #[derive(Debug, Snafu)]
        #[snafu(visibility(pub(crate)))]
        pub enum Error {
            UnexpectedResponse { status_code: StatusCode, body: bytes::Bytes },
            BuildRequestError { source: reqwest::Error },
            ExecuteRequestError { source: reqwest::Error },
            ResponseBytesError { source: reqwest::Error },
            DeserializeError { source: serde_json::Error, body: bytes::Bytes },
            GetTokenError { source: azure_core::errors::AzureError },
        }
    }
}
