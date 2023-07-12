import { sveltekit } from "@sveltejs/kit/vite";
import { defineConfig } from "vite";
import Icons from "unplugin-icons/vite";

export default defineConfig({
  plugins: [
    sveltekit(),
    Icons({
      compiler: "svelte",
    }),
  ],
  server: {
    port: 3000,
    strictPort: false,
  },
  preview: {
    port: 8080,
    strictPort: false,
  },
});
