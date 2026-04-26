use dotenv::dotenv;
use sqlx::MySqlPool;
use std::env;

pub async fn get_pool() -> MySqlPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://user:password@localhost/fintrak_settings".to_string());

    MySqlPool::connect(&database_url)
        .await
        .expect("Failed to create pool")
}

pub async fn run_migrations(pool: &MySqlPool) {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .expect("Failed to run database migrations");
}

#[cfg(test)]
mod tests {
    use std::env;

    #[test]
    fn test_default_database_url() {
        env::remove_var("DATABASE_URL");
        let default_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "mysql://user:password@localhost/fintrak_settings".to_string());
        assert!(default_url.contains("mysql://"));
        assert!(default_url.contains("fintrak_settings"));
    }
}
