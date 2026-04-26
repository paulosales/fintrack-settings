use axum::middleware as mw;
use axum::Router;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

mod app_state;
mod controllers;
mod db;
mod middleware;
mod models;
mod routes;
mod services;

use app_state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("settings_service=info".parse().unwrap()),
        )
        .init();

    let pool = db::get_pool().await;
    db::run_migrations(&pool).await;
    println!("Database migrations applied successfully");

    let keycloak_realm_url = std::env::var("KEYCLOAK_REALM_URL")
        .unwrap_or_else(|_| "http://keycloak:8080/realms/fintrack".to_string());

    let state = AppState {
        pool,
        keycloak_realm_url,
        http_client: reqwest::Client::new(),
        jwks_cache: app_state::new_jwks_cache(),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        // Public read endpoints (no auth — accessible by internal services)
        .merge(routes::settings_routes::public_routes())
        // Protected write endpoints (require Keycloak JWT)
        .merge(
            routes::settings_routes::protected_routes().route_layer(mw::from_fn_with_state(
                state.clone(),
                middleware::auth_middleware::validate_bearer_token,
            )),
        )
        .route(
            "/health",
            axum::routing::get(|| async { axum::Json(serde_json::json!({"status": "ok"})) }),
        )
        .layer(cors)
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3004")
        .await
        .expect("Failed to bind");

    println!("Settings service running on http://0.0.0.0:3004");
    axum::serve(listener, app).await.expect("Server error");
}
