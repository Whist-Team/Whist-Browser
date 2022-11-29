#[tokio::test]
#[ignore]
async fn test_start() {
    let service = whist_browser::network::ServerService::new("http://localhost:8080");
    let response = service.check_connection().await;
    assert_eq!(true, response.is_ok());
}

#[tokio::test]
#[ignore]
async fn test_game_info() {
    let service = whist_browser::network::ServerService::new("http://localhost:8080");
    let whist_info = service.get_info().await.unwrap();
    assert_eq!(whist_info.info.game, "whist");
}
