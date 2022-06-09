#[cfg(test)]
mod test_server_connection {
    use whist_browser::response::whist_info::{WhistInfo, WhistInfoFactory};
    use whist_browser::server_connection::get_json;
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_get_json() {
        let expected_info = WhistInfoFactory::new_info("whist", "0.1.0");

        let mock_server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(ResponseTemplate::new(200).set_body_json(expected_info.clone()))
            .mount(&mock_server)
            .await;
        let response_json = get_json::<WhistInfo>(&mock_server.uri()).await.unwrap();
        assert_eq!(response_json, expected_info);
    }
}
