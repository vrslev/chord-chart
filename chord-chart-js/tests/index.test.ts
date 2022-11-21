import { beforeAll, describe, expect, it } from "vitest";
import init, { transposeChart, validateChart, ValidationError } from "../";

beforeAll(async () => {
  await init();
});

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
