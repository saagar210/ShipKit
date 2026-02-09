import { DatabasePanel } from "./components/DatabasePanel";
import { SettingsPanel } from "./components/SettingsPanel";
import { ThemePanel } from "./components/ThemePanel";
import { LogPanel } from "./components/LogPanel";

function App() {
  return (
    <div
      style={{
        display: "grid",
        gridTemplateColumns: "1fr 1fr",
        gap: 16,
        padding: 16,
        fontFamily: "system-ui, sans-serif",
      }}
    >
      <DatabasePanel />
      <SettingsPanel />
      <ThemePanel />
      <LogPanel />
    </div>
  );
}

export default App;
