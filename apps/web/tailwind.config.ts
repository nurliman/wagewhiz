import { Config } from "tailwindcss";
import flowbitePlugin from "flowbite/plugin";
import skeletonPlugin from "@skeletonlabs/skeleton/tailwind/skeleton.cjs";

export default {
  content: [
    "./src/**/*.{html,js,svelte,ts}",
    "./node_modules/@skeletonlabs/skeleton/**/*.{html,js,svelte,ts}",
  ],
  darkMode: "class",
  plugins: [flowbitePlugin, ...skeletonPlugin()],
} satisfies Config;
