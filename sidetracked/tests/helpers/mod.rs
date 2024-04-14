use axum_test::{TestServer, TestServerConfig};

use sidetracked_lib::web::application::Application;

// @<newtestapp
#[cfg(test)]
pub async fn new_test_app() -> TestServer {
    let app = Application.router();

    let config = TestServerConfig::builder()
        // Use an actual HTTP transport on a random port.
        .http_transport()
        // Behave like a browser and save cookies between requests.
        .save_cookies()
        // We are testing a JSON API.
        .default_content_type("application/json")
        // Panic if the response is outside the 2XX range (Unless request marked as expected failure).
        .expect_success_by_default()
        .build();

    TestServer::new_with_config(app, config).unwrap()
}
// >@
