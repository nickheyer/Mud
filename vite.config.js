import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

/** @type {import('vite').UserConfig} */
export default defineConfig(async () => ({
  plugins: [sveltekit()],
  clearScreen: false,
  optimizeDeps: {
    exclude: [
      // "svelte-codemirror-editor",
      // "codemirror",
      // "@codemirror/legacy-modes",
      // "@codemirror/language"
    ]
  },
  server: {
    port: 5173,
    strictPort: true,
    host: host || 'localhost', // Default to 'localhost' for development consistency
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 5174,
        }
      : undefined,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
}));
