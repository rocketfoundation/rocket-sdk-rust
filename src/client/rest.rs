use crate::{
    endpoints::{HTTPMethod, RocketChainRequest},
    error::{ClientError, Error},
};

/// A basic example client for sending requests and transactions to the rocket chain node.
#[derive(Debug, Clone)]
pub struct RestClient {
    inner_client: reqwest::Client,
    base_url: reqwest::Url,
}

impl RestClient {
    /// Create a new instance with default [reqwest::Client].
    /// Example:
    /// ```
    /// use rocket_chain_sdk::client::rest::RestClient;
    ///
    /// let client_instance = RestClient::new("http://127.0.0.1:3000");
    /// ```
    pub fn new<U: reqwest::IntoUrl>(base_url: U) -> Result<Self, reqwest::Error> {
        Self::new_with_inner(base_url, reqwest::Client::new())
    }

    /// Create a new instance with a premade [reqwest::Client].
    pub fn new_with_inner<U: reqwest::IntoUrl>(
        base_url: U,
        inner_client: reqwest::Client,
    ) -> Result<Self, reqwest::Error> {
        Ok(Self {
            inner_client,
            base_url: base_url.into_url()?,
        })
    }

    /// Send a request and wait for response.
    pub async fn send_request<R: RocketChainRequest>(
        &self,
        request: R,
    ) -> Result<R::Response, Error> {
        let mut url = self.base_url.clone();
        url.set_path(R::ENDPOINT);

        let prepared_request = match R::HTTP_METHOD {
            HTTPMethod::GET => self.inner_client.get(url).query(&request),
            HTTPMethod::POST => {
                let serialized_request = serde_json::to_string(&request)
                    .map_err(|err| ClientError::SerializeResponse(err))?;
                self.inner_client.post(url).body(serialized_request)
            }
        };

        let response = prepared_request.send().await.map_err(ClientError::Send)?;
        let response_text = response
            .error_for_status()
            .map_err(ClientError::Send)?
            .text()
            .await
            .map_err(ClientError::Send)?;

        serde_json::from_str::<R::Response>(&response_text)
            .map_err(|err| ClientError::DeserializeResponse(err).into())
    }
}
