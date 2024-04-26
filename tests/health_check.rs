use std::net::TcpListener;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("[TCP] Failed to bind test");
    let port = listener.local_addr().unwrap().port();
    let server = r2p::run(listener).expect("Failed to bind address");
    let _ = tokio::spawn(server);
    return format!("http://127.0.0.1:{}", port);
}

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = reqwest::Client::new();
    let resp = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to launch");
    assert!(resp.status().is_success());
    assert_eq!(Some(0), resp.content_length())
}

#[tokio::test]
async fn subscribe_returns_200_valid_form() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let body = "name=le%persona&email=test%40gmail.com";

    let resp = client
        .post(&format!("{}/subscriptions", app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("[API] Failed to execute POST subscriptions.");

    assert_eq!(200, resp.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_400_missing_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20persona", "missing email"),
        ("email=test%40example.com", "missing name"),
        ("", "missing name & email"),
    ];

    for (invalid_body, error_msg) in test_cases {
        let resp = client
            .post(&format!("{}/subscriptions", app_address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to init test");
        assert_eq!(
            400,
            resp.status().as_u16(),
            "[API] Expected failure: {}",
            error_msg
        );
    }
}
