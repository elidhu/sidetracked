use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// @<mainuselib
use sidetracked_lib::web::application::{run, Application, ApplicationConfig};
// >@

// @<main
#[tokio::main]
async fn main() {
    init_logging();

    let config = ApplicationConfig::default();

    let app = Application::new_with_default_authorizer()
        .await
        .expect("Failed to build application");

    run(app, config).await;
}
// >@

// @<initlogging
fn init_logging() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                let crate_name = env!("CARGO_CRATE_NAME");
                format!("{crate_name}=debug,tower_http=debug,axum::rejection=trace").into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
// >@
