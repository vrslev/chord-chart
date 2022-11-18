import { defineConfig } from "vitest/config";

export default defineConfig({
  assetsInclude: ["**/*.wasm"],
  build: {
    lib: {
      entry: ["index.ts"],
      formats: ["es"],
    },
  },
});
