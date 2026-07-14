type InstallNoticeProps = {
  appPath: string;
  didCopyToApplications: boolean;
  installedPath?: string;
  isMoving: boolean;
  isOpeningInstalledApp: boolean;
  onDismiss: () => void;
  onMove: () => void;
  onOpenInstalledApp: () => void;
};

export function InstallNotice({
  appPath,
  didCopyToApplications,
  installedPath,
  isMoving,
  isOpeningInstalledApp,
  onDismiss,
  onMove,
  onOpenInstalledApp,
}: InstallNoticeProps) {
  const message = didCopyToApplications
    ? "DropSquash was copied to Applications. Open that copy, then eject the disk image."
    : "DropSquash is running from the disk image. Move it to Applications before regular use.";
  const title = installedPath ?? appPath;
  const isBusy = isMoving || isOpeningInstalledApp;

  return (
    <section className="install-notice" aria-label="Install notice">
      <span title={title}>{message}</span>
      <div>
        {!didCopyToApplications && (
          <button type="button" disabled={isBusy} onClick={onMove}>
            {isMoving ? "Moving..." : "Move"}
          </button>
        )}
        {didCopyToApplications && (
          <button type="button" disabled={isBusy} onClick={onOpenInstalledApp}>
            {isOpeningInstalledApp ? "Opening..." : "Open"}
          </button>
        )}
        <button type="button" disabled={isBusy} onClick={onDismiss}>OK</button>
      </div>
    </section>
  );
}
