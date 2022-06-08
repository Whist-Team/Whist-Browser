#[cfg(test)]
mod test_server_connection {
    use whist_browser::server_connection::get_json;
    use whist_browser::whist_info::{WhistInfo, WhistInfoFactory};

    #[tokio::test]
    async fn test_get_json() {
        let url = "http://localhost:9001";
        let response_json = get_json::<WhistInfo>(&url).await.unwrap();
        let expected_info = WhistInfoFactory::new_info("whist", "0.1.0");
        assert_eq!(response_json, expected_info);
    }
}
