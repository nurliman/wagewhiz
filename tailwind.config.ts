import { Config } from "tailwindcss";
import flowbitePlugin from "flowbite/plugin";

export default {
  content: [
    "./src/**/*.{html,js,svelte,ts}",
    "./node_modules/flowbite-svelte/**/*.{html,js,svelte,ts}",
  ],
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        primary: {
          "50": "#edfaff",
          "100": "#d6f1ff",
          "200": "#b5e9ff",
          "300": "#83ddff",
          "400": "#48c7ff",
          "500": "#1ea8ff",
          "600": "#068aff",
          "700": "#0070f3",
          "800": "#085ac5",
          "900": "#0d4e9b",
          "950": "#0e305d",
        },
      },
    },
  },
  plugins: [flowbitePlugin],
} satisfies Config;
