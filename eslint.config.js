// ESLint workspace config — FSD boundary rules + base TypeScript.
// Plugin install and CI integration: feat-4.
// Rule reference: docs/pegin-wiki/10-architecture/project-structure-principles.md §2

import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

/** Resolve a path relative to the workspace root. */
const r = (...parts) => path.resolve(__dirname, ...parts);

// ---------------------------------------------------------------------------
// FSD import-direction zones
// Defined here so feat-4 can drop in eslint-plugin-import without restructuring.
// ---------------------------------------------------------------------------

const sdkSrc = r("packages/sdk/src");
const demoSrc = r("apps/demo-web/src");
const miniSrc = r("apps/mini/src");

/**
 * Build FSD boundary zones for a given src root.
 * Layers (lowest → highest): shared → entities → features → widgets → pages → app
 */
function fsdZones(src) {
  const layers = ["shared", "entities", "features", "widgets", "pages", "app"];
  const zones = [];

  // Lower layers may not import from higher layers
  for (let i = 0; i < layers.length; i++) {
    for (let j = i + 1; j < layers.length; j++) {
      zones.push({
        target: path.join(src, layers[i]),
        from: path.join(src, layers[j]),
        message: `FSD: '${layers[i]}' must not import from '${layers[j]}'`,
      });
    }
  }

  // No cross-slice imports at features or entities layers
  for (const layer of ["features", "entities"]) {
    zones.push({
      target: path.join(src, layer),
      from: path.join(src, layer),
      // eslint-plugin-import handles intra-layer cross-slice via allowSameFolder:false
      message: `FSD: cross-slice imports within '${layer}' are forbidden — use entity stores to communicate`,
    });
  }

  return zones;
}

// Cross-app isolation: apps must not import each other
const crossAppZones = [
  {
    target: r("apps/mini/src"),
    from: r("apps/demo-web/src"),
    message: "Cross-app import: apps/mini must not import from apps/demo-web",
  },
  {
    target: r("apps/demo-web/src"),
    from: r("apps/mini/src"),
    message: "Cross-app import: apps/demo-web must not import from apps/mini",
  },
];

// ---------------------------------------------------------------------------
// Config export (flat config format — ESLint ≥ 9)
// Requires: eslint-plugin-import (feat-4 installs this)
// ---------------------------------------------------------------------------

export default [
  {
    // Applied to all TypeScript source files
    files: ["packages/*/src/**/*.{ts,tsx}", "apps/*/src/**/*.{ts,tsx}"],
    rules: {
      // FSD boundary enforcement — activate once eslint-plugin-import is installed (feat-4)
      // "import/no-restricted-paths": [
      //   "error",
      //   {
      //     zones: [
      //       ...fsdZones(sdkSrc),
      //       ...fsdZones(demoSrc),
      //       ...fsdZones(miniSrc),
      //       ...crossAppZones,
      //     ],
      //   },
      // ],
    },
  },
];

// The zones are exported for feat-4 to reference when wiring up the plugin.
export { fsdZones, crossAppZones, sdkSrc, demoSrc, miniSrc };
