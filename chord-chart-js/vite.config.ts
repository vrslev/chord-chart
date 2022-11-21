import { defineConfig } from "vitest/config";

export default defineConfig({
  resolve: {
    alias: {
      "@index": "../index",
    },
  },
  define: { "typeof input === 'undefined'": false },
  assetsInclude: ["**/*.wasm"],
  build: {
    lib: {
      entry: ["index.ts"],
      formats: ["es"],
    },
  },
});
