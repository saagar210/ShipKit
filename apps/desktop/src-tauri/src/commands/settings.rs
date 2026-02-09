use std::collections::HashMap;

use serde_json::Value;
use shipkit_core::SettingsBackend;
use tauri::State;

use crate::state::AppState;

#[tauri::command]
pub fn get_setting(
    state: State<'_, AppState>,
    namespace: String,
    key: String,
) -> Result<Option<Value>, String> {
    state
        .settings_store
        .get(&namespace, &key)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_setting(
    state: State<'_, AppState>,
    namespace: String,
    key: String,
    value: Value,
) -> Result<(), String> {
    state
        .settings_store
        .set(&namespace, &key, value)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_all_settings(
    state: State<'_, AppState>,
    namespace: String,
) -> Result<HashMap<String, Value>, String> {
    state
        .settings_store
        .get_all(&namespace)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn load_settings(
    state: State<'_, AppState>,
    namespace: String,
) -> Result<Value, String> {
    let all = state
        .settings_store
        .get_all(&namespace)
        .map_err(|e| e.to_string())?;
    Ok(Value::Object(all.into_iter().collect()))
}

#[tauri::command]
pub fn save_settings(
    state: State<'_, AppState>,
    namespace: String,
    settings: Value,
) -> Result<(), String> {
    if let Value::Object(map) = settings {
        for (key, val) in map {
            state
                .settings_store
                .set(&namespace, &key, val)
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    } else {
        Err("settings must be a JSON object".into())
    }
}
