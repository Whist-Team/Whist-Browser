#[cfg(test)]
mod test_server_connection {
    use reqwest::Response;
    use whist_browser::response::whist_info::{WhistInfo, WhistInfoFactory};
    use whist_browser::server_connection::ServerConnection;
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_json() {
        let expected_info = WhistInfoFactory::new_info(String::from("whist"), String::from("0.1.0"));

        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_info.clone()))
            .mount(&mock_server)
            .await;
        let conn = ServerConnection::new(mock_server.uri() + "/route");
        let response_json = conn.get_json::<WhistInfo>("/route").await.unwrap();
        assert_eq!(response_json, expected_info);
    }

    #[tokio::test]
    async fn test_post_json_without_response_body() {
        let expected_info = WhistInfoFactory::new_info(String::from("whist"), String::from("0.1.0"));

        let mock_server = MockServer::start().await;
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        let conn = ServerConnection::new(mock_server.uri() + "/route");
        let response_json = conn.post_json::<WhistInfo>("/route", expected_info).await.unwrap();
        assert_eq!(response_json.status(), 200);
    }
}
