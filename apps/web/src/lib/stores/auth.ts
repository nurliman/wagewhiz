import { persistentAtom } from "@nanostores/persistent";

export const isLoggedIn = persistentAtom<boolean>("isLoggedIn", false, {
  encode: JSON.stringify,
  decode: JSON.parse,
  listen: true,
});

export const accessToken = persistentAtom<string | null>("accessToken", null, {
  encode: JSON.stringify,
  decode: JSON.parse,
  listen: true,
});
