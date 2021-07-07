import { fib } from "./pkg/wasm_deno_rust.js";

for (let i = 0; i < 46; i++) {
  console.log(fib(i).toString()); // BigIntをそのまま出力すると末尾にnが付くのでtoStringした
}
