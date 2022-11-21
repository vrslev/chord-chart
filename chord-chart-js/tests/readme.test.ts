import { expect, it } from "vitest";
import init, { transposeChart, validateChart, ValidationError } from "../";

it("works", async () => {
  await init();

  // that's a valid chart: | *chord-with-bass-note* *chord-without-accidental* | *chord* | *(end of the bar, then new bar ->)*
  // | *chord-with-accidental-and-symbols* |
  expect(validateChart("| A/E E | E | \n| C#m|")).toEqual(
    "| A/E E | E |\n| C#m |"
  );

  // and that's not a valid one: chords without stripes between lines of bars
  expect(() => validateChart("A/E E\nC#m")).toThrow(
    new ValidationError("BarLineShouldStartWithStripe", "A/E E")
  );

  expect(transposeChart("| A/E| E |\n| C#m|", "E", "Db")).toEqual(
    "| Gb/Db | Db |\n| Bbm |"
  );
});
