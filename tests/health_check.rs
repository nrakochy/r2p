fn spawn_app() {
    let server = r2p::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}

#[tokio::test]
async fn health_check_works() {
    spawn_app();
    let client = reqwest::Client::new();

    let resp = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to launch");
    assert!(resp.status().is_success());
    assert_eq!(Some(0), resp.content_length())
}
