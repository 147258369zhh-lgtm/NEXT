import type { CopyBundle, Locale, MainView } from "../types";

/**
 * Topbar — Minimal top bar with view switch and language toggle.
 * Codex-style: thin, unobtrusive, functional.
 */
export function Topbar({
  t,
  locale,
  pendingCount,
  mainView,
  onLocaleChange,
  onMainViewChange
}: {
  t: CopyBundle;
  locale: Locale;
  pendingCount: number;
  mainView: MainView;
  onLocaleChange: (locale: Locale) => void;
  onMainViewChange: (view: MainView) => void;
}) {
  return (
    <header className="topbar">
      <div className="topbar-title">
        <strong>{t.appTitle}</strong>
      </div>

      <div className="topbar-metrics">
        <div className="view-switch">
          <button
            className={mainView === "workspace" ? "active" : ""}
            type="button"
            onClick={() => onMainViewChange("workspace")}
          >
            {t.workspaceTab}
          </button>
          <button
            className={mainView === "control" ? "active" : ""}
            type="button"
            onClick={() => onMainViewChange("control")}
          >
            {t.controlTab}
          </button>
        </div>
        {pendingCount > 0 && (
          <span className="metric-chip warn">
            {t.approvalsCount} {pendingCount}
          </span>
        )}
      </div>

      <div className="lang-switch">
        <button
          className={`lang-btn ${locale === "zh-CN" ? "active" : ""}`}
          type="button"
          onClick={() => onLocaleChange("zh-CN")}
        >
          {t.langZh}
        </button>
        <button
          className={`lang-btn ${locale === "en-US" ? "active" : ""}`}
          type="button"
          onClick={() => onLocaleChange("en-US")}
        >
          {t.langEn}
        </button>
      </div>
    </header>
  );
}
