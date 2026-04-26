use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Setting {
    pub id: i64,
    pub code: String,
    pub description: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct SettingUpsert {
    pub code: String,
    pub description: String,
    pub value: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setting_serialization() {
        let setting = Setting {
            id: 1,
            code: "current_currency".to_string(),
            description: "Display currency".to_string(),
            value: Some("USD".to_string()),
        };
        let json = serde_json::to_string(&setting).unwrap();
        assert!(json.contains("currentCurrency") || json.contains("current_currency"));
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["code"], "current_currency");
        assert_eq!(v["value"], "USD");
    }

    #[test]
    fn test_setting_null_value() {
        let setting = Setting {
            id: 2,
            code: "some_key".to_string(),
            description: "Some description".to_string(),
            value: None,
        };
        let json = serde_json::to_string(&setting).unwrap();
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert!(v["value"].is_null());
    }
}
