// Matches shipkit_core::MigrationStatus
export interface MigrationStatus {
  version: number;
  name: string;
  applied: boolean;
  applied_at: string | null;
}

// Matches shipkit_core::ThemeMode
export type ThemeMode = "light" | "dark" | "system";

// Matches shipkit_core::ThemeDefinition
export interface ThemeDefinition {
  name: string;
  mode: ThemeMode;
  variables: Record<string, string>;
}

// Matches shipkit_core::logger::LogEntry
export interface LogEntry {
  timestamp: string;
  level: string;
  message: string;
  target: string;
  fields: unknown;
}
