fn main() {
    // TODO: Fix the compiler error. Don't remove any numbers!
    let primes: [u32; 5] = [2, 3, 5, 7, 11, 13];

    println!("The 6th prime is {}", primes[5]);
    assert_eq!(primes[0], 2);
    assert_eq!(primes[5], 13);
}
