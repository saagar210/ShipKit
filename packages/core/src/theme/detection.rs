//! Platform-specific system theme detection.

use super::engine::ThemeMode;

/// Detect the current system theme preference.
pub fn detect_system_theme() -> ThemeMode {
    #[cfg(target_os = "macos")]
    {
        match std::process::Command::new("defaults")
            .args(["read", "-g", "AppleInterfaceStyle"])
            .output()
        {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if stdout.trim().eq_ignore_ascii_case("dark") {
                    ThemeMode::Dark
                } else {
                    ThemeMode::Light
                }
            }
            Err(_) => ThemeMode::Light,
        }
    }

    #[cfg(target_os = "windows")]
    {
        // Registry: HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize
        // Key: AppsUseLightTheme (0 = dark, 1 = light)
        ThemeMode::Light
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        ThemeMode::Light
    }
}
