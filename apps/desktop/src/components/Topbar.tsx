import { Icon } from "./Icon";
import type { CopyBundle, Locale, MainView } from "../types";

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
        <div className="topbar-title-main">
          <strong>{t.appTitle}</strong>
          <span className="eyebrow-badge">{t.workspaceLabel}</span>
        </div>
        <span className="topbar-subtitle">{t.appSubtitle}</span>
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
        <span className="metric-chip">
          <Icon name="provider" /> {t.providerLive}
        </span>
        <span className="metric-chip">
          <Icon name="risk" /> {t.riskActive}
        </span>
        <span className={`metric-chip ${pendingCount > 0 ? "warn" : "ok"}`}>
          <Icon name="approval" /> {t.approvalsCount} {pendingCount}
        </span>
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
