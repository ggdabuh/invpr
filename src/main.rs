use rayon::prelude::*;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

fn calc(n: u32, primes: &Vec<u32>) -> u32 {
    let mut res = n - 1;
    let mut acc = res;
    for p in primes {
        if p * p > acc {
            break;
        }
        while acc % p == 0 {
            acc /= p;
            let x = res / p;
            if modular_pow(10, x, n) == 1 {
                res = x;
            }
        }
    }
    if acc != 1 {
        let x = res / acc;
        if modular_pow(10, x, n) == 1 {
            res = x;
        }
    }
    return res;
}

fn modular_pow(base: u32, mut exponent: u32, modulus: u32) -> u32 {
    let mut base = base as u64;
    let modulus = modulus as u64;
    if modulus == 1 {
        return 0;
    }
    let mut result = 1_u64;
    base %= modulus;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus as u64;
        }
        exponent = exponent >> 1;
        base = (base * base) % modulus;
    }
    return result as u32;
}

fn sieve_of_sundaram(n: u32) -> Vec<u32> {
    let k = (n - 3) / 2 + 1;

    let mut integers_list = Vec::with_capacity((k + 1) as usize);
    integers_list.resize((k + 1) as usize, true);

    for i in 0..((n as f64).sqrt() as u32 - 3) / 2 + 1 {
        if integers_list[i as usize] {
            // adding this condition turns it into a SoE!
            let p = 2 * i + 3;
            let s = (p * p - 3) / 2; // compute cull start

            for j in (s..k).step_by(p as usize) {
                integers_list[j as usize] = false;
            }
        }
    }

    let mut res = vec![2];
    for i in 0..k {
        if integers_list[i as usize] {
            res.push(2 * i + 3);
        }
    }
    res
}

fn primes(to: u32) -> Vec<u32> {
    sieve_of_sundaram(to)
}

fn main() {
    let mut args = std::env::args();
    if args.len() != 2 {
        panic!("1 arg");
    }
    let n = args.nth(1).unwrap().parse().expect("Invalid number");
    let primes = primes(n);
    let rng = primes[3..].to_vec();
    println!("Starting");
    let eval = |f: fn(u32, &Vec<u32>) -> u32| {
        let start = Instant::now();
        let res: Vec<(u32, u32)> = rng.par_iter().map(|i| (*i, f(*i, &primes))).collect();
        let duration = start.elapsed();
        println!("Time elapsed in calc() is: {:?}", duration);
        res
    };
    let r1 = eval(calc);
    let mut file = File::create("out.txt").unwrap();
    for (x, y) in r1 {
        file.write_fmt(format_args!("{}, {}\n", x, y)).unwrap();
    }
}
