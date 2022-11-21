# chord-chart-wasm

This package provides chord chart validation and transposition functionality.

It is written in Rust and has an accompanying [library](https://github.com/vrslev/chord-chart/tree/main/chord-chart-py) in Python. Used in [vrslev/songbook](https://github.com/vrslev/songbook) — lyrics and chords webapp.

## Example

Here's an example tests that describe ins and outs of the available functions:

```js
import { expect, it } from "vitest";
import init, {
  transposeChart,
  validateChart,
  ValidationError,
} from "chord-chart-wasm";

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
```

## Installation

Just `npm i chord-chart-wasm`. Works on both Node and browser.

## Development

- `npm i` to install deps.
- `npm run build` to clean up, build wasm artifacts and package bundle.
- `npm test` to test the package.
- `npm run build-test` to build and test the package.
- `npm run check-types` to run tsc.

Also make sure to install pre-commit hooks (`pre-commit install` from the repository root).
