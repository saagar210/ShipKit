import { useEffect, useState } from "react";
import type { MigrationStatus } from "../lib/bindings";
import {
  migrationStatus,
  applyMigrations,
  rollbackMigration,
} from "../lib/invoke";

export function DatabasePanel() {
  const [migrations, setMigrations] = useState<MigrationStatus[]>([]);
  const [error, setError] = useState<string | null>(null);

  const refresh = () => {
    migrationStatus()
      .then(setMigrations)
      .catch((e: unknown) => setError(String(e)));
  };

  useEffect(refresh, []);

  const handleApply = () => {
    applyMigrations()
      .then(setMigrations)
      .catch((e: unknown) => setError(String(e)));
  };

  const handleRollback = () => {
    rollbackMigration()
      .then(() => refresh())
      .catch((e: unknown) => setError(String(e)));
  };

  return (
    <div style={{ border: "1px solid #ccc", borderRadius: 8, padding: 16 }}>
      <h2>Database Migrations</h2>
      {error && <p style={{ color: "red" }}>{error}</p>}
      <div style={{ display: "flex", gap: 8, marginBottom: 12 }}>
        <button onClick={handleApply}>Apply All</button>
        <button onClick={handleRollback}>Rollback Last</button>
        <button onClick={refresh}>Refresh</button>
      </div>
      {migrations.length === 0 ? (
        <p>No migrations registered.</p>
      ) : (
        <table style={{ width: "100%", borderCollapse: "collapse" }}>
          <thead>
            <tr>
              <th style={{ textAlign: "left" }}>Version</th>
              <th style={{ textAlign: "left" }}>Name</th>
              <th style={{ textAlign: "left" }}>Status</th>
              <th style={{ textAlign: "left" }}>Applied At</th>
            </tr>
          </thead>
          <tbody>
            {migrations.map((m) => (
              <tr key={m.version}>
                <td>{m.version}</td>
                <td>{m.name}</td>
                <td>{m.applied ? "Applied" : "Pending"}</td>
                <td>{m.applied_at ?? "-"}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  );
}
