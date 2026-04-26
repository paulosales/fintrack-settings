# settings-service

A Rust/Axum microservice that stores and serves application-wide settings for the Fintrack platform.

---

## Architecture

```
Browser/FE or internal services
        │
        ▼
 settings-service  (Rust / Axum)
  - GET /settings           → list all settings (public)
  - GET /settings/{code}    → get one setting (public)
  - POST /settings          → create setting (auth required)
  - PUT /settings/{code}    → update setting (auth required)
  - DELETE /settings/{code} → delete setting (auth required)
        │
        ▼
   MySQL (fintrak_settings schema)
```

Read endpoints are unauthenticated to allow internal service-to-service calls over the Docker network without JWT overhead.

---

## Default settings

| Code               | Default | Description                                          |
|--------------------|---------|------------------------------------------------------|
| `current_currency` | `USD`   | The default display currency for all monetary values |

---

## REST API

### `GET /settings`
Returns all settings.

### `GET /settings/{code}`
Returns a single setting by code. Returns `404` if not found.

### `POST /settings`
Creates a new setting. Requires `Authorization: Bearer <token>`.

**Body**
```json
{ "code": "my_key", "description": "My description", "value": "my_value" }
```

### `PUT /settings/{code}`
Updates an existing setting. Requires auth.

### `DELETE /settings/{code}`
Deletes a setting. Requires auth.

### `GET /health`
Returns `{"status": "ok"}`.

---

## Configuration

| Variable             | Default                              | Description            |
|----------------------|--------------------------------------|------------------------|
| `DATABASE_URL`       | `mysql://user:password@localhost/fintrak_settings` | MySQL DSN |
| `KEYCLOAK_REALM_URL` | `http://keycloak:8080/realms/fintrack` | Keycloak realm URL   |

---

## Development

### Prerequisites
- Rust 1.85+
- MySQL 8.0+

### Run locally
```bash
cd settings-service
DATABASE_URL=mysql://user:password@localhost/fintrak_settings \
KEYCLOAK_REALM_URL=http://localhost:8081/realms/fintrack \
cargo run
```

### Run tests
```bash
cargo test
```

### Lint & format
```bash
cargo clippy -- -D warnings
cargo fmt --check
```

---

## Docker

```bash
# Production
docker build -t settings-service .

# Development (hot-reload)
docker build -f Dockerfile.dev -t settings-service:dev .
```

---

## Project structure

```
settings-service/
├── Cargo.toml
├── Dockerfile
├── Dockerfile.dev
├── clippy.toml
├── rustfmt.toml
├── migrations/
│   └── 0001_initial_schema.sql
├── src/
│   ├── main.rs
│   ├── app_state.rs
│   ├── db/
│   │   ├── mod.rs
│   │   └── connection.rs
│   ├── models/
│   │   ├── mod.rs
│   │   └── settings.rs
│   ├── controllers/
│   │   ├── mod.rs
│   │   └── settings_controller.rs
│   ├── services/
│   │   ├── mod.rs
│   │   └── settings_service.rs
│   ├── routes/
│   │   ├── mod.rs
│   │   └── settings_routes.rs
│   └── middleware/
│       ├── mod.rs
│       └── auth_middleware.rs
└── tests/
    └── integration_tests.rs
```
