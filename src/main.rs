use std::eprintln;
use std::str::FromStr;

fn main() {
    let mut numbers: Vec<u64> = vec![];

    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("Could not convert."))
    }

    if numbers.len() == 0 {
        eprintln!("add more values");
        std::process::exit(1);
    }
    println!("Working here");
    let mut num = numbers[0];
    for m in &numbers[1..] {
        num = gc2d(num, *m)
    }

    println!("GCD of {:?} is {}", numbers, num);
}

// fn gcd(mut n: u64, mut m: u64) -> u64 {
//     assert!(m != 0 && n != 0);
//     while m != 0 { // Error: What you did was compare m != n [Logic]
//         if m < n { // Error: What you did was compared m > n [Logic]
//             let t = m;
//             m = n;
//             n = t;
//         }
//         m = m % n;
//     }
//     n
// }

fn gc2d(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
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

#[test]
fn test_gcd() {
    assert_eq!(gc2d(14, 15), 1);

    assert_eq!(gc2d(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}
