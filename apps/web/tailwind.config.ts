import { join } from "node:path";
import skeletonPlugin from "@skeletonlabs/skeleton/tailwind/skeleton.cjs";
import basePlugin from "./tailwind-base.cjs";
import type { Config } from "tailwindcss";

export default {
  content: [
    "./src/**/*.{html,js,svelte,ts}",
    join(require.resolve("@skeletonlabs/skeleton"), "../**/*.{html,js,svelte,ts}"),
  ],
  darkMode: "class",
  plugins: [basePlugin(), ...skeletonPlugin()],
} satisfies Config;
