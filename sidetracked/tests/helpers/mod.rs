use axum_test::{TestServer, TestServerConfig};

use sidetracked_lib::web::application::Application;

// @<newtestapp
#[cfg(test)]
pub async fn new_test_app() -> TestServer {
    let app = Application.router();

    let config = TestServerConfig::builder()
        .save_cookies()
        .expect_success_by_default()
        .mock_transport()
        .build();

    TestServer::new_with_config(app, config).unwrap()
}
// >@
