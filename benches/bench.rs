use std::vec;
use std::{collections::HashMap, hash::Hash, collections::HashSet};

use criterion::{black_box, criterion_group, criterion_main, Criterion};


use rand::RngCore;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::rngs::SmallRng;

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





fn sorted_set_bench(mut s: HashSet<u32>, n:Vec<u32>){
  for x in n.iter() {
    s.insert(*x);
  }
}


pub fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    // c.bench_function("fib 20", |b| b.iter(|| better_fibonacci(black_box(20))));
    // c.bench_function("fib 20", |b| b.iter(|| better_better_fibo(black_box(20))));
    let mut rng = SmallRng::from_entropy();
    let v_arr:[u32; 10e2 as usize] = [0; 10e2 as usize];
    let mut v_rand = v_arr.map(|_| rng.next_u32());
    v_rand.sort();
    let mut v = Vec::from(v_rand);
    let mut s = HashSet::from(v_rand);


    let n_arr:[u32; 10 as usize] = [0; 10 as usize];
    let n_rand = n_arr.map(|_| rng.next_u32());
    let n = Vec::from(n_rand);

    c.bench_function("sorted_set_bench", |b| b.iter(|| sorted_set_bench(s.clone(), n.clone())));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);