import { initSync, transposeChart, validateChart } from "./pkg/chord_chart_js";
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

function decodeBase64(value: string) {
  let buf;
  const isNode =
    typeof process !== "undefined" &&
    process.versions != null &&
    process.versions.node != null;
  const newValue = value.replace("data:application/wasm;base64,", "");

  if (isNode) {
    buf = Buffer.from(newValue, "base64");
  } else {
    let raw = globalThis.atob(newValue);
    let rawLength = raw.length;
    buf = new Uint8Array(new ArrayBuffer(rawLength));
    for (var i = 0; i < rawLength; i++) {
      buf[i] = raw.charCodeAt(i);
    }
  }

  return buf;
}

initSync(decodeBase64(wasm as unknown as string));

export { validateChart, transposeChart };
