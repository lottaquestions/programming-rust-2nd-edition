use std::str::FromStr;
use std::env;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(m != 0 && n != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

// To run main do: cargo run params, eg: cargo run 10 5 20 25 30
fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1){
        numbers.push(u64::from_str(&arg).expect("Error parsing argument"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }
    let mut d = numbers[0];
    for m in &numbers[1..]{
        d = gcd(d, *m);
    }
    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

// To run test_gcd do: cargo test
#[test]
fn test_gcd() {
    let test_data: [(u64, u64, u64); 5] = [
        (100, 10, 10),
        (20, 25, 5),
        (18, 12, 6),
        (14, 15, 1),
        (2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19, 3 * 11),
    ];
    for (idx, &(m, n, expected)) in test_data.iter().enumerate() {
        let res = gcd(n, m);
        println!(
            "test case {} : gcd of m = {} and n = {}  is {}",
            idx, m, n, res
        );
        assert_eq!(res, expected);
    }
}
