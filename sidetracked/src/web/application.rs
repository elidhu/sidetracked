use anyhow::{Context, Result};
use axum::{routing::get, Router};
use jwt_authorizer::{layer::JwtSource, Authorizer, AuthorizerBuilder, IntoLayer};
use std::{
    net::{IpAddr, SocketAddr},
    sync::Arc,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;

use super::{
    auth::Claims,
    handlers::{health_check, profile},
};

// @<run
/// Run the application
pub async fn run(app: Application, config: ApplicationConfig) {
    let addr = SocketAddr::new(config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    let router = app.router();

    info!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router.into_make_service())
        .await
        .expect("Unexpected error during server execution");
}
// >@

// @<structapplicationconfig
pub struct ApplicationConfig {
    /// The host to listen on
    pub host: IpAddr,
    /// The port to listen on
    pub port: u16,
}
// >@

// @<impldefaultapplicationconfig
impl Default for ApplicationConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".parse().unwrap(),
            port: 3000,
        }
    }
}
// >@

// @<structapplication
pub struct Application {
    authorizer: Arc<Authorizer<Claims>>,
}
// >@

// @<implapplication
impl Application {
    // @<applicationnew
    /// Create a new application with the provided authorizer
    pub fn new(authorizer: Authorizer<Claims>) -> Self {
        Self {
            authorizer: Arc::new(authorizer),
        }
    }
    // >@

    // @<applicationnewwithdefaultauthorizer
    /// Create a new application with a default authorizer that expects a secret to be set in the
    /// `SIDETRACKED_SECRET` environment variable.
    pub async fn new_with_default_authorizer() -> Result<Self> {
        let secret = std::env::var("SIDETRACKED_SECRET").context("SIDETRACKED_SECRET not set")?;

        let authorizer = AuthorizerBuilder::<Claims>::from_secret(&secret)
            .jwt_source(JwtSource::AuthorizationHeader)
            .build()
            .await?;

        Ok(Self::new(authorizer))
    }
    // >@

    /// Create the application router
    pub fn router(&self) -> Router {
        // @<applicationrouter
        let protected = Router::new()
            // Add a profile route
            .route("/profile", get(profile))
            // Add the authorizer layer
            .layer(ServiceBuilder::new().layer(self.authorizer.clone().into_layer()));

        let unprotected = Router::new()
            // Add a health check route
            .route("/health_check", get(health_check));

        Router::new()
            .merge(protected)
            .merge(unprotected)
            // Add `TraceLayer` to log all incoming requests
            .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        // >@
    }
}
// >@
