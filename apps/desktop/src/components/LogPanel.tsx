import { useEffect, useState } from "react";
import type { LogEntry } from "../lib/bindings";
import { getLogEntries } from "../lib/invoke";

export function LogPanel() {
  const [entries, setEntries] = useState<LogEntry[]>([]);
  const [level, setLevel] = useState<string>("all");
  const [error, setError] = useState<string | null>(null);

  const refresh = () => {
    const filter = level === "all" ? undefined : level;
    getLogEntries(50, filter)
      .then(setEntries)
      .catch((e: unknown) => setError(String(e)));
  };

  useEffect(refresh, [level]);

  return (
    <div style={{ border: "1px solid #ccc", borderRadius: 8, padding: 16 }}>
      <h2>Logs</h2>
      {error && <p style={{ color: "red" }}>{error}</p>}
      <div style={{ display: "flex", gap: 8, marginBottom: 12 }}>
        <select value={level} onChange={(e) => setLevel(e.target.value)}>
          <option value="all">All Levels</option>
          <option value="INFO">Info</option>
          <option value="WARN">Warn</option>
          <option value="ERROR">Error</option>
          <option value="DEBUG">Debug</option>
          <option value="TRACE">Trace</option>
        </select>
        <button onClick={refresh}>Refresh</button>
      </div>
      <div
        style={{
          maxHeight: 300,
          overflow: "auto",
          fontSize: 12,
          fontFamily: "monospace",
        }}
      >
        {entries.length === 0 ? (
          <p>No log entries.</p>
        ) : (
          entries.map((e, i) => (
            <div
              key={i}
              style={{
                padding: "2px 0",
                borderBottom: "1px solid #eee",
                color:
                  e.level === "ERROR"
                    ? "red"
                    : e.level === "WARN"
                      ? "orange"
                      : "inherit",
              }}
            >
              <span>{e.timestamp}</span>{" "}
              <strong>[{e.level}]</strong>{" "}
              <span>{e.message}</span>
            </div>
          ))
        )}
      </div>
    </div>
  );
}
