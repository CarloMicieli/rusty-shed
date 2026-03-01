import prettier from 'eslint-config-prettier';
import { fileURLToPath } from 'node:url';
import { includeIgnoreFile } from '@eslint/compat';
import js from '@eslint/js';
import svelte from 'eslint-plugin-svelte';
import { defineConfig } from 'eslint/config';
import globals from 'globals';
import ts from 'typescript-eslint';
import svelteConfig from './svelte.config.js';

const gitignorePath = fileURLToPath(new URL('./.gitignore', import.meta.url));

const allowDefaultProject = [
  'eslint.config.js',
  'svelte.config.js',
  'vitest.config.ts',
  'tools/*.js',
  'src/lib/types/*.d.ts'
];

export default defineConfig(
  includeIgnoreFile(gitignorePath),
  {
    // Ignore auto-generated Tauri bindings and build output
    ignores: [
      'eslint.config.js',
      'src/lib/bindings.ts',
      'build/**',
      'specs/**',
      'src/lib/paraglide/server.js',
      'src/lib/paraglide/messages.js',
      'src/lib/paraglide/registry.js',
      'src/lib/paraglide/runtime.js',
      'src/lib/paraglide/messages/**',
      'src/paraglide/messages.js',
      'src/paraglide/registry.js',
      'src/paraglide/runtime.js',
      'src/paraglide/server.js',
      'src/paraglide/messages/**'
    ]
  },
  js.configs.recommended,
  ...ts.configs.recommended,
  ...svelte.configs.recommended,
  prettier,
  ...svelte.configs.prettier,
  {
    languageOptions: {
      globals: { ...globals.browser, ...globals.node },
      parserOptions: {
        // projectService: true is the modern way to handle type-aware linting
        projectService: {
          allowDefaultProject
        },
        extraFileExtensions: ['.svelte'],
        svelteConfig
      }
    },

    rules: {
      // typescript-eslint strongly recommend that you do not use the no-undef lint rule on TypeScript projects.
      // see: https://typescript-eslint.io/troubleshooting/faqs/eslint/#i-get-errors-from-the-no-undef-rule-about-global-variables-not-being-defined-even-though-there-are-no-typescript-errors
      'no-undef': 'off',
      // Disable for Tauri apps where base path handling is not needed
      'svelte/no-navigation-without-resolve': 'off',
      '@typescript-eslint/no-explicit-any': 'error',
      '@typescript-eslint/no-unsafe-assignment': 'error',
      '@typescript-eslint/no-unsafe-member-access': 'error',
      '@typescript-eslint/no-unsafe-call': 'error',
      '@typescript-eslint/no-unsafe-return': 'error',
      // Allow unused variables that start with _
      '@typescript-eslint/no-unused-vars': [
        'error',
        { argsIgnorePattern: '^_', varsIgnorePattern: '^_' }
      ]
    }
  },
  {
    files: ['**/*.svelte', '**/*.svelte.ts', '**/*.svelte.js'],

    languageOptions: {
      parserOptions: {
        projectService: {
          allowDefaultProject
        },
        extraFileExtensions: ['.svelte'],
        parser: ts.parser,
        svelteConfig
      }
    },

    rules: {
      '@typescript-eslint/no-unsafe-assignment': 'off',
      '@typescript-eslint/no-unsafe-member-access': 'off',
      '@typescript-eslint/no-unsafe-call': 'off',
      '@typescript-eslint/no-unsafe-return': 'off'
    }
  },
  {
    files: ['**/__tests__/**', '**/*.test.ts', '**/*.test.svelte', '**/*.spec.ts'],

    rules: {
      '@typescript-eslint/no-explicit-any': 'off',
      '@typescript-eslint/no-unsafe-assignment': 'off',
      '@typescript-eslint/no-unsafe-member-access': 'off',
      '@typescript-eslint/no-unsafe-call': 'off',
      '@typescript-eslint/no-unsafe-return': 'off'
    }
  },
  {
    files: ['**/*.d.ts'],

    rules: {
      '@typescript-eslint/no-unsafe-return': 'off'
    }
  },
  {
    files: ['tools/**'],

    rules: {
      '@typescript-eslint/no-explicit-any': 'off',
      '@typescript-eslint/no-unsafe-assignment': 'off',
      '@typescript-eslint/no-unsafe-member-access': 'off',
      '@typescript-eslint/no-unsafe-call': 'off',
      '@typescript-eslint/no-unsafe-return': 'off'
    }
  }
);
