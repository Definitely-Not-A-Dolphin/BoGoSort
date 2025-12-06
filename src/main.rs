use std::env;
use std::time::Instant;

fn factorial(n: i32) -> i64 {
  let mut prod: i64 = 1;
  for i in 1..=n {
    prod *= i as i64;
  }
  prod
}

fn inv_factorial(n: i32) -> f64 {
  let mut prod = 1f64;
  for i in 1..=n {
    prod /= i as f64;
  }
  prod
}

fn prob_sorted_at(prob_sorted: f64, tries: i64) -> f64 {
  (1f64 - prob_sorted).powf(tries as f64 - 1f64) * prob_sorted
}

fn tries_required_exceed_prob(prob_sorted: f64, prob: f64) -> i64 {
  let mut sum = 0f64;
  let mut i = 1i64;
  loop {
    let prob_sorted_at_i = prob_sorted_at(prob_sorted, i);
    if prob_sorted_at_i == 0f64 {
      return -1i64;
    }
    sum += prob_sorted_at_i;
    if sum >= prob {
      return i;
    };
    i += 1;
  }
}

fn prob_sorted_after_n_iterations(array_length: i32, iterations: i64) -> f64 {
  let mut acc_prob = 0f64;
  let prob_sorted = inv_factorial(array_length);
  for k in 1..=iterations {
    let prob_sorted_at_k = prob_sorted_at(prob_sorted, k);
    if prob_sorted_at_k == 1f64 {
      return 1f64;
    };
    if prob_sorted_at_k == 0f64 {
      return -1f64;
    }
    acc_prob += prob_sorted_at_k;
  }
  acc_prob
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let arg1 = &args[1];

  if arg1.starts_with("-n=") {
    // this means we want to calculate n -> p
    let n = match arg1.clone().split_off(3).parse::<i64>() {
      Ok(n) => n,
      Err(error) => panic!("Invald argument given! {}", error),
    };

    for i in 1.. {
      let starting_time = Instant::now();
      let required_iterations = prob_sorted_after_n_iterations(i, n);
      if required_iterations == -1f64 {
        panic!("0 reached; Infinite loop entered");
      };
      print!("Array length: {}\n", i);
      print!(
        "Prob sorted after {} iterations: {}\n",
        n, required_iterations
      );
      print!(
        "Took \x1b[41m{}ms\x1b[0m\n\n",
        starting_time.elapsed().as_millis()
      );
    }
  } else if arg1.starts_with("-p=") {
    // this means we want to calculate p -> n
    let p = match arg1.clone().split_off(3).parse::<f64>() {
      Ok(p) => p,
      Err(error) => panic!("Invald argument given! {}", error),
    };

    for i in 1.. {
      let starting_time = Instant::now();
      let required_tries = tries_required_exceed_prob(inv_factorial(i), p);
      if required_tries == -1i64 {
        panic!("0 reached; Infinite loop entered");
      }
      print!("Tries to sort {}: \x1b[44m{}\x1b[0m,\n", i, required_tries);
      print!("      while {}! = \x1b[42m{}\x1b[0m\n", i, factorial(i));
      print!(
        "Took \x1b[41m{}ms\x1b[0m\n\n",
        starting_time.elapsed().as_millis()
      );
    }
  }
}
