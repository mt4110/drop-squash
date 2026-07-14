type InstallNoticeProps = {
  appPath: string;
  didCopyToApplications: boolean;
  didOpenInstalledApp: boolean;
  installedPath?: string;
  isQuittingAfterEject: boolean;
  isMoving: boolean;
  isOpeningInstalledApp: boolean;
  shouldOfferInstallerVolumeEject: boolean;
  onDismiss: () => void;
  onQuitAfterInstallerVolumeEject: () => void;
  onMove: () => void;
  onOpenInstalledApp: () => void;
  onQuitCurrentApp: () => void;
};

export function InstallNotice({
  appPath,
  didCopyToApplications,
  didOpenInstalledApp,
  installedPath,
  isQuittingAfterEject,
  isMoving,
  isOpeningInstalledApp,
  shouldOfferInstallerVolumeEject,
  onDismiss,
  onQuitAfterInstallerVolumeEject,
  onMove,
  onOpenInstalledApp,
  onQuitCurrentApp,
}: InstallNoticeProps) {
  const message = didOpenInstalledApp
    ? "The Applications copy is open. Eject this disk image copy, or quit it manually."
    : didCopyToApplications
      ? "DropSquash was copied to Applications. Open that copy, then eject the disk image."
      : "DropSquash is running from the disk image. Move it to Applications before regular use.";
  const title = installedPath ?? appPath;
  const isBusy = isMoving || isOpeningInstalledApp || isQuittingAfterEject;

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
        {didOpenInstalledApp && (
          shouldOfferInstallerVolumeEject && (
            <button type="button" disabled={isBusy} onClick={onQuitAfterInstallerVolumeEject}>
              {isQuittingAfterEject ? "Ejecting..." : "Eject & Quit"}
            </button>
          )
        )}
        {didOpenInstalledApp && (
          <button type="button" disabled={isBusy} onClick={onQuitCurrentApp}>Quit</button>
        )}
        <button type="button" disabled={isBusy} onClick={onDismiss}>OK</button>
      </div>
    </section>
  );
}
