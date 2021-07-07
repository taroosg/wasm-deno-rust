import {
  create_line,
  fib,
  keccak_digest,
  lowest_common_denominator,
  obfusticate,
  sha3_digest,
} from "./pkg/wasm_deno_rust.js";

for (let i = 0; i < 46; i++) {
  console.log(fib(i).toString()); // BigIntをそのまま出力すると末尾にnが付くのでtoStringした
}

const encoder = new TextEncoder();

console.log(obfusticate("A quick brown fox jumps over the lazy dog"));
console.log(lowest_common_denominator(123, 2));
console.log(sha3_digest(encoder.encode("This is an important message")));
console.log(keccak_digest(encoder.encode("This is an important message")));

var p1 = { x: 1.5, y: 3.8 };
var p2 = { x: 2.5, y: 5.8 };
var line = JSON.parse(
  create_line(JSON.stringify(p1), JSON.stringify(p2), "A thin red line"),
);
console.log(line);
