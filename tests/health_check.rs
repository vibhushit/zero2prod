//! tests/health_check.rs


// `tokio::test` is the testing equivalent of `tokio::main`.
//It also spares you from having to specify the `#[test]` attribute.

//you can inspecct what code gets generated using
//`cargo expand --test helath_check` (<- name of the test file)
#[tokio::test]
async fn health_check_works() {
    //Arrange
    spawn_app();
    // We need to bring in `reqwest`
    //to perform HTTP requests against our application.
    let client =reqwest::Client::new();

    //Act
    let response = client
        .get("http://127.0.0.1:8000/health_check")
        .send()
        .await
        .expect("Failed to execcute request.");

    //Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app()  {
    let server = zero2prod::run().expect("Failed to bind address");
    let _ = tokio::spawn(server);
}