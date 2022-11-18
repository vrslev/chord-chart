import { describe, expect, it } from "vitest";
import { transposeChart, validateChart, ValidationError } from ".";

describe("validateChart", () => {
  it("works", () => {
    expect(validateChart("| C|")).toEqual("| C |");
  });

  it("throws", () => {
    expect(() => validateChart("C")).toThrowError(
      new ValidationError("BarLineShouldStartWithStripe", "C")
    );
  });
});

describe("transposeChart", () => {
  it("works", () => {
    expect(transposeChart("| D |", "D", "Gb")).toEqual("| Gb |");
  });

  it("throws", () => {
    expect(() => transposeChart("C", "C", "C")).toThrow(
      new ValidationError("BarLineShouldStartWithStripe", "C")
    );
  });
});
