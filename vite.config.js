import { defineConfig } from "vite";
import { sveltekit } from "@sveltejs/kit/vite";
import * as path from 'path';

// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

/** @type {import('vite').UserConfig} */
export default defineConfig(async () => ({
  plugins: [sveltekit()],
  clearScreen: false,
  optimizeDeps: {     
    include: [
      'codemirror',
      '@codemirror/state',
      '@codemirror/view',
      '@codemirror/language',
      '@codemirror/commands',
      '@codemirror/autocomplete',
      '@codemirror/lint'
    ],
  },
  resolve: {
      alias: {
          'codemirror': path.resolve(
            __dirname,
            './node_modules/codemirror/dist/index.cjs'
          ),
          '@codemirror/state': path.resolve(
            __dirname,
            './node_modules/@codemirror/state/dist/index.cjs'
          ),
          '@codemirror/view': path.resolve(
            __dirname,
            './node_modules/@codemirror/view/dist/index.cjs'
          ),
          '@codemirror/language': path.resolve(
            __dirname,
            './node_modules/@codemirror/language/dist/index.cjs'
          ),
          '@codemirror/commands': path.resolve(
            __dirname,
            './node_modules/@codemirror/commands/dist/index.cjs'
          )
        }
  }
}));
