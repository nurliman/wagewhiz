import * as crypto from "node:crypto";
import { generateKeys } from "paseto-ts/v4";

console.log(
  generateKeys("public", {
    format: "paserk",
    getRandomValues: crypto.webcrypto.getRandomValues.bind(crypto.webcrypto),
  }),
);
