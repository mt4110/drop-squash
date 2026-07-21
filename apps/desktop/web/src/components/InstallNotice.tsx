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
    ? "Applications copy is open. Eject this disk image copy, or quit it manually."
    : didCopyToApplications
      ? "DropSquash was copied to Applications. Open that copy, then eject the disk image."
      : "DropSquash is running from the disk image. Move it to Applications before regular use.";
  const messageMain = didOpenInstalledApp
    ? "Applications版を開きました"
    : didCopyToApplications
      ? "Applicationsへコピーしました"
      : "ディスクイメージから起動しています";
  const statusMain = didOpenInstalledApp
    ? "このコピーは閉じて大丈夫です"
    : didCopyToApplications
      ? "Applications版へ切り替えてください"
      : "通常利用の前に移動してください";
  const title = installedPath ?? appPath;
  const isBusy = isMoving || isOpeningInstalledApp || isQuittingAfterEject;

  return (
    <section className="install-notice" aria-label="インストール案内 / Install notice">
      <span title={title}>
        <span className="ui-copy">
          <span className="ui-main">{messageMain}</span>
          <span className="ui-main ui-main-inline">{statusMain}</span>
          <span className="ui-sub">{message}</span>
        </span>
      </span>
      <div>
        {!didCopyToApplications && (
          <button type="button" disabled={isBusy} onClick={onMove}>
            <span className="ui-copy ui-copy-center">
              <span className="ui-main">{isMoving ? "移動中" : "移動"}</span>
              <span className="ui-sub">{isMoving ? "Moving..." : "Move"}</span>
            </span>
          </button>
        )}
        {didCopyToApplications && (
          <button type="button" disabled={isBusy} onClick={onOpenInstalledApp}>
            <span className="ui-copy ui-copy-center">
              <span className="ui-main">{isOpeningInstalledApp ? "起動中" : "開く"}</span>
              <span className="ui-sub">{isOpeningInstalledApp ? "Opening..." : "Open"}</span>
            </span>
          </button>
        )}
        {didOpenInstalledApp && (
          shouldOfferInstallerVolumeEject && (
            <button type="button" disabled={isBusy} onClick={onQuitAfterInstallerVolumeEject}>
              <span className="ui-copy ui-copy-center">
                <span className="ui-main">{isQuittingAfterEject ? "取り出し中" : "取り出して終了"}</span>
                <span className="ui-sub">{isQuittingAfterEject ? "Ejecting..." : "Eject & Quit"}</span>
              </span>
            </button>
          )
        )}
        {didOpenInstalledApp && (
          <button type="button" disabled={isBusy} onClick={onQuitCurrentApp}>
            <span className="ui-copy ui-copy-center">
              <span className="ui-main">終了</span>
              <span className="ui-sub">Quit</span>
            </span>
          </button>
        )}
        <button type="button" disabled={isBusy} onClick={onDismiss}>
          <span className="ui-copy ui-copy-center">
            <span className="ui-main">閉じる</span>
            <span className="ui-sub">OK</span>
          </span>
        </button>
      </div>
    </section>
  );
}
