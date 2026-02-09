//! CSS variable theme engine with system theme detection.

pub mod detection;
pub mod engine;

pub use engine::{default_themes, ThemeDefinition, ThemeEngine, ThemeMode};
