use std::collections::HashSet;
use std::time::Instant;

fn calc(n: u32) -> u32 {
    let k = (n as f64).log10().ceil() as u32;
    let mut acc = 10_u32.pow(k);
    let mut res = 0;
    let mut done = HashSet::<u32>::new();
    loop {
        acc = acc % n;
        if !done.insert(acc) {
            return res;
        }
        res += 1;
        acc *= 10;
    }
}

fn sieve(mut n: u32) -> Vec<u32> {
    let mut res = Vec::<u32>::new();
    while n % 2 == 0 {
        res.push(2);
        n /= 2;
    }
    let mut i = 3;
    loop {
        if i * i > n {
            break;
        }
        if n % i == 0 {
            res.push(i);
            n /= i;
        } else {
            i += 2;
        }
    }
    if n != 1 {
        res.push(n);
    }
    return res;
}

fn sieve2(mut n: u32, primes: &Vec<u32>) -> Vec<u32> {
    let mut res = Vec::<u32>::new();
    let mut i = 0;
    loop {
        let k = primes[i];
        if k * k > n {
            break;
        }
        if n % k == 0 {
            res.push(k);
            n /= k;
        } else {
            i += 1;
        }
    }
    if n != 1 {
        res.push(n);
    }
    return res;
}

fn generate(factors: &Vec<u32>, generator: &Vec<bool>) -> u32 {
    return generator.iter().zip(factors).map(|(g, f)| -> u32 {
        return if *g {*f} else {1}
    }).product();
}

fn calc2(n: u32, primes: &Vec<u32>) -> u32 {
    let factors = sieve2(n - 1, &primes);
    let mut i = 0;
    let mut generator = Vec::<bool>::with_capacity(factors.len());
    for _ in 0..factors.len() {
        generator.push(false);
    }
    loop {
        if !generator[i] {
            generator[i] = true;
            let generated = generate(&factors, &generator);
            let x = modular_pow(10, generated, n);
            if x == 1 {
                return generated;
            }
            i = 0;
        } else {
            generator[i] = false;
            i += 1;
            if i == factors.len() {
                panic!("Argh! {}, {:?}", n, factors);
            }

        }
    }
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

// fn f() {
//     let mut n = 10_u32;
//     let exp = 5001_u32;
//     let module = 60_013_u32;
//     let one = 1_u32;
//     n = modular_pow(n, exp, module);
//     if n == one {
//         println!("Youhou!");
//     } else {
//         println!("Bouhou!");
//     }
// }

fn primes(to: u32) -> Vec<u32> {
    let mut res = vec![2];
    for i in (3..=to).step_by(2) {
        if res.iter().find(|e| {
            i % **e == 0
        }).is_none() {
            res.push(i);
        }
    }
    res = res[3..].to_vec();
    res
}

fn main() {
    // println!("{:?}", sieve(1422));
    let x = modular_pow(10, 65536, 65537);
    println!("{:?}", x);

    let mut args = std::env::args();
    if args.len() != 2 {
        panic!("1 arg");
    }
    let n = args.nth(1).unwrap().parse().expect("INvalid number");
    let primes = primes(n);
    let rng = primes[3..].to_vec();
    // println!("{:?}", primes);
    // let start = Instant::now();
    // let res1: Vec<(u32, u32)> = primes.iter().map(|i| {
    //     (*i, calc(*i))
    // }).collect();
    // let duration = start.elapsed();
    // println!("Time elapsed in calc() is: {:?}", duration);
    let start = Instant::now();
    let res2: Vec<(u32, u32)> = rng.iter().map(|i| {
        (*i, calc2(*i, &primes))
    }).collect();
    let duration = start.elapsed();
    println!("Time elapsed in calc2() is: {:?}", duration);
    // let x = res1.iter().zip(&res2).find(|(l, r)| {*l != *r});
    // if let Some(y) = x {
    //     panic!("bhjygyhkhkl {:?} {:?} {:?}", y, res1, res2);
    // }

}
