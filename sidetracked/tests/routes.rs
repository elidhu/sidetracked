// @<modtestroutes
mod helpers;
// >@

use axum::http::StatusCode;

// @<testhealthcheck
#[cfg(test)]
mod test_health_check {
    use super::*;

    #[tokio::test]
    async fn it_should_return_200() {
        // Arrange
        let app = helpers::new_test_app().await;

        // Act & Assert
        app.get("/health_check").await.assert_status(StatusCode::OK);
    }
}
// >@
