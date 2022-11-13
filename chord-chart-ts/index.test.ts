import { describe, expect, test } from "vitest";
import { validateChart } from ".";

describe("validateChart", () => {
  test("ok", () => {
    expect(validateChart("| C|")).toEqual("| C |");
  });

  test("throws", () => {
    expect(() => validateChart("C")).toThrow(
      "bar line should start with stripe: C"
    );
  });
});
