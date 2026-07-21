import type { LockedReason } from "./commands.js";

type Copy = {
  message: string;
  submitLabel: string;
};

export function licensePanelCopy(
  isPro: boolean,
  lockedReason?: LockedReason,
): Copy {
  if (isPro) {
    return {
      message: "このMacでは Pro のまま使えます。録画は引き続きローカルのままです / Pro keeps squashing unlocked on this Mac. Your recordings still stay local.",
      submitLabel: "このMacのライセンスを削除 / Forget license on this Mac",
    };
  }
  if (lockedReason === "license-refresh-required") {
    return {
      message: "このMacの Pro を更新するため、一度だけ再認証してください。録画はローカルのままです / Reconnect once to refresh Pro on this Mac. Your recordings still stay local.",
      submitLabel: "Proを再認証 / Refresh Pro",
    };
  }
  return {
    message: "このMacで続けて使うなら Pro を有効化してください。録画はローカルのままです / Unlock Pro once to keep squashing on this Mac. Your recordings still stay local.",
    submitLabel: "Proを有効化 / Unlock Pro",
  };
}
