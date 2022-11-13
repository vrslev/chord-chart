import { describe, expect, it } from "vitest";
import { tranposeChart, validateChart } from ".";

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

describe("tranposeChart", () => {
  it("works", () => {
    expect(tranposeChart("| D |", "D", "Gb")).toEqual("| Gb |");
  });

  it("throws", () => {
    expect(() => tranposeChart("C", "C", "C")).toThrow(
      "bar line should start with stripe: C"
    );
  });
});
