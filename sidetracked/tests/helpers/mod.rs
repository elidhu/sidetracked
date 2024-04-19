use axum_test::{TestServer, TestServerConfig};
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use jwt_authorizer::{layer::JwtSource, AuthorizerBuilder};
use sha2::Sha256;

use sidetracked_lib::web::application::Application;
use sidetracked_lib::web::auth::Claims;

// @<testsecret
const TEST_SECRET: &str = "7750e0e7ad62179c3a5299f40ec6fb69fffa0b95aff0424955f654012e5cedb5";
// >@

// @<newtestapp
#[cfg(test)]
pub async fn new_test_app() -> TestServer {
    let authorizer = AuthorizerBuilder::<Claims>::from_secret(TEST_SECRET)
        .jwt_source(JwtSource::AuthorizationHeader)
        .build()
        .await
        .expect("Failed to build authorizer");

    let app = Application::new(authorizer);

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

    TestServer::new_with_config(app.router(), config).unwrap()
}
// >@

// @<newtesttoken
#[cfg(test)]
pub async fn new_test_token(claims: Claims) -> String {
    let key: Hmac<Sha256> =
        Hmac::new_from_slice(TEST_SECRET.as_bytes()).expect("Failed to create key");

    claims.sign_with_key(&key).expect("Failed to sign token")
}
// >@
