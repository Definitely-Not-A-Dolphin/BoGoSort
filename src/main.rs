use sap::{Argument, Parser};
use std::sync::mpsc;
use std::{thread, time::Instant};

struct Args {
    p: f64,
    n: u64,
    l_start: u32,
    l_end: u32,
}

fn factorial(n: u32) -> u64 {
    (1..=(n as u64)).product()
}

fn inv_factorial(n: u32) -> f64 {
    1f64 / (factorial(n) as f64)
}

fn prob_sorted_at(prob_sorted: f64, tries: u64) -> f64 {
    (1f64 - prob_sorted).powf((tries - 1) as f64) * prob_sorted
}

fn tries_required_exceed_prob(prob_sorted: f64, prob: f64) -> u64 {
    let mut sum = 0f64;
    for i in 1.. {
        let prob_sorted_at_i = prob_sorted_at(prob_sorted, i);
        if prob_sorted_at_i == 0f64 {
            return 0u64;
        }
        sum += prob_sorted_at_i;
        if sum >= prob {
            return i;
        };
    }

    0 // this will never return but otherwise rust wil complain
}

fn prob_sorted_after_n_iterations(array_length: u32, iterations: u64) -> f64 {
    let prob_sorted = inv_factorial(array_length);
    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        let mut acc_prob_new = 0f64;
        for k in (2..=iterations).step_by(2) {
            acc_prob_new += prob_sorted_at(prob_sorted, k);
        }
        tx.send(acc_prob_new).unwrap();
    });

    let mut acc_prob = 0f64;
    for k in (1..iterations).step_by(2) {
        acc_prob += prob_sorted_at(prob_sorted, k);
    }

    handle.join().unwrap();

    let received = rx.recv().unwrap();

    acc_prob + received
}

fn main() {
    let mut parser = Parser::from_env().unwrap();
    let mut args = Args {
        p: -1f64,
        n: 0,
        l_start: 1,
        l_end: 1,
    };

    while let Some(arg) = parser.forward().unwrap() {
        match arg {
            Argument::Long("p") => {
                if let Some(p) = parser.value() {
                    args.p = match p.parse::<f64>() {
                        Ok(p) => p,
                        Err(e) => panic!("Invalid argument for p: {}", e),
                    };
                }
            }
            Argument::Long("n") => {
                if let Some(n) = parser.value() {
                    args.n = match n.parse::<u64>() {
                        Ok(n) => n,
                        Err(e) => panic!("Invalid argument for p: {}", e),
                    };
                }
            }
            Argument::Long("l-start") => {
                if let Some(l) = parser.value() {
                    args.l_start = match l.parse::<u32>() {
                        Ok(l) => l,
                        Err(e) => panic!("Invalid argument for p: {}", e),
                    };
                }
            }
            Argument::Long("l-end") => {
                if let Some(l) = parser.value() {
                    args.l_end = l.parse::<u32>().unwrap();
                }
            }
            _ => {}
        }
    }

    if !(args.n == 0 || args.p == -1f64) {
        panic!("Use either n or p, not both!");
    };

    if args.l_start > args.l_end {
        panic!("Invalid range: start value is greater than ending value");
    }

    let l_range = if args.l_end == 1 {
        args.l_start..=216
    } else {
        args.l_start..=args.l_end
    };

    if args.n != 0 {
        // this means we want to calculate n -> p
        for i in l_range {
            let starting_time = Instant::now();
            let required_iterations = prob_sorted_after_n_iterations(i, args.n);
            if required_iterations == -1f64 {
                panic!("0 reached; Infinite loop entered");
            };
            println!("Array length: {}", i);
            println!(
                "Prob sorted after {} iterations: {}",
                args.n, required_iterations
            );
            print!(
                "Took \x1b[41m{}μs\x1b[0m\n\n",
                starting_time.elapsed().as_micros()
            );
        }
    } else if args.p != 0f64 {
        // this means we want to calculate p -> n
        for l in l_range {
            let starting_time = Instant::now();
            let required_tries = tries_required_exceed_prob(inv_factorial(l), args.p);
            println!("Array length: {}", l);
            println!("Tries to sort: \x1b[44m{}\x1b[0m,", required_tries);
            println!("    while {}! = \x1b[42m{}\x1b[0m", l, factorial(l));
            print!(
                "Took \x1b[41m{}μs\x1b[0m\n\n",
                starting_time.elapsed().as_micros()
            );
        }
    }
}
