import { Icon } from "./Icon";

export function EmptyState({
  title,
  desc,
  large = false
}: {
  title: string;
  desc: string;
  large?: boolean;
}) {
  return (
    <div className={`empty-state ${large ? "large" : ""}`}>
      <div className="empty-orb">
        <Icon name="empty" />
      </div>
      <strong>{title}</strong>
      <span>{desc}</span>
    </div>
  );
}
