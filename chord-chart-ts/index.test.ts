import { describe, expect, it } from "vitest";
import { transposeChart, validateChart } from ".";

describe("validateChart", () => {
  it("works", () => {
    expect(validateChart("| C|")).toEqual("| C |");
  });

  it("throws", () => {
    expect(() => validateChart("C")).toThrow(
      "bar line should start with stripe: C"
    );
  });
});

describe("transposeChart", () => {
  it("works", () => {
    expect(transposeChart("| D |", "D", "Gb")).toEqual("| Gb |");
  });

  it("throws", () => {
    expect(() => transposeChart("C", "C", "C")).toThrow(
      "bar line should start with stripe: C"
    );
  });
});
