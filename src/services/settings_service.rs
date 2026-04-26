use crate::models::settings::{Setting, SettingUpsert};
use anyhow::{anyhow, bail};
use sqlx::MySqlPool;

async fn get_setting_by_code(pool: &MySqlPool, code: &str) -> Result<Setting, anyhow::Error> {
    sqlx::query_as::<_, Setting>("SELECT id, code, description, value FROM settings WHERE code = ?")
        .bind(code)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| anyhow!("Setting '{}' not found", code))
}

pub async fn list_settings(pool: &MySqlPool) -> Result<Vec<Setting>, anyhow::Error> {
    let settings = sqlx::query_as::<_, Setting>(
        "SELECT id, code, description, value FROM settings ORDER BY code ASC",
    )
    .fetch_all(pool)
    .await?;
    Ok(settings)
}

pub async fn get_setting(pool: &MySqlPool, code: &str) -> Result<Setting, anyhow::Error> {
    get_setting_by_code(pool, code).await
}

pub async fn create_setting(
    pool: &MySqlPool,
    payload: SettingUpsert,
) -> Result<Setting, anyhow::Error> {
    sqlx::query("INSERT INTO settings (code, description, value) VALUES (?, ?, ?)")
        .bind(&payload.code)
        .bind(&payload.description)
        .bind(&payload.value)
        .execute(pool)
        .await?;
    get_setting_by_code(pool, &payload.code).await
}

pub async fn update_setting(
    pool: &MySqlPool,
    code: &str,
    payload: SettingUpsert,
) -> Result<Setting, anyhow::Error> {
    let result = sqlx::query("UPDATE settings SET description = ?, value = ? WHERE code = ?")
        .bind(&payload.description)
        .bind(&payload.value)
        .bind(code)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        bail!("Setting '{}' not found", code);
    }
    get_setting_by_code(pool, code).await
}

pub async fn delete_setting(pool: &MySqlPool, code: &str) -> Result<(), anyhow::Error> {
    let result = sqlx::query("DELETE FROM settings WHERE code = ?")
        .bind(code)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        bail!("Setting '{}' not found", code);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_upsert(code: &str, desc: &str, value: Option<&str>) -> SettingUpsert {
        SettingUpsert {
            code: code.to_string(),
            description: desc.to_string(),
            value: value.map(|s| s.to_string()),
        }
    }

    #[test]
    fn test_setting_upsert_construction() {
        let u = make_upsert("key", "A key", Some("val"));
        assert_eq!(u.code, "key");
        assert_eq!(u.value, Some("val".to_string()));
    }

    #[test]
    fn test_setting_upsert_no_value() {
        let u = make_upsert("key", "A key", None);
        assert!(u.value.is_none());
    }
}
