import { expect, it } from "vitest";
import { transposeChart, validateChart, ValidationError } from "../dist";

it("works", () => {
  expect(validateChart("| A/E E | E | \n| C#m|")).toEqual(
    "| A/E E | E |\n| C#m |"
  );

  expect(() => validateChart("A/E E\nC#m")).toThrow(
    new ValidationError("BarLineShouldStartWithStripe", "A/E E")
  );

  expect(transposeChart("| A/E| E |\n| C#m|", "E", "Db")).toEqual(
    "| Gb/Db | Db |\n| Bbm |"
  );
});
