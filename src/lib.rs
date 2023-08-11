use rand::prelude::*;

use fast_exponentiation::{
    fast_exp_mod,
};

// Return a pseudorandom value in the range [min, max).
fn next_u64(prng: &mut ThreadRng, min: u64, max: u64) -> u64 {
    let range = (max - min) as u64;
    min + prng.next_u64() % range
}

pub fn is_probably_prime(prime_cand: u32, tests: i32) -> bool {
    let mut rng = thread_rng();
    let power: u32 = prime_cand - 1;
    println!("-----\nTesting {prime_cand}");
    for _ in 0..tests {
        let n: u64 = next_u64(&mut rng, 1, prime_cand as u64);
        println!("n = {n}");
        let r = fast_exp_mod(n as i64, power as i64, prime_cand as i64);
        if r != 1 {return false;}
    }
    println!("true return");
    true
}

pub fn find_prime(prng: &mut ThreadRng, min: u64, max: u64, tests: i32) -> u64 {
   loop {
       let n: u64 = next_u64(prng, min, max);
       if is_probably_prime(n as u32, tests) {
           return n;
       }
   }
}

#[cfg(test)]
mod tests {
    use super::*;
    use prime_factors::{
        find_factors,
    };

    const PRIMES: [i32; 168] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
        101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193, 197, 199,
        211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281, 283, 293,
        307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397,
        401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499,
        503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599,
        601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691,
        701, 709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797,
        809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887,
        907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997,
    ];

    #[test]
    fn prime_2() {
        let result = is_probably_prime(2, 1);
        assert!(result);
    }

    #[test]
    fn prime_3() {
        let result = is_probably_prime(3, 1);
        assert!(result);
    }

    #[test]
    fn prime_4() {
        let result = is_probably_prime(4, 3);
        assert!(! result);
    }

    #[test]
    fn prime_general() {
        for p in PRIMES {
            let result = is_probably_prime(p as u32, 25);
            assert!(result);
        }
    }

    #[test]
    fn composite_general() {
        let m = PRIMES[PRIMES.len() - 1];
        for composite in 2..(m + 1) {
            if ! PRIMES.contains(&composite) {
                let result = is_probably_prime(composite as u32, 25);
                println!("Testing {composite}");
                assert!(! result);
            }
        }
    }

    #[test]
    fn find_prime_7() {
        let mut rng = thread_rng();
        let min: u64 = 10_u64.pow(6);
        let max: u64 = 10_u64 * min - 1;
        let p = find_prime(&mut rng, min, max, 25);

        let factors = find_factors(p as usize);
        assert!(factors.len() == 1);
    }

    #[test]
    fn find_prime_8() {
        let mut rng = thread_rng();
        let min: u64 = 10_u64.pow(7);
        let max: u64 = 10_u64 * min - 1;
        let p = find_prime(&mut rng, min, max, 25);

        let factors = find_factors(p as usize);
        assert!(factors.len() == 1);
    }

    #[test]
    fn find_prime_9() {
        let mut rng = thread_rng();
        let min: u64 = 10_u64.pow(8);
        let max: u64 = 10_u64 * min - 1;
        let p = find_prime(&mut rng, min, max, 25);

        let factors = find_factors(p as usize);
        assert!(factors.len() == 1);
    }

    #[test]
    fn find_prime_10() {
        let mut rng = thread_rng();
        let min: u64 = 10_u64.pow(9);
        let max: u64 = 10_u64 * min - 1;
        let p = find_prime(&mut rng, min, max, 25);

        let factors = find_factors(p as usize);
        let msg = format!("{} has factors: {:?}", p, factors);
        assert!(factors.len() == 1, "{}", msg);
    }
}
