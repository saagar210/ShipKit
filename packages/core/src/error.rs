//! Error types for shipkit-core.

use thiserror::Error;

/// The main error type for all shipkit-core operations.
#[derive(Error, Debug)]
pub enum ShipKitError {
    #[error("database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("connection pool error: {0}")]
    Pool(#[from] r2d2::Error),

    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("migration failed: {0}")]
    Migration(String),

    #[error("setting not found: {namespace}.{key}")]
    SettingNotFound { namespace: String, key: String },

    #[error("invalid setting value for {key}: {reason}")]
    InvalidSetting { key: String, reason: String },

    #[error("theme not found: {0}")]
    ThemeNotFound(String),

    #[error("logger already initialized")]
    LoggerAlreadyInitialized,

    #[error("{0}")]
    Other(String),
}

/// Convenience alias for `Result<T, ShipKitError>`.
pub type Result<T> = std::result::Result<T, ShipKitError>;

#[cfg(feature = "tauri")]
impl serde::Serialize for ShipKitError {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
