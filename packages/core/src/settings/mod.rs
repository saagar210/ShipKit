//! Type-safe settings management with SQLite persistence.

pub mod store;
pub mod traits;

pub use store::SqliteSettingsStore;
pub use traits::{Settings, SettingsBackend};

/// Convenience wrapper that combines a store with type-safe access.
pub struct SettingsManager {
    store: Box<dyn SettingsBackend>,
}

impl SettingsManager {
    /// Create a new manager wrapping the given backend.
    pub fn new(store: impl SettingsBackend + 'static) -> Self {
        Self {
            store: Box::new(store),
        }
    }

    /// Load settings of type `S`, filling missing fields with defaults.
    pub fn load<S: Settings>(&self) -> crate::error::Result<S> {
        S::load(self.store.as_ref())
    }

    /// Save all fields of a settings struct.
    pub fn save<S: Settings>(&self, settings: &S) -> crate::error::Result<()> {
        settings.save(self.store.as_ref())
    }

    /// Get a single field's value.
    pub fn get<S: Settings>(&self, field: &str) -> crate::error::Result<serde_json::Value> {
        S::get_field(self.store.as_ref(), field)
    }

    /// Set a single field's value.
    pub fn set<S: Settings>(
        &self,
        field: &str,
        value: serde_json::Value,
    ) -> crate::error::Result<()> {
        S::set_field(self.store.as_ref(), field, value)
    }
}
