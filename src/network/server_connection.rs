use reqwest::{Client, Error, IntoUrl, Method, Response, Url};
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

    /// Does a HTTP request and returns the raw response.
    ///
    /// # Arguments
    ///
    /// * `method`: HTTP method to use.
    /// * `route`: The route after the base url, include optional path variables.
    /// * `body`: Optional data that needs to be JSON serialized into the request body.
    ///
    /// returns: Result<Response, Error>
    pub async fn request<S: Serialize>(
        &self,
        method: Method,
        route: impl AsRef<str>,
        body: Option<&S>,
    ) -> Result<Response, Error> {
        let mut req = self.http_client.request(method, self.join_url(route));
        if let Some(body) = body {
            req = req.json(body);
        }

        let res = req.send().await;
        if let Ok(res) = res {
            res.error_for_status()
        } else {
            res
        }
    }

    /// Does a HTTP request and transforms the response body to a JSON object.
    ///
    /// # Arguments
    /// * 'method' - HTTP method to use.
    /// * 'route' - The route after the base url, include optional path variables.
    /// * 'body' - Optional data that needs to be JSON serialized into the request body.
    ///
    /// returns: Result<Response, Error>
    pub async fn request_with_result<T: DeserializeOwned, S: Serialize>(
        &self,
        method: Method,
        route: impl AsRef<str>,
        body: Option<&S>,
    ) -> Result<T, Error> {
        match self.request(method, route, body).await {
            Ok(res) => res.json::<T>().await,
            Err(e) => Err(e),
        }
    }

    fn join_url(&self, route: impl AsRef<str>) -> Url {
        let route_ref = route.as_ref();
        assert!(!route_ref.starts_with('/'), "route must be relative");
        self.base_url.join(route_ref).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use reqwest::{Method, Url};
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use crate::network::*;

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
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_info.to_owned()))
            .mount(&mock_server)
            .await;
        let conn = ServerConnection::new(mock_server.uri());
        let response_json: WhistInfo = conn
            .request_with_result(Method::GET, "route", Option::<&()>::None)
            .await
            .unwrap();
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
        let response_json = conn
            .request(Method::POST, "route", Some(&expected_info))
            .await
            .unwrap();
        assert_eq!(response_json.status(), 200);
    }
}
