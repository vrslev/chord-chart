import { expect, test } from "vitest";
import { validateChart } from "..";

test("works", async () => {
  expect(validateChart("| C|")).toEqual("| C |");
});
