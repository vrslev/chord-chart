import { defineConfig } from "vitest/config";

export default defineConfig({
  resolve: {
    alias: {
      "@index": "../index",
    },
  },
  assetsInclude: ["**/*.wasm"],
  build: {
    lib: {
      entry: ["index.ts"],
      formats: ["es"],
    },
  },
});
