export function normalizedLicenseKey(value: string) {
  return value.trim();
}

export function canSubmitLicenseKey(value: string) {
  return normalizedLicenseKey(value).length > 0;
}
