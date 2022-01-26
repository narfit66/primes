#![cfg_attr(test, feature(test))]
#![allow(unused_imports)]

#[rustversion::nightly]
extern crate test;

/// is the passed number a prime number.
fn is_prime(number: u64) -> bool {
    #[inline]
    fn is_even(number: u64) -> bool {
        number % 2 == 0
    }

    // if we have a 0 or an even number then not a prime.
    if number == 0 || is_even(number) {
        false
    } else {
        // loop from 3 to our stopping number.
        for i in 3..((number as f64).sqrt() + 1.0) as u64 {
            // if we have an odd number and are divisible by `i` then not a prime.
            if !is_even(i) && number % i == 0 {
                return false
            }
        }

        true
    }
}

fn main() {
    let limit = 10000;
    let mut primes = vec!();

    for number in 0..limit {
        if is_prime(number) {
            primes.push(number);
        }
    }

    println!("count of {} produced {} primes", limit, primes.len());
    println!("{:?}", primes);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[rustversion::nightly]
    use test::Bencher;

    #[test]
    fn first_25_primes() {
        let test_primes = vec!(1, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97);
        let mut primes = vec!();

        for number in 0..100 {
            if is_prime(number) {
                primes.push(number);
            }
        }

        assert_eq!(primes, test_primes);
    }

    #[test]
    fn number_of_primes_in_ten_million() {
        let mut primes = vec!();

        for number in 0..10_000_000 {
            if is_prime(number) {
                primes.push(number);
            }
        }

        assert_eq!(primes.len(), 664579);
    }

    #[rustversion::nightly]
    #[bench]
    fn primes_in_a_thousand(bench: &mut Bencher) {
        bench.iter(|| {
            let mut primes = vec!();

            for number in 0..1000 {
                if is_prime(number) {
                    primes.push(number);
                }
            }
        })
    }
}
