import initWasm, {
  initSync as initWasmSync,
  transposeChart,
  validateChart,
} from "./pkg/chord_chart_js";
import wasm from "./pkg/chord_chart_js_bg.wasm";

type ErrorType =
  | "NoNatural"
  | "InvalidNatural"
  | "InvalidNote"
  | "BarLineShouldStartWithStripe"
  | "BarLineShouldEndWithStripe";
export class ValidationError extends Error {
  type: ErrorType;
  value?: string;

  constructor(type_: ErrorType, value?: string) {
    super();
    this.type = type_;
    this.value = value;
  }
}

export default async function init(): Promise<void> {
  const isNode =
    typeof process !== "undefined" &&
    process.versions != null &&
    process.versions.node != null;
  if (isNode) {
    initWasmSync(
      Buffer.from(
        (wasm as unknown as string).replace(
          "data:application/wasm;base64,",
          ""
        ),
        "base64"
      )
    );
  } else {
    await initWasm(wasm as unknown as string);
  }
}

export { validateChart, transposeChart };
