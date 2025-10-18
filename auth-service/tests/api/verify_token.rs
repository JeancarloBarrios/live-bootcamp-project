use crate::helpers::TestApp;
use auth_service::ErrorResponse;

#[tokio::test]
async fn should_return_200_valid_token() {
    let app = TestApp::new().await;

    let response = app
        .post_verify_token(&serde_json::json!({
            "token": "valid_token_string"
        }))
        .await;

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn should_return_401_if_invalid_token() {
    let app = TestApp::new().await;

    // Test with empty token
    let response = app
        .post_verify_token(&serde_json::json!({
            "token": ""
        }))
        .await;

    assert_eq!(response.status().as_u16(), 401);

    assert_eq!(
        response
            .json::<ErrorResponse>()
            .await
            .expect("Could not deserialize response body to ErrorResponse")
            .error,
        "Invalid token".to_owned()
    );
}

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let test_cases = [
        serde_json::json!({}),
        serde_json::json!({"token": 123}),
        serde_json::json!({"wrong_field": "value"}),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_verify_token(test_case).await;
        assert_eq!(
            response.status().as_u16(),
            422,
            "Failed for input: {:?}",
            test_case
        );
    }
}
