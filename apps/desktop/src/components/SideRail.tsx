import { Icon } from "./Icon";
import type { CopyBundle, IconName, Locale, MainView } from "../types";

export function SideRail({
  t,
  locale,
  mainView,
  pendingCount,
  memoryCards,
  onLocaleChange,
  onMainViewChange
}: {
  t: CopyBundle;
  locale: Locale;
  mainView: MainView;
  pendingCount: number;
  memoryCards: number;
  onLocaleChange: (locale: Locale) => void;
  onMainViewChange: (view: MainView) => void;
}) {
  const navItems: Array<{ id: string; label: string; icon: IconName; view?: MainView }> = [
    { id: "chat", label: t.navChat, icon: "spark", view: "workspace" },
    { id: "search", label: t.navSearch, icon: "history", view: "search" },
    { id: "skills", label: t.navSkills, icon: "brain", view: "skills" },
    { id: "plugins", label: t.navPlugins, icon: "modules", view: "plugins" },
    { id: "automation", label: t.navAutomation, icon: "approval", view: "automation" },
    { id: "projects", label: t.navProjects, icon: "memory", view: "projects" },
    { id: "settings", label: t.navSettings, icon: "provider", view: "control" }
  ];

  return (
    <aside className="side-rail codex-nav">
      <div className="nav-brand">
        <div className="brand-mark">N</div>
        <div>
          <strong>{t.appTitle}</strong>
          <span>{t.appSubtitle}</span>
        </div>
      </div>

      <nav className="nav-list" aria-label={t.workspaceLabel}>
        {navItems.map((item) => {
          const active = item.view ? mainView === item.view : false;
          return (
            <button
              className={active ? "active" : ""}
              key={item.id}
              type="button"
              onClick={() => item.view && onMainViewChange(item.view)}
            >
              <Icon name={item.icon} />
              <span>{item.label}</span>
              {item.id === "settings" && pendingCount > 0 ? <em>{pendingCount}</em> : null}
            </button>
          );
        })}
      </nav>

      <section className="nav-projects">
        <span>{t.projectSection}</span>
        <button className={`project-pill ${mainView === "projects" ? "active" : ""}`} type="button" onClick={() => onMainViewChange("projects")}>
          <strong>NEXT</strong>
          <small>{t.currentThread}</small>
        </button>
      </section>

      <section className="nav-bottom">
        <div className="mini-stat">
          <span>{t.pendingItems}</span>
          <strong>{pendingCount}</strong>
        </div>
        <div className="mini-stat">
          <span>{t.memoryModule}</span>
          <strong>{memoryCards}</strong>
        </div>
        <div className="lang-switch compact">
          <button
            className={`lang-btn ${locale === "zh-CN" ? "active" : ""}`}
            type="button"
            onClick={() => onLocaleChange("zh-CN")}
          >
            中
          </button>
          <button
            className={`lang-btn ${locale === "en-US" ? "active" : ""}`}
            type="button"
            onClick={() => onLocaleChange("en-US")}
          >
            EN
          </button>
        </div>
      </section>
    </aside>
  );
}
