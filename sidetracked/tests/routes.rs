// @<modtestroutes
mod helpers;
// >@

use axum::http::{header::AUTHORIZATION, HeaderValue, StatusCode};

use sidetracked_lib::web::auth::Claims;

// @<testhealthcheck
#[cfg(test)]
mod test_health_check {
    use super::*;

    #[tokio::test]
    async fn it_should_return_200() {
        // Arrange
        let app = helpers::new_test_app().await;

        // Act
        let response = app.get("/health_check").await;

        // Assert
        response.assert_status(StatusCode::OK);
    }
}
// >@

// @<testprofile
#[cfg(test)]
mod test_profile {
    use super::*;

    // @<testprofile401
    #[tokio::test]
    async fn it_should_return_401() {
        // Arrange
        let mut app = helpers::new_test_app().await;
        app.expect_failure();

        // Act
        let response = app.get("/profile").await;

        // Assert
        response.assert_status(StatusCode::UNAUTHORIZED);
    }
    // >@

    // @<testprofile200
    #[tokio::test]
    async fn it_should_return_200() {
        // Arrange
        let mut app = helpers::new_test_app().await;

        // Construct some valid claims
        let test_claims = Claims {
            sub: "018eef43-1283-70dd-b738-5bc64b3313c5".to_string(),
            name: "Jason Asano".to_string(),
            iat: 1713411102,
            // Set the expiration time to a time in the future. One day this may be a problem - but
            // I don't think it is worth worrying about :)
            exp: 1913411102,
        };

        // Sign the claims to create a token
        let test_token = helpers::new_test_token(test_claims.clone()).await;

        // Add the token to the Authorization header
        app.add_header(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {test_token}")).unwrap(),
        );

        // Act
        let response = app.get("/profile").await;

        // Assert
        response.assert_status(StatusCode::OK);
        response.assert_json::<Claims>(&test_claims);
    }
    // >@

    // @<testprofile401invalid
    #[tokio::test]
    async fn it_should_return_401_invalid_token() {
        // Arrange
        let mut app = helpers::new_test_app().await;
        app.expect_failure();

        // Add an invalid token to the Authorization header
        app.add_header(
            AUTHORIZATION,
            HeaderValue::from_str("Bearer invalid_token").unwrap(),
        );

        // Act
        let response = app.get("/profile").await;

        // Assert
        response.assert_status(StatusCode::UNAUTHORIZED);
    }
    // >@

    // @<testprofile401expired
    #[tokio::test]
    async fn it_should_return_401_expired_token() {
        // Arrange
        let mut app = helpers::new_test_app().await;
        app.expect_failure();

        let test_claims = Claims {
            sub: "018eef43-1283-70dd-b738-5bc64b3313c5".to_string(),
            name: "Jason Asano".to_string(),
            iat: 1713411102,
            // Set the expiration time to a time in the past
            exp: 1713411102,
        };

        let test_token = helpers::new_test_token(test_claims.clone()).await;

        app.add_header(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {test_token}")).unwrap(),
        );

        // Act
        let response = app.get("/profile").await;

        // Assert
        response.assert_status(StatusCode::UNAUTHORIZED);
    }
    // >@
}
// >@
