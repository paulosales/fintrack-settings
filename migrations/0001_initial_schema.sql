-- Application settings table (schema: fintrak_settings)
CREATE TABLE IF NOT EXISTS settings (
    id          BIGINT       NOT NULL AUTO_INCREMENT,
    code        VARCHAR(100) NOT NULL,
    description VARCHAR(200) NOT NULL,
    value       VARCHAR(300) DEFAULT NULL,
    PRIMARY KEY (id),
    UNIQUE KEY settings_code_idx (code) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

-- Seed the default display currency setting
INSERT IGNORE INTO settings (code, description, value)
VALUES ('current_currency', 'The default display currency for all monetary values', 'USD');
