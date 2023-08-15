import { Config } from "tailwindcss";
import basePlugin from "./tailwind-base.cjs";
import skeletonPlugin from "@skeletonlabs/skeleton/tailwind/skeleton.cjs";

export default {
  content: [
    "./src/**/*.{html,js,svelte,ts}",
    "./node_modules/@skeletonlabs/skeleton/**/*.{html,js,svelte,ts}",
  ],
  darkMode: "class",
  plugins: [basePlugin(), ...skeletonPlugin()],
} satisfies Config;
