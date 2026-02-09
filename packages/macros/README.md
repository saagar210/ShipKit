# shipkit-macros

Proc macros for [shipkit-core](../core/).

## `#[derive(Settings)]`

Generates the `Settings` trait implementation for a struct, enabling type-safe
settings with SQLite persistence.

### Usage

```rust
use shipkit_core::Settings;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Settings)]
#[settings(namespace = "appearance")]
pub struct AppearanceSettings {
    #[settings(default = "system")]
    pub theme: String,

    #[settings(default = 1.0)]
    pub font_scale: f64,

    #[settings(default = true)]
    pub animations_enabled: bool,

    pub custom_css: Option<String>,
}
```

### Attributes

**Struct-level:**
- `#[settings(namespace = "...")]` — Required. Storage namespace prefix.

**Field-level:**
- `#[settings(default = ...)]` — Optional default value. Supports string, bool, int, float literals.

### Default Values Without `#[settings(default)]`

| Type | Default |
|------|---------|
| `String` | `""` |
| `bool` | `false` |
| Integers | `0` |
| Floats | `0.0` |
| `Option<T>` | `null` (None) |

## License

MIT
