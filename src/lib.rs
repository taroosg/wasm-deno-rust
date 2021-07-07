use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use num_integer::lcm;
use sha3::{Digest, Sha3_256, Keccak256};
use rand::Rng;

fn fib_helper(n: i32, acc1: i64, acc2: i64) -> i64 {
  if n < 1 {
    acc1
  } else {
    fib_helper(n - 1, acc1 + acc2, acc1)
  }
}

#[wasm_bindgen]
pub fn fib(n: i32) -> i64 {
  fib_helper(n, 0, 1)
}

#[wasm_bindgen]
pub fn obfusticate(s: String) -> String {
  (&s).chars().map(|c| {
    match c {
      'A' ..= 'M' | 'a' ..= 'm' => ((c as u8) + 13) as char,
      'N' ..= 'Z' | 'n' ..= 'z' => ((c as u8) - 13) as char,
      _ => c
    }
  }).collect()
}

#[wasm_bindgen]
pub fn lowest_common_denominator(a: i32, b: i32) -> i32 {
  let r = lcm(a, b);
  return r;
}

#[wasm_bindgen]
pub fn sha3_digest(v: Vec<u8>) -> Vec<u8> {
  return Sha3_256::digest(&v).as_slice().to_vec();
}

#[wasm_bindgen]
pub fn keccak_digest(s: &[u8]) -> Vec<u8> {
  return Keccak256::digest(s).as_slice().to_vec();
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
  x: f32,
  y: f32
}

#[derive(Serialize, Deserialize, Debug)]
struct Line {
  points: Vec<Point>,
  valid: bool,
  length: f32,
  desc: String
}

#[wasm_bindgen]
pub fn create_line (p1: &str, p2: &str, desc: &str) -> String {
  let point1: Point = serde_json::from_str(p1).unwrap();
  let point2: Point = serde_json::from_str(p2).unwrap();
  let length = ((point1.x - point2.x) * (point1.x - point2.x) + (point1.y - point2.y) * (point1.y - point2.y)).sqrt();

  let valid = if length == 0.0 { false } else { true };
  let line = Line { points: vec![point1, point2], valid: valid, length: length, desc: desc.to_string() };
  return serde_json::to_string(&line).unwrap();
}

#[wasm_bindgen]
pub fn say(s: &str) -> String {
  let r = String::from("hello ");
  return r + s;
}

fn collatz(n: isize) -> isize {
  match n {
    1 => n,
    n if n % 2 == 0 => collatz(n / 2),
    _ => collatz(3 * n + 1),
  }
}

fn create_rand(min: isize, max: isize) ->isize {
  let mut rng = rand::thread_rng();
  rng.gen_range(min..max)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    for _i in 1..10000 {
      assert_eq!(collatz(create_rand(1,100000000)), 1);
    }
  }
}
