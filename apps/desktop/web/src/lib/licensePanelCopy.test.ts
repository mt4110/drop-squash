import { licensePanelCopy } from "./licensePanelCopy.js";

function assert(condition: boolean, message: string) {
  if (!condition) throw new Error(message);
}

assert(
  licensePanelCopy(false, "trial-complete").submitLabel === "Proを有効化 / Unlock Pro",
  "trial lock should offer an unlock action",
);
assert(
  licensePanelCopy(false, "license-refresh-required").submitLabel === "Proを再認証 / Refresh Pro",
  "refresh-required lock should offer a refresh action",
);
assert(
  licensePanelCopy(true).message
    === "このMacでは Pro のまま使えます。録画は引き続きローカルのままです / Pro keeps squashing unlocked on this Mac. Your recordings still stay local.",
  "active Pro should reassure the user about the local-only promise",
);
