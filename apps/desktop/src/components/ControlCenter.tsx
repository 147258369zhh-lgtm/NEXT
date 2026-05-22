import { useState } from "react";
import { Icon } from "./Icon";
import type {
  AuditView,
  BrowserRuntimeDescriptor,
  CopyBundle,
  DevModeDescriptor,
  ExecutorDescriptor,
  ModuleDescriptor,
  ModuleStatus,
  PatchRunnerDescriptor,
  ProviderDescriptor
} from "../types";

type ChannelConfig = { apiKey: string; baseUrl: string; model: string };
type CustomChannel = { id: string; name: string; apiKey: string; baseUrl: string; model: string };

const PRESET_CHANNELS = [
  { id: "mock", name: "Mock Sandbox", desc: "本地离线沙箱模型", icon: "empty", configurable: false, defaultBaseUrl: "", defaultModel: "" },
  { id: "openai", name: "OpenAI", desc: "GPT-4o-mini", icon: "spark", configurable: true, defaultBaseUrl: "https://api.openai.com/v1", defaultModel: "gpt-4o-mini" },
  { id: "deepseek", name: "DeepSeek", desc: "DeepSeek Chat", icon: "spark", configurable: true, defaultBaseUrl: "https://api.deepseek.com/v1", defaultModel: "deepseek-chat" },
  { id: "ollama", name: "Ollama (本地)", desc: "本地推理 qwen2.5", icon: "brain", configurable: true, defaultBaseUrl: "http://localhost:11434/v1", defaultModel: "qwen2.5" },
  { id: "qwen", name: "通义千问 (Qwen)", desc: "阿里千问兼容通道", icon: "spark", configurable: true, defaultBaseUrl: "https://dashscope.aliyuncs.com/compatible-mode/v1", defaultModel: "qwen-plus" }
];

function loadChannelConfig(channelId: string): ChannelConfig {
  try {
    const raw = localStorage.getItem(`nexus_channel_config_${channelId}`);
    return raw ? JSON.parse(raw) : { apiKey: "", baseUrl: "", model: "" };
  } catch { return { apiKey: "", baseUrl: "", model: "" }; }
}

function saveChannelConfig(channelId: string, config: ChannelConfig): void {
  localStorage.setItem(`nexus_channel_config_${channelId}`, JSON.stringify(config));
}

function loadCustomChannels(): CustomChannel[] {
  try {
    const raw = localStorage.getItem("nexus_custom_channels");
    return raw ? JSON.parse(raw) : [];
  } catch { return []; }
}

function saveCustomChannels(channels: CustomChannel[]): void {
  localStorage.setItem("nexus_custom_channels", JSON.stringify(channels));
}

function loadAllChannelConfigs(): Record<string, ChannelConfig> {
  const configs: Record<string, ChannelConfig> = {};
  for (const ch of PRESET_CHANNELS) {
    if (ch.configurable) configs[ch.id] = loadChannelConfig(ch.id);
  }
  return configs;
}

export function ControlCenter({
  t,
  moduleStatus,
  modules,
  executors,
  providers,
  browserRuntimes,
  patchRunners,
  devModes = [],
  audits,
  onReloadProvider
}: {
  t: CopyBundle;
  moduleStatus: ModuleStatus;
  modules: ModuleDescriptor[];
  executors: ExecutorDescriptor[];
  providers: ProviderDescriptor[];
  browserRuntimes: BrowserRuntimeDescriptor[];
  patchRunners: PatchRunnerDescriptor[];
  devModes?: DevModeDescriptor[];
  audits: AuditView[];
  onReloadProvider: (mode: string) => Promise<void>;
}) {
  const [activeTab, setActiveTab] = useState<"model" | "modules" | "runtimes" | "integrations" | "audit">("model");
  const [expandedChannel, setExpandedChannel] = useState<string | null>(null);
  const [customChannels, setCustomChannels] = useState<CustomChannel[]>(() => loadCustomChannels());
  const [channelConfigs, setChannelConfigs] = useState<Record<string, ChannelConfig>>(() => loadAllChannelConfigs());
  const [draftConfig, setDraftConfig] = useState<ChannelConfig>({ apiKey: "", baseUrl: "", model: "" });
  const [showApiKey, setShowApiKey] = useState<string | null>(null);
  const [showAddCustom, setShowAddCustom] = useState(false);
  const [editingCustom, setEditingCustom] = useState<string | null>(null);
  const [editDraft, setEditDraft] = useState<CustomChannel>({ id: "", name: "", apiKey: "", baseUrl: "", model: "" });
  const [newCustom, setNewCustom] = useState<CustomChannel>({ id: "", name: "", apiKey: "", baseUrl: "", model: "" });
  const [activeCustomId, setActiveCustomId] = useState<string | null>(null);

  const browserAudits = audits.filter((audit) =>
    audit.event_type.startsWith("browser.")
  );
  const devAudits = audits.filter((audit) => audit.event_type.startsWith("dev."));
  const patchRunnerAudits = audits.filter((audit) => audit.event_type === "dev.runner");
  const latestPatchRunnerLogAudit = audits.find(
    (audit) => audit.event_type === "dev.runner_log"
  );
  const patchRunnerStatus = extractPatchRunnerStatus(latestPatchRunnerLogAudit?.result);

  return (
    <div className="secondary-workspace">
      <div className="secondary-header-row">
        <header className="secondary-header">
          <span>Nexus</span>
          <h1>{t.controlTitle}</h1>
          <p>{t.controlDesc}</p>
        </header>

        {/* Modern Sleek Sub-tabs for Settings */}
        <div className="control-tabs">
          <button
            className={`control-tab-btn ${activeTab === "model" ? "active" : ""}`}
            onClick={() => setActiveTab("model")}
            type="button"
          >
            <Icon name="provider" />
            <span>模型配置</span>
          </button>
          <button
            className={`control-tab-btn ${activeTab === "modules" ? "active" : ""}`}
            onClick={() => setActiveTab("modules")}
            type="button"
          >
            <Icon name="modules" />
            <span>模块与执行器</span>
          </button>
          <button
            className={`control-tab-btn ${activeTab === "runtimes" ? "active" : ""}`}
            onClick={() => setActiveTab("runtimes")}
            type="button"
          >
            <Icon name="memory" />
            <span>环境与运行器</span>
          </button>
          <button
            className={`control-tab-btn ${activeTab === "integrations" ? "active" : ""}`}
            onClick={() => setActiveTab("integrations")}
            type="button"
          >
            <Icon name="spark" />
            <span>外部集成</span>
          </button>
          <button
            className={`control-tab-btn ${activeTab === "audit" ? "active" : ""}`}
            onClick={() => setActiveTab("audit")}
            type="button"
          >
            <Icon name="risk" />
            <span>安全与审计</span>
          </button>
        </div>
      </div>

      <section className="control-board">
        {activeTab === "model" && (
          <>
            <article className="workspace-card control-span-full model-channel-card">
              <div className="panel-header">
                <h2>大模型通道预设与配置</h2>
                <span className="channel-badge">
                  当前活动通道: <strong>{moduleStatus.provider_source}</strong>
                </span>
              </div>
              <p className="card-desc">
                在下方直接配置各通道参数，点击「⚙ 配置」展开详细设置。
              </p>
              <div className="channels-grid">
                {PRESET_CHANNELS.map((chan) => {
                  const isActive = moduleStatus.provider_source.toLowerCase() === chan.id;
                  const isExpanded = expandedChannel === chan.id;
                  const savedConfig = channelConfigs[chan.id];
                  const hasSavedConfig = savedConfig && (savedConfig.apiKey || savedConfig.baseUrl || savedConfig.model);
                  return (
                    <div key={chan.id} className={`channel-item ${isActive ? "active" : ""} ${isExpanded ? "expanded" : ""}`}>
                      <div className="channel-meta">
                        <Icon name={chan.icon as any} />
                        <div>
                          <strong>{chan.name}</strong>
                          <span>{chan.desc}{hasSavedConfig ? " · ✓ 已配置" : ""}</span>
                        </div>
                      </div>
                      <div className="channel-actions-row">
                        {chan.configurable && (
                          <button
                            className={`channel-config-toggle ${isExpanded ? "open" : ""}`}
                            onClick={() => {
                              if (isExpanded) {
                                setExpandedChannel(null);
                              } else {
                                setExpandedChannel(chan.id);
                                const existing = channelConfigs[chan.id] || { apiKey: "", baseUrl: "", model: "" };
                                setDraftConfig({
                                  apiKey: existing.apiKey || "",
                                  baseUrl: existing.baseUrl || chan.defaultBaseUrl,
                                  model: existing.model || chan.defaultModel
                                });
                              }
                            }}
                            type="button"
                          >
                            {isExpanded ? "收起" : "⚙ 配置"}
                          </button>
                        )}
                        <button
                          className={`channel-action-btn ${isActive ? "active" : ""}`}
                          onClick={() => { if (!isActive) { setActiveCustomId(null); onReloadProvider(chan.id); } }}
                          disabled={isActive}
                          type="button"
                        >
                          {isActive ? "● 运行中" : "切换通道"}
                        </button>
                      </div>
                      {isExpanded && chan.configurable && (
                        <div className="channel-config-panel">
                          <div className="channel-config-field">
                            <label>API Key</label>
                            <div className="config-input-wrap">
                              <input
                                type={showApiKey === chan.id ? "text" : "password"}
                                className="channel-config-input"
                                value={draftConfig.apiKey}
                                onChange={(e) => setDraftConfig({ ...draftConfig, apiKey: e.target.value })}
                                placeholder="输入 API Key..."
                              />
                              <button type="button" className="eye-toggle" onClick={() => setShowApiKey(showApiKey === chan.id ? null : chan.id)}>
                                {showApiKey === chan.id ? "◉" : "○"}
                              </button>
                            </div>
                          </div>
                          <div className="channel-config-field">
                            <label>Base URL</label>
                            <input
                              type="text"
                              className="channel-config-input"
                              value={draftConfig.baseUrl}
                              onChange={(e) => setDraftConfig({ ...draftConfig, baseUrl: e.target.value })}
                              placeholder={chan.defaultBaseUrl}
                            />
                          </div>
                          <div className="channel-config-field">
                            <label>Model</label>
                            <input
                              type="text"
                              className="channel-config-input"
                              value={draftConfig.model}
                              onChange={(e) => setDraftConfig({ ...draftConfig, model: e.target.value })}
                              placeholder={chan.defaultModel}
                            />
                          </div>
                          <div className="channel-config-actions">
                            <button type="button" className="config-save-btn" onClick={() => {
                              saveChannelConfig(chan.id, draftConfig);
                              setChannelConfigs({ ...channelConfigs, [chan.id]: { ...draftConfig } });
                              setExpandedChannel(null);
                            }}>
                              保存配置
                            </button>
                            <button type="button" className="config-cancel-btn" onClick={() => setExpandedChannel(null)}>
                              取消
                            </button>
                          </div>
                        </div>
                      )}
                    </div>
                  );
                })}
              </div>
            </article>

            <article className="workspace-card control-span-full model-channel-card">
              <div className="panel-header">
                <h2>自定义模型通道</h2>
                <span className="channel-badge">
                  已添加 <strong>{customChannels.length}</strong> 个
                </span>
              </div>
              <p className="card-desc">
                添加任意 OpenAI 兼容的第三方模型通道，支持 OneAPI、LiteLLM 等中转服务。
              </p>
              <div className="channels-grid">
                {customChannels.map((chan) => {
                  const isActive = activeCustomId === chan.id;
                  const isEditing = editingCustom === chan.id;
                  return (
                    <div key={chan.id} className={`channel-item ${isActive ? "active" : ""} ${isEditing ? "expanded" : ""}`}>
                      {isEditing ? (
                        <div className="channel-config-panel inline">
                          <div className="channel-config-field">
                            <label>名称</label>
                            <input type="text" className="channel-config-input" value={editDraft.name} onChange={(e) => setEditDraft({ ...editDraft, name: e.target.value })} placeholder="通道名称" />
                          </div>
                          <div className="channel-config-field">
                            <label>API Key</label>
                            <div className="config-input-wrap">
                              <input type={showApiKey === chan.id ? "text" : "password"} className="channel-config-input" value={editDraft.apiKey} onChange={(e) => setEditDraft({ ...editDraft, apiKey: e.target.value })} placeholder="sk-..." />
                              <button type="button" className="eye-toggle" onClick={() => setShowApiKey(showApiKey === chan.id ? null : chan.id)}>{showApiKey === chan.id ? "◉" : "○"}</button>
                            </div>
                          </div>
                          <div className="channel-config-field">
                            <label>Base URL</label>
                            <input type="text" className="channel-config-input" value={editDraft.baseUrl} onChange={(e) => setEditDraft({ ...editDraft, baseUrl: e.target.value })} placeholder="https://api.example.com/v1" />
                          </div>
                          <div className="channel-config-field">
                            <label>Model</label>
                            <input type="text" className="channel-config-input" value={editDraft.model} onChange={(e) => setEditDraft({ ...editDraft, model: e.target.value })} placeholder="gpt-4o-mini" />
                          </div>
                          <div className="channel-config-actions">
                            <button type="button" className="config-save-btn" onClick={() => {
                              const updated = customChannels.map((c) => c.id === chan.id ? { ...editDraft } : c);
                              setCustomChannels(updated);
                              saveCustomChannels(updated);
                              setEditingCustom(null);
                            }}>保存</button>
                            <button type="button" className="config-cancel-btn" onClick={() => setEditingCustom(null)}>取消</button>
                          </div>
                        </div>
                      ) : (
                        <>
                          <div className="channel-meta">
                            <Icon name="spark" />
                            <div>
                              <strong>{chan.name}</strong>
                              <span>{chan.model || "未配置模型"}</span>
                            </div>
                          </div>
                          <div className="channel-actions-row">
                            <button type="button" className="channel-config-toggle" onClick={() => { setEditingCustom(chan.id); setEditDraft({ ...chan }); }}>编辑</button>
                            <button type="button" className="channel-delete-btn" onClick={() => {
                              const updated = customChannels.filter((c) => c.id !== chan.id);
                              setCustomChannels(updated);
                              saveCustomChannels(updated);
                              if (activeCustomId === chan.id) setActiveCustomId(null);
                            }}>删除</button>
                            <button className={`channel-action-btn ${isActive ? "active" : ""}`} onClick={() => { if (!isActive) { setActiveCustomId(chan.id); onReloadProvider("openai-compatible"); } }} disabled={isActive} type="button">
                              {isActive ? "● 运行中" : "切换通道"}
                            </button>
                          </div>
                        </>
                      )}
                    </div>
                  );
                })}
                {showAddCustom ? (
                  <div className="channel-item expanded add-form">
                    <div className="channel-config-panel inline">
                      <div className="channel-config-field">
                        <label>名称</label>
                        <input type="text" className="channel-config-input" value={newCustom.name} onChange={(e) => setNewCustom({ ...newCustom, name: e.target.value })} placeholder="例: GPT-4o via OneAPI" />
                      </div>
                      <div className="channel-config-field">
                        <label>API Key</label>
                        <div className="config-input-wrap">
                          <input type={showApiKey === "_new" ? "text" : "password"} className="channel-config-input" value={newCustom.apiKey} onChange={(e) => setNewCustom({ ...newCustom, apiKey: e.target.value })} placeholder="sk-..." />
                          <button type="button" className="eye-toggle" onClick={() => setShowApiKey(showApiKey === "_new" ? null : "_new")}>{showApiKey === "_new" ? "◉" : "○"}</button>
                        </div>
                      </div>
                      <div className="channel-config-field">
                        <label>Base URL</label>
                        <input type="text" className="channel-config-input" value={newCustom.baseUrl} onChange={(e) => setNewCustom({ ...newCustom, baseUrl: e.target.value })} placeholder="https://api.example.com/v1" />
                      </div>
                      <div className="channel-config-field">
                        <label>Model</label>
                        <input type="text" className="channel-config-input" value={newCustom.model} onChange={(e) => setNewCustom({ ...newCustom, model: e.target.value })} placeholder="gpt-4o-mini" />
                      </div>
                      <div className="channel-config-actions">
                        <button type="button" className="config-save-btn" onClick={() => {
                          if (!newCustom.name.trim()) return;
                          const channel: CustomChannel = { ...newCustom, id: `c_${Date.now()}` };
                          const updated = [...customChannels, channel];
                          setCustomChannels(updated);
                          saveCustomChannels(updated);
                          setNewCustom({ id: "", name: "", apiKey: "", baseUrl: "", model: "" });
                          setShowAddCustom(false);
                        }}>添加通道</button>
                        <button type="button" className="config-cancel-btn" onClick={() => { setShowAddCustom(false); setNewCustom({ id: "", name: "", apiKey: "", baseUrl: "", model: "" }); }}>取消</button>
                      </div>
                    </div>
                  </div>
                ) : (
                  <button type="button" className="channel-add-card" onClick={() => setShowAddCustom(true)}>
                    <span className="add-icon">+</span>
                    <span>添加自定义模型</span>
                  </button>
                )}
              </div>
            </article>

            <article className="workspace-card control-span-full">
              <div className="panel-header">
                <h2>模型使用统计</h2>
                <span className="channel-badge">最近 7 天</span>
              </div>
              <div className="usage-chart-area">
                <svg className="usage-chart" viewBox="0 0 700 180" preserveAspectRatio="none">
                  <defs>
                    <linearGradient id="chartGrad" x1="0" y1="0" x2="0" y2="1">
                      <stop offset="0%" stopColor="#18181b" stopOpacity="0.12" />
                      <stop offset="100%" stopColor="#18181b" stopOpacity="0.01" />
                    </linearGradient>
                  </defs>
                  <path d="M0 180 L0 140 Q50 130 100 120 T200 90 T300 100 T400 60 T500 70 T600 40 T700 30 L700 180 Z" fill="url(#chartGrad)" />
                  <path d="M0 140 Q50 130 100 120 T200 90 T300 100 T400 60 T500 70 T600 40 T700 30" fill="none" stroke="#18181b" strokeWidth="2.5" strokeLinecap="round" />
                  {[0, 100, 200, 300, 400, 500, 600, 700].map((x, i) => (
                    <line key={i} x1={x} y1="0" x2={x} y2="180" stroke="#e4e4e7" strokeWidth="0.5" strokeDasharray="4 4" />
                  ))}
                  {[0, 45, 90, 135, 180].map((y, i) => (
                    <line key={i} x1="0" y1={y} x2="700" y2={y} stroke="#e4e4e7" strokeWidth="0.5" />
                  ))}
                </svg>
                <div className="chart-x-labels">
                  {["周一", "周二", "周三", "周四", "周五", "周六", "周日"].map((d) => (
                    <span key={d}>{d}</span>
                  ))}
                </div>
              </div>
              <div className="usage-summary-row">
                <div className="usage-summary-item">
                  <span>总请求数</span>
                  <strong>1,284</strong>
                </div>
                <div className="usage-summary-item">
                  <span>Token 消耗</span>
                  <strong>2.4M</strong>
                </div>
                <div className="usage-summary-item">
                  <span>平均延迟</span>
                  <strong>320ms</strong>
                </div>
                <div className="usage-summary-item">
                  <span>成功率</span>
                  <strong>99.6%</strong>
                </div>
              </div>
            </article>

            <article className="workspace-card control-span-full">
              <div className="panel-header">
                <h2>最近使用记录</h2>
              </div>
              <div className="usage-log-list">
                {[
                  { time: "16:38", model: moduleStatus.provider_source, tokens: "1,240", latency: "285ms", status: "success" },
                  { time: "16:35", model: moduleStatus.provider_source, tokens: "3,802", latency: "412ms", status: "success" },
                  { time: "16:31", model: moduleStatus.provider_source, tokens: "956", latency: "198ms", status: "success" },
                  { time: "16:27", model: moduleStatus.provider_source, tokens: "2,115", latency: "345ms", status: "success" },
                  { time: "16:20", model: moduleStatus.provider_source, tokens: "4,501", latency: "520ms", status: "success" },
                  { time: "16:14", model: moduleStatus.provider_source, tokens: "780", latency: "156ms", status: "success" }
                ].map((log, i) => (
                  <div className="usage-log-item" key={i}>
                    <span className="log-time">{log.time}</span>
                    <span className="log-model">{log.model}</span>
                    <span className="log-tokens">{log.tokens} tokens</span>
                    <span className="log-latency">{log.latency}</span>
                    <span className={`log-status ${log.status}`}>✓</span>
                  </div>
                ))}
              </div>
            </article>
          </>
        )}

        {activeTab === "modules" && (
          <>
            <article className="workspace-card">
              <div className="panel-header">
                <h2>{t.controlModuleInventory}</h2>
              </div>
              <div className="module-inventory">
                {modules.map((module) => (
                  <div className="inventory-item" key={module.id}>
                    <div className="inventory-copy">
                      <strong>{module.title}</strong>
                      <span>{module.hot_swappable ? t.hotSwappable : t.nativeSettings}</span>
                    </div>
                    <span className={`module-state ${module.enabled ? "on" : "off"}`}>
                      {module.enabled ? t.enabled : t.disabled}
                    </span>
                  </div>
                ))}
              </div>
            </article>

            <article className="workspace-card">
              <div className="panel-header">
                <h2>{t.controlExecutors}</h2>
              </div>
              <div className="module-inventory">
                {executors.map((executor) => (
                  <div className="inventory-item" key={executor.id}>
                    <div className="inventory-copy">
                      <strong>{executor.title}</strong>
                      <span>
                        {executor.family} / {executor.risk_ceiling} / {executor.integration_level}
                      </span>
                      <span>{executor.summary}</span>
                      <span>{executor.task_kinds.join(" / ")}</span>
                    </div>
                    <span className={`module-state ${executor.enabled ? "on" : "off"}`}>
                      {executor.enabled ? t.enabled : t.disabled}
                    </span>
                  </div>
                ))}
              </div>
            </article>

            <article className="workspace-card control-span-full">
              <div className="panel-header">
                <h2>{t.controlProviders}</h2>
              </div>
              <div className="module-inventory">
                {providers.map((provider) => (
                  <div className="inventory-item" key={provider.id}>
                    <div className="inventory-copy">
                      <strong>{provider.title}</strong>
                      <span>
                        {provider.vendor} / {provider.family} /{" "}
                        {provider.local_first ? "local-first" : "cloud-ready"}
                      </span>
                    </div>
                    <span className={`module-state ${provider.enabled ? "on" : "off"}`}>
                      {provider.enabled ? t.enabled : t.disabled}
                    </span>
                  </div>
                ))}
              </div>
            </article>

            <article className="workspace-card control-span-full">
              <div className="panel-header">
                <h2>{t.controlRuntime}</h2>
              </div>
              <div className="control-grid">
                <div className="control-stat">
                  <span>{t.providerModule}</span>
                  <strong>{moduleStatus.provider_source}</strong>
                </div>
                <div className="control-stat">
                  <span>{t.riskPolicy}</span>
                  <strong>{moduleStatus.risk_policy_source}</strong>
                </div>
                <div className="control-stat">
                  <span>{t.brainKernel}</span>
                  <strong>{moduleStatus.brain_enabled ? t.enabled : t.disabled}</strong>
                </div>
                <div className="control-stat">
                  <span>{t.memoryModule}</span>
                  <strong>{moduleStatus.memory_enabled ? t.enabled : t.disabled}</strong>
                </div>
              </div>
            </article>
          </>
        )}

        {activeTab === "runtimes" && (
          <>
            <article className="workspace-card">
              <div className="panel-header">
                <h2>{t.controlBrowserRuntimes}</h2>
              </div>
              <div className="module-inventory">
                {browserRuntimes.map((runtime) => (
                  <div className="inventory-item" key={runtime.id}>
                    <div className="inventory-copy">
                      <strong>{runtime.title}</strong>
                      <span>
                        {runtime.engine} /{" "}
                        {runtime.headless_default ? "headless-default" : "interactive-default"} /{" "}
                        {runtime.supports_live_control ? "live-control" : "no-live-control"}
                      </span>
                    </div>
                    <span className={`module-state ${runtime.enabled ? "on" : "off"}`}>
                      {runtime.enabled ? t.enabled : t.disabled}
                    </span>
                  </div>
                ))}
              </div>
            </article>

            <article className="workspace-card">
              <div className="panel-header">
                <h2>{t.controlPatchRunners}</h2>
              </div>
              <div className="module-inventory">
                {patchRunners.map((runner) => (
                  <div className="inventory-item" key={runner.id}>
                    <div className="inventory-copy">
                      <strong>{runner.title}</strong>
                      <span>
                        {runner.family} / {runner.mode} / {runner.integration_level}
                      </span>
                      <span>
                        {runner.source} / {runner.license} / {runner.review_status}
                      </span>
                      <span>
                        {runner.mutates_files ? "mutates-files" : "dry-run"} /{" "}
                        {runner.requires_approval ? "approval" : "no-approval"}
                      </span>
                    </div>
                    <span className={`module-state ${runner.enabled ? "on" : "off"}`}>
                      {runner.enabled ? t.enabled : t.disabled}
                    </span>
                  </div>
                ))}
              </div>
            </article>

            <article className="workspace-card control-span-full">
              <div className="panel-header">
                <h2>{t.controlDevModes}</h2>
              </div>
              <div className="module-inventory">
                {devModes.map((mode) => (
                  <div className="inventory-item" key={mode.slug}>
                    <div className="inventory-copy">
                      <strong>{mode.title}</strong>
                      <span>
                        {mode.slug} / {mode.intent} / {mode.default_runner}
                      </span>
                      <span>{mode.allowed_tool_groups.join(" / ")}</span>
                      <span>
                        {mode.mutates_files ? "mutates-files" : "read-only"} /{" "}
                        {mode.requires_approval ? "approval" : "no-approval"}
                      </span>
                    </div>
                    <span className="module-state on">{mode.borrowed_from}</span>
                  </div>
                ))}
              </div>
            </article>
          </>
        )}

        {activeTab === "integrations" && (
          <>
            <article className="workspace-card">
              <div className="panel-header">
                <h2>{t.controlConnectors}</h2>
              </div>
              <div className="control-placeholder">
                <strong>WeChat Connector</strong>
                <p>{t.controlPlaceholder}</p>
              </div>
            </article>

            <article className="workspace-card">
              <div className="panel-header">
                <h2>{t.controlVoice}</h2>
              </div>
              <div className="control-placeholder">
                <strong>Push-to-talk runtime</strong>
                <p>{t.controlPlaceholder}</p>
              </div>
            </article>
          </>
        )}

        {activeTab === "audit" && (
          <>
            <article className="workspace-card">
              <div className="panel-header">
                <h2>{t.controlPatchRunnerStatus}</h2>
              </div>
              {patchRunnerStatus.length > 0 ? (
                <div className="workspace-actions">
                  {patchRunnerStatus.map((item) => (
                    <div key={item} className="action-item">
                      <strong>{item}</strong>
                    </div>
                  ))}
                </div>
              ) : (
                <div className="control-placeholder">
                  <strong>{t.noDataTitle}</strong>
                  <p>{t.controlPlaceholder}</p>
                </div>
              )}
            </article>

            <article className="workspace-card">
              <div className="panel-header">
                <h2>{t.controlPatchRunnerActivity}</h2>
              </div>
              <div className="audit-list">
                {patchRunnerAudits.length === 0 ? (
                  <div className="control-placeholder">
                    <strong>{t.noDataTitle}</strong>
                    <p>{t.controlPlaceholder}</p>
                  </div>
                ) : (
                  patchRunnerAudits.map((audit) => (
                    <div className="audit-item" key={audit.id}>
                      <div className="audit-copy">
                        <strong>{audit.event_type}</strong>
                        <span>{audit.tool_name ?? "patch-runner"}</span>
                      </div>
                      <div className="audit-meta">
                        <span>{audit.risk_level}</span>
                        <span>{new Date(audit.timestamp).toLocaleString()}</span>
                      </div>
                      <p>{audit.result}</p>
                    </div>
                  ))
                )}
              </div>
            </article>

            <article className="workspace-card">
              <div className="panel-header">
                <h2>{t.controlBrowserActivity}</h2>
              </div>
              <div className="audit-list">
                {browserAudits.length === 0 ? (
                  <div className="control-placeholder">
                    <strong>{t.noDataTitle}</strong>
                    <p>{t.controlPlaceholder}</p>
                  </div>
                ) : (
                  browserAudits.map((audit) => (
                    <div className="audit-item browser-audit" key={audit.id}>
                      <div className="audit-copy">
                        <strong>{audit.event_type}</strong>
                        <span>{audit.tool_name ?? "browser-executor"}</span>
                      </div>
                      <div className="audit-meta">
                        <span>{audit.risk_level}</span>
                        <span>{new Date(audit.timestamp).toLocaleString()}</span>
                      </div>
                      <p>{audit.result}</p>
                    </div>
                  ))
                )}
              </div>
            </article>

            <article className="workspace-card">
              <div className="panel-header">
                <h2>{t.controlDevActivity}</h2>
              </div>
              <div className="audit-list">
                {devAudits.length === 0 ? (
                  <div className="control-placeholder">
                    <strong>{t.noDataTitle}</strong>
                    <p>{t.controlPlaceholder}</p>
                  </div>
                ) : (
                  devAudits.map((audit) => (
                    <div className="audit-item" key={audit.id}>
                      <div className="audit-copy">
                        <strong>{audit.event_type}</strong>
                        <span>{audit.tool_name ?? "dev-executor"}</span>
                      </div>
                      <div className="audit-meta">
                        <span>{audit.risk_level}</span>
                        <span>{new Date(audit.timestamp).toLocaleString()}</span>
                      </div>
                      <p>{audit.result}</p>
                    </div>
                  ))
                )}
              </div>
            </article>

            <article className="workspace-card control-span-full">
              <div className="panel-header">
                <h2>{t.controlAudit}</h2>
              </div>
              <div className="audit-list">
                {audits.length === 0 ? (
                  <div className="control-placeholder">
                    <strong>{t.noDataTitle}</strong>
                    <p>{t.controlPlaceholder}</p>
                  </div>
                ) : (
                  audits.map((audit) => (
                    <div className="audit-item" key={audit.id}>
                      <div className="audit-copy">
                        <strong>{audit.event_type}</strong>
                        <span>
                          {audit.actor} / {audit.channel}
                          {audit.tool_name ? ` / ${audit.tool_name}` : ""}
                        </span>
                      </div>
                      <div className="audit-meta">
                        <span>{audit.risk_level}</span>
                        <span>{new Date(audit.timestamp).toLocaleString()}</span>
                      </div>
                      <p>{audit.result}</p>
                    </div>
                  ))
                )}
              </div>
            </article>
          </>
        )}
      </section>
    </div>
  );
}

function extractPatchRunnerStatus(raw?: string): string[] {
  if (!raw) {
    return [];
  }

  try {
    const parsed = JSON.parse(raw) as {
      runner_id?: string;
      mode?: string;
      guard?: { status?: string; violations?: string[]; mode_slug?: string };
      log_entries?: string[];
    };

    const items = [
      parsed.runner_id ? `runner: ${parsed.runner_id}` : null,
      parsed.mode ? `mode: ${parsed.mode}` : null,
      parsed.guard?.status ? `guard: ${parsed.guard.status}` : null,
      parsed.guard?.mode_slug ? `mode: ${parsed.guard.mode_slug}` : null,
      Array.isArray(parsed.guard?.violations)
        ? `violations: ${parsed.guard?.violations.length}`
        : null,
      Array.isArray(parsed.log_entries)
        ? `log entries: ${parsed.log_entries.length}`
        : null,
      Array.isArray(parsed.log_entries) && parsed.log_entries.length > 0
        ? `latest: ${parsed.log_entries[parsed.log_entries.length - 1]}`
        : null
    ];

    return items.filter((item): item is string => Boolean(item));
  } catch {
    return [];
  }
}





