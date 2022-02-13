// The is_prime function should take an i32 and return a boolean.
// - It's fine to special-case numbers less than 2.
// - We declare negative numbers not to be prime.
// - Don't use a sieve; just loop over potential factors.
// - Get it working for the uncommented tests first, and then add the last two.
// - Note that there's no need to check factors larger than the square root of
//   our number. You should avoid calling a sqrt library function. (Rust has one
//   for floating-point types, but not integers.)
// - To rebuild and run #[test] functions, just type `cargo test`.

pub fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    let mut factors = 0;
    for p in 1..n / 2 + 1 {
        if n % p == 0 {
            factors = factors + 1;
        }
    }
    if factors < 2 {
        return true;
    }
    false
}

#[test]
fn check_primes() {
    assert!(!is_prime(0));
    assert!(!is_prime(1));
    assert!(is_prime(2));
    assert!(is_prime(3));
    assert!(!is_prime(4));
    assert!(is_prime(5));
    assert!(!is_prime(6));
    assert!(is_prime(7));

    assert!(!is_prime(51));
    assert!(is_prime(53));
    assert!(!is_prime(1013 * 1069));
    assert!(is_prime(2147483647));
}
