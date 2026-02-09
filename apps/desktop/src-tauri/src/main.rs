#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod state;

use std::sync::{Mutex, RwLock};

use shipkit_core::theme::default_themes;
use shipkit_core::{
    ConnectionPool, LoggerConfig, Migration, MigrationEngine, SettingsBackend,
    SqliteSettingsStore, ThemeEngine,
};

#[allow(clippy::expect_used)]
fn main() {
    let data_dir = dirs::data_local_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("shipkit-desktop");

    std::fs::create_dir_all(&data_dir).expect("failed to create data directory");

    // 1. Logger — initialized first to capture everything after
    let logger = shipkit_core::Logger::init(LoggerConfig {
        log_dir: data_dir.join("logs"),
        file_prefix: "shipkit".into(),
        console_output: true,
        ..LoggerConfig::default()
    })
    .expect("failed to initialize logger");

    tracing::info!("ShipKit Desktop starting up");

    // 2. Database pool
    let pool =
        ConnectionPool::new(data_dir.join("data.db")).expect("failed to create connection pool");

    // 3. Settings store
    let settings_store =
        SqliteSettingsStore::new(pool.clone()).expect("failed to create settings store");

    // 4. Migration engine with a demo migration
    let mut migration_engine = MigrationEngine::new(pool.clone());
    migration_engine.register(Migration {
        version: 1,
        name: "create_notes".into(),
        up_sql: "CREATE TABLE IF NOT EXISTS notes (id INTEGER PRIMARY KEY, title TEXT NOT NULL, content TEXT, created_at TEXT DEFAULT (datetime('now')));".into(),
        down_sql: Some("DROP TABLE IF EXISTS notes;".into()),
    });

    // 5. Theme engine — restore persisted theme preference
    let themes = default_themes();
    let active_theme = settings_store
        .get("shipkit_internal", "active_theme")
        .ok()
        .flatten()
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_else(|| "dark".to_string());

    let theme_engine = ThemeEngine::new(themes.clone(), &active_theme).unwrap_or_else(|_| {
        // Stored theme name no longer valid — fall back to dark
        ThemeEngine::new(themes, "dark").expect("default themes must include 'dark'")
    });

    tracing::info!(theme = %active_theme, "theme engine initialized");

    let app_state = state::AppState {
        _pool: pool,
        migrations: Mutex::new(migration_engine),
        settings_store,
        theme_engine: RwLock::new(theme_engine),
        logger,
    };

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            commands::database::migration_status,
            commands::database::apply_migrations,
            commands::database::rollback_migration,
            commands::settings::get_setting,
            commands::settings::set_setting,
            commands::settings::get_all_settings,
            commands::settings::load_settings,
            commands::settings::save_settings,
            commands::theme::get_theme,
            commands::theme::set_theme,
            commands::theme::list_themes,
            commands::theme::get_css_variables,
            commands::logger::get_log_entries,
        ])
        .run(tauri::generate_context!())
        .expect("error running tauri application");
}
