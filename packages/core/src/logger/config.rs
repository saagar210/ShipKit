//! Logger configuration.

/// Log file rotation strategy.
#[derive(Debug, Clone)]
pub enum Rotation {
    Daily,
    Hourly,
    Never,
}

/// Configuration for the structured logger.
#[derive(Debug, Clone)]
pub struct LoggerConfig {
    /// Directory where log files are written.
    pub log_dir: std::path::PathBuf,
    /// Prefix for log filenames (default: "shipkit").
    pub file_prefix: String,
    /// File rotation strategy.
    pub rotation: Rotation,
    /// Minimum log level.
    pub level: tracing::Level,
    /// Use JSON format for log files.
    pub json_format: bool,
    /// Also log to stderr.
    pub console_output: bool,
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            log_dir: dirs::data_local_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .join("shipkit")
                .join("logs"),
            file_prefix: "shipkit".into(),
            rotation: Rotation::Daily,
            level: tracing::Level::INFO,
            json_format: true,
            console_output: true,
        }
    }
}
