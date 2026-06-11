// ESLint workspace config — FSD boundary rules + TypeScript strict.
// Rule reference: docs/pegin-wiki/10-architecture/project-structure-principles.md §2

import path from 'node:path'
import { fileURLToPath } from 'node:url'
import tseslint from 'typescript-eslint'
import pluginImport from 'eslint-plugin-import'
import prettierConfig from 'eslint-config-prettier'

const __dirname = path.dirname(fileURLToPath(import.meta.url))

/** Resolve a path relative to the workspace root. */
const r = (...parts) => path.resolve(__dirname, ...parts)

// ---------------------------------------------------------------------------
// FSD import-direction zones
// Layers (lowest → highest): shared → entities → features → widgets → pages → app
// ---------------------------------------------------------------------------

const sdkSrc = r('packages/sdk/src')
const demoSrc = r('apps/demo-web/src')
const miniSrc = r('apps/mini/src')

function fsdZones(src) {
  const layers = ['shared', 'entities', 'features', 'widgets', 'pages', 'app']
  const zones = []

  for (let i = 0; i < layers.length; i++) {
    for (let j = i + 1; j < layers.length; j++) {
      zones.push({
        target: path.join(src, layers[i]),
        from: path.join(src, layers[j]),
        message: `FSD: '${layers[i]}' must not import from '${layers[j]}'`,
      })
    }
  }

  for (const layer of ['features', 'entities']) {
    zones.push({
      target: path.join(src, layer),
      from: path.join(src, layer),
      message: `FSD: cross-slice imports within '${layer}' are forbidden`,
    })
  }

  return zones
}

const crossAppZones = [
  {
    target: r('apps/mini/src'),
    from: r('apps/demo-web/src'),
    message: 'Cross-app import: apps/mini must not import from apps/demo-web',
  },
  {
    target: r('apps/demo-web/src'),
    from: r('apps/mini/src'),
    message: 'Cross-app import: apps/demo-web must not import from apps/mini',
  },
]

// ---------------------------------------------------------------------------
// Flat config (ESLint ≥ 9)
// ---------------------------------------------------------------------------

export default tseslint.config(
  // TypeScript strict-type-checked preset, scoped to TS source files
  {
    extends: tseslint.configs.strictTypeChecked,
    files: ['packages/*/src/**/*.{ts,tsx}', 'apps/*/src/**/*.{ts,tsx}'],
    languageOptions: {
      parserOptions: {
        project: true,
        tsconfigRootDir: __dirname,
      },
    },
    rules: {
      '@typescript-eslint/no-explicit-any': 'error',
      '@typescript-eslint/no-unsafe-assignment': 'error',
      '@typescript-eslint/no-unsafe-call': 'error',
      '@typescript-eslint/no-unsafe-member-access': 'error',
      '@typescript-eslint/no-unsafe-return': 'error',
      '@typescript-eslint/no-unsafe-argument': 'error',
      // _prefixed params are intentionally unused (TS convention for stubs)
      '@typescript-eslint/no-unused-vars': ['error', { argsIgnorePattern: '^_', varsIgnorePattern: '^_' }],
      // numbers in template literals are a common safe pattern
      '@typescript-eslint/restrict-template-expressions': ['error', { allowNumber: true }],
      // logging goes through the logger module — see wiki: logging-strategy
      'no-console': 'error',
    },
  },

  // Node scripts (demo CLI, smoke test): same no-console rule; they use the
  // winston logger for diagnostics and process.stdout/stderr for program output.
  {
    files: ['crates/**/*.mjs'],
    rules: {
      'no-console': 'error',
    },
  },

  // FSD boundary enforcement
  {
    files: ['packages/*/src/**/*.{ts,tsx}', 'apps/*/src/**/*.{ts,tsx}'],
    plugins: { import: pluginImport },
    rules: {
      'import/no-restricted-paths': [
        'error',
        {
          zones: [...fsdZones(sdkSrc), ...fsdZones(demoSrc), ...fsdZones(miniSrc), ...crossAppZones],
        },
      ],
    },
  },

  // Disable formatting rules that conflict with Prettier
  prettierConfig,
)

export { fsdZones, crossAppZones, sdkSrc, demoSrc, miniSrc }
