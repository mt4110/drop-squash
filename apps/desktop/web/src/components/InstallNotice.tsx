type InstallNoticeProps = {
  appPath: string;
  isMoving: boolean;
  onDismiss: () => void;
  onMove: () => void;
};

export function InstallNotice({ appPath, isMoving, onDismiss, onMove }: InstallNoticeProps) {
  return (
    <section className="install-notice" aria-label="Install notice">
      <span title={appPath}>
        DropSquash is running from the disk image. Move it to Applications before regular use.
      </span>
      <div>
        <button type="button" disabled={isMoving} onClick={onMove}>
          {isMoving ? "Moving..." : "Move"}
        </button>
        <button type="button" disabled={isMoving} onClick={onDismiss}>OK</button>
      </div>
    </section>
  );
}
