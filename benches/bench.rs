use std::{collections::HashMap, hash::Hash};

use criterion::{black_box, criterion_group, criterion_main, Criterion};


#[inline]
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n-1) + fibonacci(n-2),
    }
}

#[inline]
fn better_fibonacci(n: u64) -> u64 {
  let mut table:HashMap<u64, u64> = HashMap::new();
  fn _better_fibo(n: u64, memo: &HashMap<u64, u64>)-> u64 {

    if memo.contains_key(&n){
      return *memo.get(&n).unwrap();
    }
    else {
      match n {
          0 => 1,
          1 => 1,
          n => _better_fibo(n - 1, memo) + fibonacci(n - 2)
      }
    }

  }
  return 0;
}


fn better_better_fibo(n: u64) -> u64 {
  let mut a = 0;
  let mut b = 1;

  match n {
      0 => b,
      _ => {
          for _ in 0..n {
              let c = a + b;
              a = b;
              b = c;
          }
          b
      }
  }
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    c.bench_function("fib 20", |b| b.iter(|| better_fibonacci(black_box(20))));
    c.bench_function("fib 20", |b| b.iter(|| better_better_fibo(black_box(20))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);