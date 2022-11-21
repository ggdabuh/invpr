use rayon::prelude::*;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use std::cell::{RefCell, RefMut, Cell};

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

fn sieve_of_sundaram(n: u32) -> Vec<u32> {
    let k = (n - 3) / 2 + 1;
    let s = ((k + 1) as f32 / 64_f32).ceil() as u32;

    let mut integers_list = Vec::<u64>::with_capacity(s as usize);
    integers_list.resize((s) as usize, u64::max_value());

    let mut res = vec![2];
    for i in 0..((n as f64).sqrt() as u32 - 3) / 2 + 1 {
        if is_set(&integers_list, i as usize) {
            res.push(2 * i + 3);
            // adding this condition turns it into a SoE!
            let p = 2 * i + 3;
            let s = (p * p - 3) / 2; // compute cull start

            for j in (s..k).step_by(p as usize) {
                unset(&mut integers_list, j as usize);
            }
        }

    }

    for i in ((n as f64).sqrt() as u32 - 3) / 2 + 1..k {
        if is_set(&integers_list, i as usize) {
            res.push(2 * i + 3);
        }
    }
    res
}

fn clear_last_bits(n: usize, acc: u64) -> u64 {
    let j = n % 64;
    let mask = (1 << j) - 1;
    acc & mask
}

fn sieve_of_sundaram2(n: u32) -> Vec<u32> {
    let k = ((n - 3) / 2 + 1) as usize;
    let list_size = (k + 63) / 64 as usize;

    let mut integers_list = Vec::new();
    integers_list.resize(list_size, u64::max_value());
    let tmp = clear_last_bits(k, integers_list[integers_list.len() - 1]);
    integers_list[list_size - 1] = tmp;


    for i in 0..(64.min(n as usize)) {

        if is_set(&integers_list, i as usize) {
            // adding this condition turns it into a SoE!
            let p = 2 * i + 3;
            let s = (p * p - 3) / 2; // compute cull start

            for j in (s..k).step_by(p as usize) {
                unset(&mut integers_list, j as usize);
            }
        }
    }

    for i in 1..integers_list.len() {

        let mut acc = integers_list[i];
        while acc != 0 {
            let tz = acc.trailing_zeros();
            let i = i * 64 + tz as usize;
            // adding this condition turns it into a SoE!
            let p = 2 * i + 3;
            let s = (p * p - 3) / 2; // compute cull start

            for j in (s..k).step_by(p as usize) {
                unset(&mut integers_list, j as usize);
            }
            acc &= !(1_u64 << tz);
        }
    }

    let mut res = vec![2];
    for i in 0..integers_list.len() {

        let mut acc = integers_list[i];
        while acc != 0 {
            let tz = acc.trailing_zeros();
            let i = i * 64 + tz as usize;
            res.push((2 * i + 3) as u32);
            acc &= !(1_u64 << tz);
        }
    }
    res
}

fn sieve_of_sundaram3(n: u32) -> Vec<u32> {
    let k = ((n - 3) / 2 + 1) as usize;
    let list_size = (k + 63) / 64 as usize;

    let mut integers_list = Vec::<u64>::new();
    integers_list.resize(list_size, u64::max_value());
    let tmp = clear_last_bits(k, integers_list[integers_list.len() - 1]);
    integers_list[list_size - 1] = tmp;

    for i in 0..((n as f64).sqrt() as u32 - 3) / 2 + 1 {
        if is_set(&integers_list, i as usize) {
            // adding this condition turns it into a SoE!
            let p = 2 * i + 3;
            let s = (p * p - 3) / 2; // compute cull start

            for j in (s as usize..k).step_by(p as usize) {
                unset(&mut integers_list, j as usize);
            }
        }
    }

    let mut res = vec![2];
    for i in 0..integers_list.len() {

        let mut acc = integers_list[i];
        while acc != 0 {
            let tz = acc.trailing_zeros();
            let i = i * 64 + tz as usize;
            res.push((2 * i + 3) as u32);
            acc &= !(1_u64 << tz);
        }
    }
    res
}

fn sieve_of_sundaram4(n: u32) -> Vec<u32> {
    let k = ((n - 3) / 2 + 1) as usize;
    let list_size = (k + 63) / 64 as usize;

    let mut integers_list = Vec::<u64>::new();
    integers_list.resize(list_size, u64::max_value());
    let tmp = clear_last_bits(k, integers_list[integers_list.len() - 1]);
    integers_list[list_size - 1] = tmp;

    for i in 0..((n as f64).sqrt() as u32 - 3) / 2 + 1 {
        if is_set(&integers_list, i as usize) {
            // adding this condition turns it into a SoE!
            let p = 2 * i + 3;
            let s = (p * p - 3) / 2; // compute cull start

            for j in (s as usize..k).step_by(p as usize) {
                unset(&mut integers_list, j as usize);
            }
        }
    }

    let mut res = vec![2];
    for i in 0..integers_list.len() {

        let mut acc = integers_list[i];
        while acc != 0 {
            let tz = acc.trailing_zeros();
            let i = i * 64 + tz as usize;
            res.push((2 * i + 3) as u32);
            acc &= !(1_u64 << tz);
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
    let start = Instant::now();
    let primes = primes(n);
    let duration = start.elapsed();
    println!("Time elapsed in primes() is: {:?}", duration);
    let start = Instant::now();
    let primes2 = sieve_of_sundaram2(n);
    let duration = start.elapsed();
    println!("Time elapsed in primes() is: {:?}", duration);
    let start = Instant::now();
    let primes3 = sieve_of_sundaram3(n);
    let duration = start.elapsed();
    println!("Time elapsed in primes() is: {:?}", duration);
    assert_eq!(primes, primes2);
    assert_eq!(primes, primes3);
    // println!("{:?}", primes);
    // let rng = primes[3..].to_vec();
    // println!("Starting");
    // let eval = |f: fn(u32, &Vec<u32>) -> u32| {
    //     let start = Instant::now();
    //     let res: Vec<(u32, u32)> = rng.par_iter().map(|i| (*i, f(*i, &primes))).collect();
    //     let duration = start.elapsed();
    //     println!("Time elapsed in calc() is: {:?}", duration);
    //     res
    // };
    // let r1 = eval(calc);
    // let mut file = File::create("out.txt").unwrap();
    // for (x, y) in r1 {
    //     file.write_fmt(format_args!("{}, {}\n", x, y)).unwrap();
    // }
}
