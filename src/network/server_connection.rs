use reqwest::{Client, Error, IntoUrl, Response, Url};
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Provides basic REST communication with the server.
pub struct ServerConnection {
    /// The main url without any routes.
    base_url: Url,
    /// HTTP client, uses an internal connection pool and can thus be shared
    http_client: Client,
}

impl ServerConnection {
    /// Constructor for creating a new Server Connection.
    /// # Arguments
    /// * 'base_url' the url of the server
    pub fn new(base_url: impl IntoUrl) -> Self {
        let url = base_url.into_url().unwrap();
        assert!(url.path().ends_with('/'), "base_url path must end with '/'");
        Self {
            base_url: url,
            http_client: Client::new(),
        }
    }

    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// Requests a GET route and transforms it to a JSON object.
    /// # Arguments
    /// * 'route' - The route after the base url from above include optional path variables.
    pub async fn get_json<T: DeserializeOwned>(&self, route: impl AsRef<str>) -> Result<T, Error> {
        self.http_client
            .get(self.join_url(route))
            .send()
            .await?
            .json::<T>()
            .await
    }

    /// Posts a JSON object to the whist server. It returns the full response which must be
    /// processed somewhere else.
    /// # Arguments
    /// * 'route' - The route after the base url from above include optional path variables.
    /// * 'data' - A serializable object to be send to the server.
    pub async fn post_json<D: Serialize>(
        &self,
        route: impl AsRef<str>,
        data: &D,
    ) -> Result<Response, Error> {
        self.http_client
            .post(self.join_url(route))
            .json(data)
            .send()
            .await
    }

    fn join_url(&self, route: impl AsRef<str>) -> Url {
        let route_ref = route.as_ref();
        assert!(!route_ref.starts_with('/'), "route must be relative");
        self.base_url.join(route_ref).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::network::*;
    use reqwest::Url;
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn test_join_url() {
        let conn = ServerConnection::new("https://doc.rust-lang.org");
        assert_eq!(
            conn.join_url("index.html"),
            Url::parse("https://doc.rust-lang.org/index.html").unwrap()
        );
    }

    #[test]
    fn test_join_url_with_base_path() {
        let conn = ServerConnection::new("https://doc.rust-lang.org/rust-by-example/");
        assert_eq!(
            conn.join_url("index.html"),
            Url::parse("https://doc.rust-lang.org/rust-by-example/index.html").unwrap()
        );
    }

    #[tokio::test]
    async fn test_get_json() {
        let expected_info = WhistInfo::new("whist", "0.1.0");

        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_info.clone()))
            .mount(&mock_server)
            .await;
        let conn = ServerConnection::new(mock_server.uri());
        let response_json: WhistInfo = conn.get_json("route").await.unwrap();
        assert_eq!(response_json, expected_info);
    }

    #[tokio::test]
    async fn test_post_json_without_response_body() {
        let expected_info = WhistInfo::new("whist", "0.1.0");

        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        let conn = ServerConnection::new(mock_server.uri());
        let response_json = conn.post_json("route", &expected_info).await.unwrap();
        assert_eq!(response_json.status(), 200);
    }
}
