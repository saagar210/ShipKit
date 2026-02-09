import { useState } from "react";
import {
  getSetting,
  setSetting,
  loadSettings,
  saveSettings,
} from "../lib/invoke";

export function SettingsPanel() {
  const [namespace, setNamespace] = useState("demo");
  const [key, setKey] = useState("");
  const [value, setValue] = useState("");
  const [result, setResult] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleSet = () => {
    let parsed: unknown;
    try {
      parsed = JSON.parse(value);
    } catch {
      parsed = value;
    }
    setSetting(namespace, key, parsed)
      .then(() => setResult("Saved."))
      .catch((e: unknown) => setError(String(e)));
  };

  const handleGet = () => {
    getSetting(namespace, key)
      .then((v) => setResult(JSON.stringify(v, null, 2)))
      .catch((e: unknown) => setError(String(e)));
  };

  const handleLoadAll = () => {
    loadSettings(namespace)
      .then((v) => setResult(JSON.stringify(v, null, 2)))
      .catch((e: unknown) => setError(String(e)));
  };

  const handleSaveBulk = () => {
    try {
      const obj = JSON.parse(value) as Record<string, unknown>;
      saveSettings(namespace, obj)
        .then(() => setResult("Bulk save complete."))
        .catch((e: unknown) => setError(String(e)));
    } catch {
      setError("Value must be a valid JSON object for bulk save.");
    }
  };

  return (
    <div style={{ border: "1px solid #ccc", borderRadius: 8, padding: 16 }}>
      <h2>Settings</h2>
      {error && <p style={{ color: "red" }}>{error}</p>}
      <div style={{ display: "flex", flexDirection: "column", gap: 8 }}>
        <label>
          Namespace:{" "}
          <input
            value={namespace}
            onChange={(e) => setNamespace(e.target.value)}
          />
        </label>
        <label>
          Key: <input value={key} onChange={(e) => setKey(e.target.value)} />
        </label>
        <label>
          Value:{" "}
          <input value={value} onChange={(e) => setValue(e.target.value)} />
        </label>
        <div style={{ display: "flex", gap: 8 }}>
          <button onClick={handleSet}>Set</button>
          <button onClick={handleGet}>Get</button>
          <button onClick={handleLoadAll}>Load All</button>
          <button onClick={handleSaveBulk}>Save Bulk</button>
        </div>
      </div>
      {result && (
        <pre
          style={{
            marginTop: 12,
            background: "#f5f5f5",
            padding: 8,
            borderRadius: 4,
            overflow: "auto",
            maxHeight: 200,
          }}
        >
          {result}
        </pre>
      )}
    </div>
  );
}
