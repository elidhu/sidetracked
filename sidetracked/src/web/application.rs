use axum::{routing::get, Router};
use std::net::{IpAddr, SocketAddr};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing::info;

// @<structapplication
pub struct Application {
    /// The host to listen on
    pub host: IpAddr,
    /// The port to listen on
    pub port: u16,
}
// >@

// @<impldefaultapplication
impl Default for Application {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".parse().unwrap(),
            port: 3000,
        }
    }
}
// >@

// @<implapplication
impl Application {
    /// Create a new application
    pub fn new(host: IpAddr, port: u16) -> Self {
        Self { host, port }
    }

    /// Create the application router
    pub async fn router(&self) -> Router {
        Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
    }

    /// Run the application
    pub async fn run(&self) {
        let addr = SocketAddr::new(self.host, self.port);
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

        let router = self.router().await;

        info!("Listening on {}", listener.local_addr().unwrap());
        axum::serve(listener, router.into_make_service())
            .await
            .expect("Unexpected error during server execution");
    }
}
// >@
