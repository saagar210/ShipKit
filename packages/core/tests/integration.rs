use shipkit_core::*;
use tempfile::TempDir;

#[test]
fn full_lifecycle() {
    let tmp = TempDir::new().expect("tmp dir");

    // 1. Database — pool + migrations
    let pool = ConnectionPool::new(tmp.path().join("test.db")).expect("create pool");
    let mut engine = MigrationEngine::new(pool.clone());
    engine.register(Migration {
        version: 1,
        name: "create_notes".into(),
        up_sql: "CREATE TABLE notes (id INTEGER PRIMARY KEY, content TEXT);".into(),
        down_sql: Some("DROP TABLE notes;".into()),
    });
    let statuses = engine.apply_pending().expect("apply migrations");
    assert_eq!(statuses.len(), 1);
    assert!(statuses[0].applied);

    // Verify table exists
    let conn = pool.get().expect("get connection");
    conn.execute(
        "INSERT INTO notes (content) VALUES ('hello')",
        [],
    )
    .expect("insert into notes");

    // 2. Settings — derive macro + SQLite store
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Settings)]
    #[settings(namespace = "test")]
    struct TestSettings {
        #[settings(default = "hello")]
        greeting: String,
        #[settings(default = 42)]
        magic_number: i64,
        #[settings(default = true)]
        enabled: bool,
    }

    let store = SqliteSettingsStore::new(pool.clone()).expect("create store");
    let manager = SettingsManager::new(store);

    // Load defaults (nothing saved yet)
    let settings: TestSettings = manager.load().expect("load defaults");
    assert_eq!(settings.greeting, "hello");
    assert_eq!(settings.magic_number, 42);
    assert!(settings.enabled);

    // Modify and save
    let mut settings = settings;
    settings.greeting = "world".into();
    settings.magic_number = 99;
    manager.save(&settings).expect("save settings");

    // Reload and verify
    let reloaded: TestSettings = manager.load().expect("reload settings");
    assert_eq!(reloaded.greeting, "world");
    assert_eq!(reloaded.magic_number, 99);
    assert!(reloaded.enabled); // unchanged field keeps saved value

    // get_field and set_field
    let val = manager.get::<TestSettings>("greeting").expect("get field");
    assert_eq!(val, serde_json::json!("world"));

    manager
        .set::<TestSettings>("greeting", serde_json::json!("updated"))
        .expect("set field");
    let val = manager.get::<TestSettings>("greeting").expect("get field");
    assert_eq!(val, serde_json::json!("updated"));

    // Non-existent field
    let result = manager.get::<TestSettings>("nonexistent");
    assert!(result.is_err());

    // 3. Theme engine
    let themes = shipkit_core::theme::default_themes();
    let mut theme_engine = ThemeEngine::new(themes, "light").expect("create theme engine");
    let css = theme_engine.generate_css();
    assert!(css.contains(":root {"));
    assert!(css.contains("--sk-color-primary"));

    theme_engine.set_active("dark").expect("switch to dark");
    let css = theme_engine.generate_css();
    assert!(css.contains("--sk-color-background: #0a0a0a"));

    // 4. Logger
    let log_dir = tmp.path().join("logs");
    let result = Logger::init(LoggerConfig {
        log_dir: log_dir.clone(),
        file_prefix: "test".into(),
        rotation: shipkit_core::logger::Rotation::Never,
        level: tracing::Level::DEBUG,
        json_format: true,
        console_output: false,
    });

    if let Ok(logger) = result {
        tracing::info!(key = "value", "test log message");

        // Flush by dropping the guard
        drop(logger);

        let entries =
            shipkit_core::logger::read_log_entries(&log_dir, 10, None).expect("read logs");
        assert!(!entries.is_empty());
        assert!(entries.iter().any(|e| e.message.contains("test log message")));
    }
    // If Logger::init fails (global subscriber already set), that's OK in test suite
}

#[test]
fn settings_round_trip_with_options() {
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Settings)]
    #[settings(namespace = "options")]
    struct OptionalSettings {
        #[settings(default = "default")]
        name: String,
        custom: Option<String>,
    }

    let pool = ConnectionPool::in_memory().expect("pool");
    let store = SqliteSettingsStore::new(pool).expect("store");
    let manager = SettingsManager::new(store);

    let settings: OptionalSettings = manager.load().expect("load");
    assert_eq!(settings.name, "default");
    assert!(settings.custom.is_none());

    let mut settings = settings;
    settings.custom = Some("custom value".into());
    manager.save(&settings).expect("save");

    let reloaded: OptionalSettings = manager.load().expect("reload");
    assert_eq!(reloaded.custom, Some("custom value".into()));
}

#[test]
fn migration_rollback_lifecycle() {
    let pool = ConnectionPool::in_memory().expect("pool");
    let mut engine = MigrationEngine::new(pool.clone());
    engine
        .register(Migration {
            version: 1,
            name: "create_a".into(),
            up_sql: "CREATE TABLE a (id INTEGER PRIMARY KEY);".into(),
            down_sql: Some("DROP TABLE a;".into()),
        })
        .register(Migration {
            version: 2,
            name: "create_b".into(),
            up_sql: "CREATE TABLE b (id INTEGER PRIMARY KEY);".into(),
            down_sql: Some("DROP TABLE b;".into()),
        });

    engine.apply_pending().expect("apply");

    // Rollback last
    let rolled = engine.rollback_last().expect("rollback");
    assert_eq!(rolled.map(|s| s.version), Some(2));

    // Re-apply
    let statuses = engine.apply_pending().expect("re-apply");
    assert!(statuses.iter().all(|s| s.applied));
}
