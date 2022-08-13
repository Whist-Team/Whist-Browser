use std::fmt::{Debug, Formatter};

use bevy::prelude::*;
use reqwest::{Client, Error, IntoUrl, Method, Response, Url};
use reqwest::header::{ACCEPT, HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub enum Query<'a, S: Serialize + Debug = ()> {
    None,
    Some(&'a S),
}

impl<S: Serialize + Debug> Debug for Query<'_, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Query::None => write!(f, "None"),
            Query::Some(data) => write!(f, "Some({:?})", data),
        }
    }
}

pub enum Body<'a, S: Serialize + Debug = ()> {
    Empty,
    Json(&'a S),
    Form(&'a S),
}

impl<S: Serialize + Debug> Debug for Body<'_, S> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Body::Empty => write!(f, "Empty"),
            Body::Json(data) => write!(f, "Json({:?})", data),
            Body::Form(data) => write!(f, "Form({:?})", data),
        }
    }
}

/// Provides basic REST communication with the server.
pub struct ServerConnection {
    /// The main url without any routes.
    base_url: Url,
    /// HTTP client, uses an internal connection pool and can thus be shared
    http_client: Client,
    /// Authorization token
    token: Option<String>,
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
            token: None,
        }
    }

    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    pub fn token(&mut self, token: impl Into<String>) {
        self.token = Some(token.into());
    }

    pub fn remove_token(&mut self) {
        self.token = None;
    }

    /// Does a HTTP request and returns the raw response.
    ///
    /// # Arguments
    ///
    /// * `method`: HTTP method to use.
    /// * `route`: The route after the base url, include optional path variables.
    /// * `query`: Optional query parameters to append to the url.
    /// * `body`: Optional data that needs to be serialized into the request body.
    /// * `headers`: Optional additional header fields to set.
    ///
    /// returns: Result<Response, Error>
    pub async fn request<Q: Serialize + Debug, B: Serialize + Debug>(
        &self,
        method: Method,
        route: impl AsRef<str>,
        query: Query<'_, Q>,
        body: Body<'_, B>,
        headers: Option<HeaderMap>,
    ) -> Result<Response, Error> {
        info!(
            "http request: {} {}{} query={:?} body={:?} headers={:?} auth={:?}",
            method,
            self.base_url,
            route.as_ref(),
            query,
            body,
            headers,
            self.token
        );
        let mut req = self.http_client.request(method, self.join_url(route));

        if let Some(headers) = headers {
            req = req.headers(headers)
        }

        if let Query::Some(query) = query {
            req = req.query(query);
        }

        match body {
            Body::Empty => {}
            Body::Json(json_body) => {
                req = req.json(json_body);
            }
            Body::Form(form_body) => {
                req = req.form(form_body);
            }
        }

        if let Some(token) = &self.token {
            req = req.bearer_auth(token);
        }

        let resp = req.send().await?;
        info!("http response: {:?}", resp);
        resp.error_for_status()
    }

    /// Does a HTTP request and transforms the response body to a JSON object.
    ///
    /// # Arguments
    /// * 'method' - HTTP method to use.
    /// * 'route' - The route after the base url, include optional path variables.
    /// * `query`: Optional query parameters to append to the url.
    /// * 'body' - Optional data that needs to be serialized into the request body.
    /// * `headers`: Optional additional header fields to set.
    ///
    /// returns: Result<Response, Error>
    pub async fn request_with_json_result<
        Q: Serialize + Debug,
        B: Serialize + Debug,
        R: DeserializeOwned + Debug,
    >(
        &self,
        method: Method,
        route: impl AsRef<str>,
        query: Query<'_, Q>,
        body: Body<'_, B>,
        headers: Option<HeaderMap>,
    ) -> Result<R, Error> {
        let mut headers = headers.unwrap_or_default();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        self.request(method, route, query, body, Some(headers))
            .await?
            .json::<R>()
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
    use pretty_assertions::assert_eq;
    use reqwest::{Method, Url};
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::method;

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
        let expected_info = WhistInfo::new("whist", "0.1.0", "0.1.0");

        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_info.to_owned()))
            .mount(&mock_server)
            .await;
        let conn = ServerConnection::new(mock_server.uri());
        let response_json: WhistInfo = conn
            .request_with_json_result(
                Method::GET,
                "route",
                Query::<()>::None,
                Body::<()>::Empty,
                None,
            )
            .await
            .unwrap();
        assert_eq!(response_json, expected_info);
    }

    #[tokio::test]
    async fn test_post_json_without_response_body() {
        let expected_info = WhistInfo::new("whist", "0.1.0", "0.1.0");

        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        let conn = ServerConnection::new(mock_server.uri());
        let response_json = conn
            .request(
                Method::POST,
                "route",
                Query::<()>::None,
                Body::Json(&expected_info),
                None,
            )
            .await
            .unwrap();
        assert_eq!(response_json.status(), 200);
    }
}
