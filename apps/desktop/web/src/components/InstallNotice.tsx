type InstallNoticeProps = {
  appPath: string;
  didCopyToApplications: boolean;
  installedPath?: string;
  isMoving: boolean;
  onDismiss: () => void;
  onMove: () => void;
};

export function InstallNotice({
  appPath,
  didCopyToApplications,
  installedPath,
  isMoving,
  onDismiss,
  onMove,
}: InstallNoticeProps) {
  const message = didCopyToApplications
    ? "DropSquash was copied to Applications. Open that copy before ejecting the disk image."
    : "DropSquash is running from the disk image. Move it to Applications before regular use.";
  const title = installedPath ?? appPath;

  return (
    <section className="install-notice" aria-label="Install notice">
      <span title={title}>{message}</span>
      <div>
        {!didCopyToApplications && (
          <button type="button" disabled={isMoving} onClick={onMove}>
            {isMoving ? "Moving..." : "Move"}
          </button>
        )}
        <button type="button" disabled={isMoving} onClick={onDismiss}>OK</button>
      </div>
    </section>
  );
}
