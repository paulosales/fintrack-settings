use axum::extract::FromRef;
use jsonwebtoken::DecodingKey;
use sqlx::MySqlPool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type JwksCache = Arc<RwLock<HashMap<String, DecodingKey>>>;

pub fn new_jwks_cache() -> JwksCache {
    Arc::new(RwLock::new(HashMap::new()))
}

#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
    pub keycloak_realm_url: String,
    pub http_client: reqwest::Client,
    pub jwks_cache: JwksCache,
}

impl FromRef<AppState> for MySqlPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}
