use crate::helpers::{assert_is_redirect_to, create_unconfirmed_subscriber, spawn_app, TestApp};
use wiremock::matchers::{any, method, path};
use wiremock::{Mock, ResponseTemplate};
#[tokio::test]
async fn newsletters_are_not_delivered_to_unconfirmed_subscribers() {
    // Arrange
    let app = spawn_app().await;
    create_unconfirmed_subscriber(&app).await;
    Mock::given(any())
        .respond_with(ResponseTemplate::new(200))
        // We assert that no request is fired at Postmark!
        .expect(0)
        .mount(&app.email_server)
        .await;

    let login_body = serde_json::json!({
    "username": &app.test_user.username,
    "password": &app.test_user.password
    });
    let response = app.post_login(&login_body).await;
    assert_is_redirect_to(&response, "/admin/dashboard");

    let newsletter_request_body = serde_json::json!({
    "title": "Test",
    "content": "Test",
    });

    app.post_newsletters(&newsletter_request_body).await;
}

#[tokio::test]
async fn invalid_session_is_rejected() {
    // Arrange
    let app = spawn_app().await;

    let newsletter_request_body = serde_json::json!({
    "title": "Test",
    "content": "Test",
    });

    let response = app.post_newsletters(&newsletter_request_body).await;
    // Assert
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn newsletters_are_delivered_to_confirmed_subscribers() {
    // Arrange
    let app = spawn_app().await;
    app.test_user.login(&app).await;

    create_confirmed_subscriber(&app).await;
    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;
    // Act

    let newsletter_request_body = serde_json::json!({
    "title": "Test",
    "content": "Test",
    });

    let response = app.post_newsletters(&newsletter_request_body).await;
    // Assert
    assert_is_redirect_to(&response, "/admin/newsletters");

    let html_page = app.get_newsletters_html().await;
    assert!(html_page.contains("<p><i>newsletters_are_delivered_to_confirmed_subscribers</i></p>"));
}

async fn create_confirmed_subscriber(app: &TestApp) {
    // We can then reuse the same helper and just add
    // an extra step to actually call the confirmation link!
    let confirmation_link = create_unconfirmed_subscriber(app).await;
    reqwest::get(confirmation_link.html)
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
}

#[tokio::test]
async fn newsletters_returns_400_for_invalid_data() {
    // Arrange
    let app = spawn_app().await;
    app.test_user.login(&app).await;
    let test_cases = vec![
        serde_json::json!({
        "content": "Test",
        }),
        serde_json::json!({
        "title": "Test",
        }),
    ];
    for invalid_body in test_cases {
        let response = app.post_newsletters(&invalid_body).await;

        assert_eq!(400, response.status().as_u16(),);
    }
}

#[tokio::test]
async fn newsletters_returns_error_message_for_empty_data() {
    // Arrange
    let app = spawn_app().await;
    app.test_user.login(&app).await;
    let test_cases = vec![
        serde_json::json!({
            "title": "Test",
            "content": "",
        }),
        serde_json::json!({
            "content": "Test",
            "title": "",
        }),
    ];
    for invalid_body in test_cases {
        let response = app.post_newsletters(&invalid_body).await;

        // Assert
        assert_is_redirect_to(&response, "/admin/newsletters");
        let html_page = app.get_newsletters_html().await;
        assert!(html_page.contains("<p><i>Fields cannot be empty</i></p>"));
    }
}

#[tokio::test]
async fn newsletter_creation_is_idempotent() {
    // Arrange
    let app = spawn_app().await;
    create_confirmed_subscriber(&app).await;
    app.test_user.login(&app).await;
    Mock::given(path("/email"))
        .and(method("POST"))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&app.email_server)
        .await;
    // Act - Part 1 - Submit newsletter form
    let newsletter_request_body = serde_json::json!({
    "title": "Newsletter title",
    "content": "Newsletter body as plain text",
    "html_content": "<p>Newsletter body as HTML</p>",
    // We expect the idempotency key as part of the
    // form data, not as an header
    "idempotency_key": uuid::Uuid::new_v4().to_string()
    });
    let response = app.post_newsletters(&newsletter_request_body).await;
    assert_is_redirect_to(&response, "/admin/newsletters");
    // Act - Part 2 - Follow the redirect
    let html_page = app.get_newsletters_html().await;
    assert!(html_page.contains("<p><i>The newsletter issue has been published!</i></p>"));
    let response = app.post_newsletters(&newsletter_request_body).await;
    assert_is_redirect_to(&response, "/admin/newsletters");
    // Act - Part 4 - Follow the redirect
    let html_page = app.get_newsletters_html().await;
    assert!(html_page.contains("<p><i>The newsletter issue has been published!</i></p>"));
}
