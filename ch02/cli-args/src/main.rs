use std::str::FromStr;
use std::env;

fn main() {
    let mut numbers = Vec::new();

    for arg on env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER...");
        eprintln!("Compute the greatest common divisor of the given numbers.");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    // assert! macro is always enabled
    // use debug_assert! for checks that should only be active in debug builds
    assert!(n != 0 && m != 0);
    while m != 0 {
        // make sure n >= m
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        // now n <= m
        m = m % n;
    }
    // return value at the end without a semicolon
    n
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}