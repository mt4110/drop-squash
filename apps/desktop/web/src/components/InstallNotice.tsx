type InstallNoticeProps = {
  appPath: string;
  onDismiss: () => void;
};

export function InstallNotice({ appPath, onDismiss }: InstallNoticeProps) {
  return (
    <section className="install-notice" aria-label="Install notice">
      <span title={appPath}>
        DropSquash is running from the disk image. Move it to Applications before regular use.
      </span>
      <button type="button" onClick={onDismiss}>OK</button>
    </section>
  );
}
