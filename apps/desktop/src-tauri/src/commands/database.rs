use shipkit_core::MigrationStatus;
use tauri::State;

use crate::state::AppState;

#[tauri::command]
pub fn migration_status(state: State<'_, AppState>) -> Result<Vec<MigrationStatus>, String> {
    let engine = state.migrations.lock().map_err(|e| e.to_string())?;
    engine.status().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn apply_migrations(state: State<'_, AppState>) -> Result<Vec<MigrationStatus>, String> {
    let mut engine = state.migrations.lock().map_err(|e| e.to_string())?;
    engine.apply_pending().map_err(|e| e.to_string())
}

#[tauri::command]
pub fn rollback_migration(
    state: State<'_, AppState>,
) -> Result<Option<MigrationStatus>, String> {
    let mut engine = state.migrations.lock().map_err(|e| e.to_string())?;
    engine.rollback_last().map_err(|e| e.to_string())
}
