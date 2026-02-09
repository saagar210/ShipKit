import { useEffect, useState } from "react";
import type { ThemeDefinition } from "../lib/bindings";
import { listThemes, setTheme, getTheme, getCssVariables } from "../lib/invoke";

export function ThemePanel() {
  const [themes, setThemes] = useState<ThemeDefinition[]>([]);
  const [current, setCurrent] = useState<ThemeDefinition | null>(null);
  const [css, setCss] = useState("");
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    listThemes()
      .then(setThemes)
      .catch((e: unknown) => setError(String(e)));
    getTheme()
      .then(setCurrent)
      .catch((e: unknown) => setError(String(e)));
    getCssVariables()
      .then(setCss)
      .catch((e: unknown) => setError(String(e)));
  }, []);

  const handleSwitch = (name: string) => {
    setTheme(name)
      .then((t) => {
        setCurrent(t);
        return getCssVariables();
      })
      .then(setCss)
      .catch((e: unknown) => setError(String(e)));
  };

  return (
    <div style={{ border: "1px solid #ccc", borderRadius: 8, padding: 16 }}>
      <h2>Theme</h2>
      {error && <p style={{ color: "red" }}>{error}</p>}
      {current && (
        <p>
          Current: <strong>{current.name}</strong> ({current.mode})
        </p>
      )}
      <div style={{ display: "flex", gap: 8, marginBottom: 12 }}>
        {themes.map((t) => (
          <button
            key={t.name}
            onClick={() => handleSwitch(t.name)}
            style={{
              fontWeight: current?.name === t.name ? "bold" : "normal",
            }}
          >
            {t.name}
          </button>
        ))}
      </div>
      {css && (
        <pre
          style={{
            background: "#f5f5f5",
            padding: 8,
            borderRadius: 4,
            overflow: "auto",
            maxHeight: 200,
            fontSize: 12,
          }}
        >
          {css}
        </pre>
      )}
    </div>
  );
}
