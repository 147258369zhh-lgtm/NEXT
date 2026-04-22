import { Icon } from "./Icon";
import type { CopyBundle, Locale } from "../types";

export function Topbar({
  t,
  locale,
  pendingCount,
  onLocaleChange
}: {
  t: CopyBundle;
  locale: Locale;
  pendingCount: number;
  onLocaleChange: (locale: Locale) => void;
}) {
  return (
    <header className="topbar">
      <div className="topbar-title">
        <span className="eyebrow">{t.workspaceLabel}</span>
        <strong>{t.appTitle}</strong>
        <span>{t.appSubtitle}</span>
      </div>

      <div className="topbar-metrics">
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
