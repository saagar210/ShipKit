# shipkit-core

Production-ready building blocks for Tauri 2 desktop applications.

## Installation

```toml
[dependencies]
shipkit-core = "0.1"
```

## Quick Start

```rust
use shipkit_core::*;

// 1. Database
let pool = ConnectionPool::new("data.db")?;
let mut engine = MigrationEngine::new(pool.clone());
engine.register(Migration {
    version: 1,
    name: "create_users".into(),
    up_sql: "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);".into(),
    down_sql: Some("DROP TABLE users;".into()),
});
engine.apply_pending()?;

// 2. Settings
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Settings)]
#[settings(namespace = "app")]
struct AppSettings {
    #[settings(default = "dark")]
    theme: String,
    #[settings(default = 14)]
    font_size: u32,
}

let store = SqliteSettingsStore::new(pool)?;
let manager = SettingsManager::new(store);
let settings: AppSettings = manager.load()?;
```

## Modules

### Database (`db`)

SQLite connection pool with WAL mode and a migration engine with checksums.

```rust
let pool = ConnectionPool::new("app.db")?;
let mut engine = MigrationEngine::new(pool);
engine.register_from_dir("./migrations")?;
engine.apply_pending()?;
```

### Settings (`settings`)

Type-safe settings with `#[derive(Settings)]` and SQLite persistence.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Settings)]
#[settings(namespace = "appearance")]
struct Appearance {
    #[settings(default = "system")]
    theme: String,
    #[settings(default = 1.0)]
    font_scale: f64,
    custom_css: Option<String>,
}
```

### Theme (`theme`)

CSS variable theme engine with system theme detection.

```rust
let themes = shipkit_core::theme::default_themes();
let mut engine = ThemeEngine::new(themes, "light")?;
let css = engine.generate_css(); // :root { --sk-color-primary: #3b82f6; ... }
engine.set_active("dark")?;
```

### Logger (`logger`)

Structured JSON logging with file rotation.

```rust
let logger = Logger::init(LoggerConfig {
    log_dir: "./logs".into(),
    file_prefix: "myapp".into(),
    ..LoggerConfig::default()
})?;
tracing::info!("app started");
```

## Feature Flags

| Flag | Description |
|------|------------|
| `tauri` | Enables Tauri 2 commands and plugin types |

## License

MIT
