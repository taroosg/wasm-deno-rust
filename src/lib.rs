use wasm_bindgen::prelude::*;

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

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}
