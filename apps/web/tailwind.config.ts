import { join } from "node:path";
import { skeleton } from "@skeletonlabs/tw-plugin";
import { myCustomTheme } from "./my-custom-theme.ts";
import basePlugin from "./tailwind-base.cjs";
import type { Config } from "tailwindcss";

export default {
  content: [
    "./src/**/*.{html,js,svelte,ts}",
    join(require.resolve("@skeletonlabs/skeleton"), "../**/*.{html,js,svelte,ts}"),
  ],
  darkMode: "class",
  plugins: [
    basePlugin(),
    skeleton({
      themes: {
        custom: [myCustomTheme],
      },
    }),
  ],
} satisfies Config;
