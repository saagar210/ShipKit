import { invoke as tauriInvoke } from "@tauri-apps/api/core";
import type { MigrationStatus, ThemeDefinition, LogEntry } from "./bindings";

// Database
export const migrationStatus = () =>
  tauriInvoke<MigrationStatus[]>("migration_status");

export const applyMigrations = () =>
  tauriInvoke<MigrationStatus[]>("apply_migrations");

export const rollbackMigration = () =>
  tauriInvoke<MigrationStatus | null>("rollback_migration");

// Settings
export const getSetting = (namespace: string, key: string) =>
  tauriInvoke<unknown | null>("get_setting", { namespace, key });

export const setSetting = (namespace: string, key: string, value: unknown) =>
  tauriInvoke<void>("set_setting", { namespace, key, value });

export const getAllSettings = (namespace: string) =>
  tauriInvoke<Record<string, unknown>>("get_all_settings", { namespace });

export const loadSettings = (namespace: string) =>
  tauriInvoke<Record<string, unknown>>("load_settings", { namespace });

export const saveSettings = (
  namespace: string,
  settings: Record<string, unknown>,
) => tauriInvoke<void>("save_settings", { namespace, settings });

// Theme
export const getTheme = () => tauriInvoke<ThemeDefinition>("get_theme");

export const setTheme = (name: string) =>
  tauriInvoke<ThemeDefinition>("set_theme", { name });

export const listThemes = () =>
  tauriInvoke<ThemeDefinition[]>("list_themes");

export const getCssVariables = () =>
  tauriInvoke<string>("get_css_variables");

// Logger
export const getLogEntries = (count?: number, level?: string) =>
  tauriInvoke<LogEntry[]>("get_log_entries", { count, level });
