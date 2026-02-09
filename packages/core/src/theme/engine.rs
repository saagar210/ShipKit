//! Theme engine for managing CSS variable themes.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::error::{Result, ShipKitError};

/// Whether a theme is light, dark, or follows system preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

/// A complete theme definition with CSS variables.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeDefinition {
    pub name: String,
    pub mode: ThemeMode,
    /// CSS variables as key-value pairs. BTreeMap for deterministic ordering.
    pub variables: BTreeMap<String, String>,
}

/// Manages registered themes and tracks the active selection.
pub struct ThemeEngine {
    themes: Vec<ThemeDefinition>,
    active: String,
}

impl ThemeEngine {
    /// Create a new engine. `default` must be the name of a theme in `themes`.
    pub fn new(themes: Vec<ThemeDefinition>, default: &str) -> Result<Self> {
        if !themes.iter().any(|t| t.name == default) {
            return Err(ShipKitError::ThemeNotFound(default.to_string()));
        }
        Ok(Self {
            themes,
            active: default.to_string(),
        })
    }

    /// Get the currently active theme.
    ///
    /// # Panics
    /// Only if the internal state is corrupted (active name not in themes list).
    /// This cannot happen through the public API since `new()` and `set_active()`
    /// both validate the name.
    #[allow(clippy::expect_used)]
    pub fn active(&self) -> &ThemeDefinition {
        self.themes
            .iter()
            .find(|t| t.name == self.active)
            .expect("active theme must exist in themes list — validated on construction")
    }

    /// Switch to a different theme by name.
    pub fn set_active(&mut self, name: &str) -> Result<&ThemeDefinition> {
        if !self.themes.iter().any(|t| t.name == name) {
            return Err(ShipKitError::ThemeNotFound(name.to_string()));
        }
        self.active = name.to_string();
        Ok(self.active())
    }

    /// List all registered themes.
    pub fn list(&self) -> &[ThemeDefinition] {
        &self.themes
    }

    /// Generate a CSS `:root` block with the active theme's variables.
    pub fn generate_css(&self) -> String {
        let theme = self.active();
        let mut css = String::from(":root {\n");
        for (key, value) in &theme.variables {
            css.push_str(&format!("  {key}: {value};\n"));
        }
        css.push('}');
        css
    }

    /// Detect the system theme preference.
    pub fn resolve_system_mode() -> ThemeMode {
        super::detection::detect_system_theme()
    }
}

/// Built-in light and dark themes.
pub fn default_themes() -> Vec<ThemeDefinition> {
    vec![
        ThemeDefinition {
            name: "light".to_string(),
            mode: ThemeMode::Light,
            variables: BTreeMap::from([
                ("--sk-color-background".into(), "#ffffff".into()),
                ("--sk-color-border".into(), "#e5e5e5".into()),
                ("--sk-color-destructive".into(), "#ef4444".into()),
                ("--sk-color-foreground".into(), "#0a0a0a".into()),
                ("--sk-color-muted".into(), "#f5f5f5".into()),
                ("--sk-color-muted-foreground".into(), "#737373".into()),
                ("--sk-color-primary".into(), "#3b82f6".into()),
                ("--sk-color-primary-foreground".into(), "#ffffff".into()),
            ]),
        },
        ThemeDefinition {
            name: "dark".to_string(),
            mode: ThemeMode::Dark,
            variables: BTreeMap::from([
                ("--sk-color-background".into(), "#0a0a0a".into()),
                ("--sk-color-border".into(), "#262626".into()),
                ("--sk-color-destructive".into(), "#ef4444".into()),
                ("--sk-color-foreground".into(), "#fafafa".into()),
                ("--sk-color-muted".into(), "#262626".into()),
                ("--sk-color-muted-foreground".into(), "#a3a3a3".into()),
                ("--sk-color-primary".into(), "#3b82f6".into()),
                ("--sk-color-primary-foreground".into(), "#ffffff".into()),
            ]),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_with_defaults() {
        let themes = default_themes();
        let engine = ThemeEngine::new(themes, "light").expect("create engine");
        assert_eq!(engine.list().len(), 2);
        assert_eq!(engine.active().name, "light");
    }

    #[test]
    fn switch_theme() {
        let themes = default_themes();
        let mut engine = ThemeEngine::new(themes, "light").expect("create engine");
        let dark = engine.set_active("dark").expect("switch to dark");
        assert_eq!(dark.name, "dark");
        assert_eq!(engine.active().name, "dark");
    }

    #[test]
    fn invalid_theme_name() {
        let themes = default_themes();
        let mut engine = ThemeEngine::new(themes, "light").expect("create engine");
        let result = engine.set_active("nonexistent");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("nonexistent"));
    }

    #[test]
    fn css_generation() {
        let themes = default_themes();
        let engine = ThemeEngine::new(themes, "light").expect("create engine");
        let css = engine.generate_css();
        assert!(css.starts_with(":root {"));
        assert!(css.contains("--sk-color-primary"));
        assert!(css.ends_with('}'));
    }

    #[test]
    fn css_alphabetical_order() {
        let themes = default_themes();
        let engine = ThemeEngine::new(themes, "light").expect("create engine");
        let css = engine.generate_css();
        // BTreeMap guarantees alphabetical order
        let bg_pos = css.find("--sk-color-background").expect("bg");
        let fg_pos = css.find("--sk-color-foreground").expect("fg");
        assert!(bg_pos < fg_pos, "variables should be in alphabetical order");
    }

    #[test]
    fn system_detection_runs() {
        // Just verify it doesn't panic
        let _mode = ThemeEngine::resolve_system_mode();
    }

    #[test]
    fn dark_theme_css_values() {
        let themes = default_themes();
        let mut engine = ThemeEngine::new(themes, "light").expect("create engine");
        engine.set_active("dark").expect("switch");
        let css = engine.generate_css();
        assert!(css.contains("--sk-color-background: #0a0a0a"));
    }
}
