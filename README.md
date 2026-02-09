# ShipKit

> Production-ready building blocks for Tauri 2 desktop applications

ShipKit is a batteries-included Rust library and desktop app template that provides essential infrastructure for building Tauri applications. It eliminates boilerplate by providing database migrations, settings management, theme system, and structured logging out of the box.

## Status

**Phase 2 Complete** — All core modules integrated into a running Tauri 2 desktop app with full IPC coverage.

- ✅ **Phase 1:** Core library with 4 modules (42 tests, zero clippy warnings)
- ✅ **Phase 2:** Tauri integration with 13 IPC commands and React frontend demo

## Features

### Core Library (`shipkit-core`)

- **Database Module** — SQLite connection pool with WAL mode, migration engine with SHA256 checksums, file-based migrations with rollback support
- **Settings Module** — Type-safe settings with derive macro, SQLite backend, namespace isolation
- **Theme Module** — CSS variable themes with light/dark defaults, system theme detection (macOS), runtime theme switching
- **Logger Module** — Structured JSON logging with tracing, file rotation (daily/hourly/never), level filtering

### Desktop App (`shipkit-desktop`)

- **13 IPC Commands** — Full Tauri 2 integration exposing every core module API
- **React Demo UI** — 4 panels demonstrating migrations, settings CRUD, theme switching, and log viewing
- **Typed TypeScript Bindings** — Hand-written types matching Rust structs
- **Persistent State** — Theme preference survives app restarts

## Architecture

```
ShipKit/
├── packages/
│   ├── core/           # shipkit-core library
│   │   ├── db/         # ConnectionPool, MigrationEngine
│   │   ├── settings/   # Settings trait, SqliteSettingsStore
│   │   ├── theme/      # ThemeEngine, default themes
│   │   └── logger/     # Logger, read_log_entries
│   └── macros/         # #[derive(Settings)]
└── apps/
    └── desktop/        # Tauri 2 app
        ├── src/        # React 19 + TypeScript frontend
        └── src-tauri/  # Rust backend with IPC commands
```

## Quick Start

### Prerequisites

- Rust 1.84+ (edition 2024)
- Node.js 18+ with pnpm
- macOS (Linux/Windows support planned)

### Run the Desktop App

```bash
# Install dependencies
pnpm install

# Run in dev mode
cd apps/desktop
pnpm tauri dev
```

### Use the Core Library

```toml
[dependencies]
shipkit-core = { git = "https://github.com/YOUR_USERNAME/ShipKit" }
```

```rust
use shipkit_core::{ConnectionPool, LoggerConfig, Logger};

let pool = ConnectionPool::new("app.db")?;
let logger = Logger::init(LoggerConfig::default())?;
```

## Development

### Run Tests

```bash
# Core library + macros (42 tests)
cargo test -p shipkit-core -p shipkit-macros

# Clippy (workspace-wide, zero warnings required)
cargo clippy --workspace -- -D warnings
```

### Build Frontend

```bash
cd apps/desktop
pnpm install
pnpm build  # TypeScript + Vite bundle
```

## API Examples

### Migrations

```rust
use shipkit_core::{MigrationEngine, Migration};

let mut engine = MigrationEngine::new(pool);
engine.register(Migration {
    version: 1,
    name: "create_users".into(),
    up_sql: "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);".into(),
    down_sql: Some("DROP TABLE users;".into()),
});

engine.apply_pending()?;  // Run all pending migrations
```

### Settings with Derive Macro

```rust
use shipkit_core::{Settings, SqliteSettingsStore};
use serde::{Serialize, Deserialize};

#[derive(Settings, Serialize, Deserialize)]
#[settings(namespace = "app")]
struct AppSettings {
    #[settings(default = "dark")]
    theme: String,

    #[settings(default = "14")]
    font_size: u32,
}

let store = SqliteSettingsStore::new(pool)?;
let settings = AppSettings::load(&store)?;
```

### Theme Switching

```rust
use shipkit_core::{ThemeEngine, theme::default_themes};

let mut engine = ThemeEngine::new(default_themes(), "dark")?;
let theme = engine.set_active("light")?;
let css = engine.generate_css();  // `:root { --sk-color-primary: #3b82f6; ... }`
```

### Structured Logging

```rust
use shipkit_core::{Logger, LoggerConfig};
use tracing::info;

let logger = Logger::init(LoggerConfig {
    log_dir: "logs".into(),
    file_prefix: "app".into(),
    json_format: true,
    ..Default::default()
})?;

info!(user_id = 42, "User logged in");
```

## Roadmap

- **Phase 3:** Plugin system for extensibility
- **Phase 4:** Auto-update mechanism with signature verification
- **Phase 5:** Cross-platform testing (Linux, Windows)

## Technical Details

- **Rust Edition:** 2024
- **Workspace Lints:** `unwrap_used = "deny"`, `expect_used = "warn"`
- **Concurrency:** Mutex for mut operations, RwLock for read/write split
- **Database:** SQLite with WAL mode, r2d2 connection pooling
- **Frontend:** React 19, Vite 6, TypeScript 5 (strict mode)
- **IPC Pattern:** App-level commands (not plugin), `Result<T, String>` error handling

## License

MIT

## Contributing

Contributions welcome! This is a early-stage project under active development.

1. Fork the repo
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Run tests (`cargo test -p shipkit-core -p shipkit-macros`)
4. Ensure clippy passes (`cargo clippy --workspace -- -D warnings`)
5. Commit (`git commit -m 'feat: add amazing feature'`)
6. Push and open a PR

---

Built with [Tauri 2](https://tauri.app) and [Rust](https://rust-lang.org).
