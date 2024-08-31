//! tests/health_check.rs

// `tokio::test` is the testing equivalent of `tokio::main`.
//It also spares you from having to specify the `#[test]` attribute.

use std::net::TcpListener;

//you can inspecct what code gets generated using
//`cargo expand --test helath_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    //Arrange
    let address = spawn_app();
    // We need to bring in `reqwest`
    //to perform HTTP requests against our application.
    let client = reqwest::Client::new();

    //Act
    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execcute request.");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("failed to bind address");
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}
