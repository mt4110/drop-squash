type HelpPopoverProps = {
  onClose: () => void;
};

export function HelpPopover({ onClose }: HelpPopoverProps) {
  return (
    <div className="help-backdrop" role="presentation" onClick={onClose}>
      <aside
        aria-label="Quick tips"
        aria-modal="true"
        className="help-popover"
        onClick={(event) => event.stopPropagation()}
        role="dialog"
      >
        <strong>Quick tips</strong>
        <p>Drop a recording or choose one. The original stays where it is.</p>
        <p>Auto chooses a share-friendly size. Choose a size when you need a specific limit.</p>
        <button type="button" onClick={onClose}>Done</button>
      </aside>
    </div>
  );
}
