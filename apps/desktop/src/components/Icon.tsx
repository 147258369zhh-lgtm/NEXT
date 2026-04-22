import type { IconName } from "../types";

export function Icon({ name }: { name: IconName }) {
  const paths: Record<IconName, string> = {
    modules: "M4 4h7v7H4V4zm9 0h7v7h-7V4zM4 13h7v7H4v-7zm9 0h7v7h-7v-7z",
    history:
      "M12 2a10 10 0 1 0 7.1 2.9L17 7a7 7 0 1 1-5-2v4l4-4-4-4v3z",
    provider:
      "M3 5h18v5H3V5zm0 9h18v5H3v-5zm4-7h2v1H7V7zm0 9h2v1H7v-1z",
    risk:
      "M12 3l9 16H3l9-16zm0 5-1 5h2l-1-5zm0 8a1.2 1.2 0 1 0 0 2.4A1.2 1.2 0 0 0 12 16z",
    approval:
      "M12 2 3 6v6c0 5 3.8 9.7 9 10 5.2-.3 9-5 9-10V6l-9-4zm-1 13-4-4 1.4-1.4L11 12.2l4.6-4.6L17 9l-6 6z",
    brain: "M6 4h12v3H6V4zm-2 5h16v11H4V9zm4 3h3v5H8v-5zm5-2h3v7h-3v-7z",
    memory:
      "M5 4h14a2 2 0 0 1 2 2v12H3V6a2 2 0 0 1 2-2zm2 4v2h10V8H7zm0 4v2h7v-2H7z",
    spark:
      "M12 2l1.8 5.5L19 9.2l-4.2 3 1.5 5.3L12 14.5l-4.3 3 1.5-5.3-4.2-3 5.2-1.7L12 2z",
    empty: "M4 6h16v12H4V6zm2 2v8h12V8H6zm2 2h8v2H8v-2z"
  };

  return (
    <svg className="icon" viewBox="0 0 24 24" aria-hidden="true">
      <path d={paths[name]} />
    </svg>
  );
}
