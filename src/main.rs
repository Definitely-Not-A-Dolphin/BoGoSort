use sap::{Argument, Parser};
use std::time::Instant;

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

fn tries_required_exceed_prob(array_length: u32, prob: f64) -> u64 {
    ((1f64 - prob).ln() / (1f64 - inv_factorial(array_length)).ln()).ceil() as u64
}

fn prob_sorted_after_n_iterations(array_length: u32, iterations: u64) -> f64 {
    1f64 - (1f64 - inv_factorial(array_length)).powi(iterations as i32)
}

fn main() {
    let mut parser = Parser::from_env().unwrap();
    let mut args = Args {
        p: -1f64,
        n: 0,
        l_start: 1,
        l_end: 0,
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

    if args.l_end == 0 {
        args.l_end = 216;
    };

    if args.l_start > args.l_end {
        panic!("Invalid range: start value is greater than ending value");
    }

    let l_range = args.l_start..=args.l_end;

    if args.n != 0 {
        // this means we want to calculate n -> p
        for l in l_range {
            let starting_time = Instant::now();
            println!("Array length: {}", l);
            println!(
                "Prob sorted after {} iterations: {}",
                args.n,
                prob_sorted_after_n_iterations(l, args.n)
            );
            println!(
                "Took \x1b[41m{}μs\x1b[0m\n",
                starting_time.elapsed().as_micros()
            );
        }
    } else if args.p != 0f64 {
        // this means we want to calculate p -> n
        for l in l_range {
            let starting_time = Instant::now();
            let required_tries = tries_required_exceed_prob(l, args.p);
            println!("Array length: {}", l);
            println!("Tries to sort: \x1b[44m{}\x1b[0m,", required_tries);
            println!("    while {}! = \x1b[42m{}\x1b[0m", l, factorial(l));
            println!(
                "Took \x1b[41m{}μs\x1b[0m\n",
                starting_time.elapsed().as_micros()
            );
        }
    }
}
