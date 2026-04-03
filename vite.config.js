import { paraglideVitePlugin } from '@inlang/paraglide-js';
import tailwindcss from '@tailwindcss/vite';
import { defineConfig } from 'vitest/config';
import { playwright } from '@vitest/browser-playwright';
import { sveltekit } from '@sveltejs/kit/vite';

const host = typeof process !== 'undefined' ? process.env?.TAURI_DEV_HOST : undefined;

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    paraglideVitePlugin({ project: './project.inlang', outdir: './src/paraglide' }),
    tailwindcss(),
    sveltekit()
  ],

  // Optimize bundle size and loading
  build: {
    rolldownOptions: {
      external: ['@vinejs/vine'],
      output: {
        // Split large icon library into separate chunk
        manualChunks: (id) => (id.includes('lucide-svelte') ? 'lucide' : undefined)
      }
    },
    // Reduce chunk size warning threshold
    chunkSizeWarningLimit: 600
  },

  resolve: {
    // Enable native TypeScript path resolution (new in Vite 8, replaces third-party tsconfigPaths plugins)
    tsconfigPaths: true
  },

  // Vite options tailored for Tauri development and only applied in  or
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: 'ws',
          host,
          port: 1421
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ['**/src-tauri/**']
    }
  },

  test: {
    expect: { requireAssertions: true },

    projects: [
      {
        extends: './vite.config.js',

        test: {
          name: 'client',

          browser: {
            enabled: true,
            provider: playwright(),
            instances: [{ browser: 'chromium', headless: true }]
          },

          include: ['src/**/*.svelte.{test,spec}.{js,ts}'],
          exclude: ['src/lib/server/**']
        }
      },

      {
        extends: './vite.config.js',

        test: {
          name: 'server',
          environment: 'node',
          include: ['src/**/*.{test,spec}.{js,ts}'],
          exclude: ['src/**/*.svelte.{test,spec}.{js,ts}']
        }
      }
    ]
  }
});
