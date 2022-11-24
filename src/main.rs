use rayon::prelude::*;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

fn calc(n: u64, primes: &Vec<u64>) -> u64 {
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

fn mul_mod(a: u64, b: u64, m: u64) -> u64 {
    let (a, b, m) = (a as u128, b as u128, m as u128);
    ((a * b) % m) as u64
}

fn modular_pow(base: u64, mut exponent: u64, modulus: u64) -> u64 {
    let mut base = base;
    if modulus == 1 {
        return 0;
    }
    let mut result = 1_u64;
    base %= modulus;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = mul_mod(result, base, modulus);
        }
        exponent = exponent >> 1;
        base = mul_mod(base, base, modulus);
    }
    return result as u64;
}

fn is_set(list: &Vec<u64>, n: usize) -> bool {
    let i = n / 64;
    let j = n % 64;
    list[i] & 1 << j != 0
}

fn unset(list: &mut Vec<u64>, n: usize) {
    let i = n / 64;
    let j = n % 64;
    list[i] &=  !(1 << j);
}

fn clear_last_bits(n: usize, acc: u64) -> u64 {
    let j = n % 64;
    let mask = (1 << j) - 1;
    acc & mask
}

fn sieve_of_sundaram(n: u64) -> Vec<u64> {
    let k = ((n - 3) / 2 + 1) as usize;
    let list_size = (k + 63) / 64 as usize;

    let mut integers_list = Vec::<u64>::new();
    integers_list.resize(list_size, u64::max_value());
    let tmp = clear_last_bits(k, integers_list[integers_list.len() - 1]);
    integers_list[list_size - 1] = tmp;

    let nn = ((n as f64).sqrt() as usize - 3) / 2;
    let nn = (nn + 63) / 64;
    let mut res = vec![2];
    for i in 0..(nn + 1) * 64 {
        if is_set(&integers_list, i as usize) {
            res.push((2 * i + 3) as u64);
            // adding this condition turns it into a SoE!
            let p = 2 * i + 3;
            let s = (p * p - 3) / 2; // compute cull start

            for j in (s as usize..k).step_by(p as usize) {
                unset(&mut integers_list, j as usize);
            }
        }
    }

    let nn = ((n as f64).sqrt() as usize - 3) / 2;
    let nn = (nn + 63) / 64;
    for i in nn + 1..integers_list.len() {

        let mut acc = integers_list[i];
        while acc != 0 {
            let tz = acc.trailing_zeros();
            let i = i * 64 + tz as usize;
            res.push((2 * i + 3) as u64);
            acc &= !(1_u64 << tz);
        }
    }
    res
}

fn primes(to: u64) -> Vec<u64> {
    sieve_of_sundaram(to)
}

fn main() {
    let mut args = std::env::args();
    if args.len() != 2 {
        panic!("1 arg");
    }
    let n = args.nth(1).unwrap().parse().expect("Invalid number");
    let start = Instant::now();
    let primes = primes(n);
    let duration = start.elapsed();
    println!("Time elapsed in primes() is: {:?}", duration);
    let rng = primes[3..].to_vec();
    println!("Starting");
    let eval = |f: fn(u64, &Vec<u64>) -> u64| {
        let start = Instant::now();
        let res: Vec<(u64, u64)> = rng.par_iter().map(|i| (*i, f(*i, &primes))).collect();
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
