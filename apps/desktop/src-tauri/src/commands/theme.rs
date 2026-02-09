use shipkit_core::{SettingsBackend, ThemeDefinition};
use tauri::State;

use crate::state::AppState;

#[tauri::command]
pub fn get_theme(state: State<'_, AppState>) -> Result<ThemeDefinition, String> {
    let engine = state.theme_engine.read().map_err(|e| e.to_string())?;
    Ok(engine.active().clone())
}

#[tauri::command]
pub fn set_theme(state: State<'_, AppState>, name: String) -> Result<ThemeDefinition, String> {
    let mut engine = state.theme_engine.write().map_err(|e| e.to_string())?;
    let theme = engine.set_active(&name).map_err(|e| e.to_string())?;
    let result = theme.clone();

    // Persist theme selection
    state
        .settings_store
        .set(
            "shipkit_internal",
            "active_theme",
            serde_json::json!(name),
        )
        .map_err(|e| e.to_string())?;

    Ok(result)
}

#[tauri::command]
pub fn list_themes(state: State<'_, AppState>) -> Result<Vec<ThemeDefinition>, String> {
    let engine = state.theme_engine.read().map_err(|e| e.to_string())?;
    Ok(engine.list().to_vec())
}

#[tauri::command]
pub fn get_css_variables(state: State<'_, AppState>) -> Result<String, String> {
    let engine = state.theme_engine.read().map_err(|e| e.to_string())?;
    Ok(engine.generate_css())
}
