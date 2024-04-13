use axum::{routing::get, Router};
use std::net::{IpAddr, SocketAddr};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;

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
pub struct Application;
// >@

// @<implapplication
impl Application {
    /// Create the application router
    pub fn router(&self) -> Router {
        // @<applicationrouter
        Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
        // >@
    }
}
// >@
