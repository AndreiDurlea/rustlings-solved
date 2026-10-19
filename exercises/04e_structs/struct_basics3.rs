struct Counter {
    count: u32,
    step: u32,
}

fn main() {
    // TODO: Fix the compiler error.
    let counter = Counter { count: 0, step: 5 };

    counter.count += counter.step;
    counter.count += counter.step;

    println!("Count: {}", counter.count);
    assert_eq!(counter.count, 10);
}
