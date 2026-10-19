struct Counter {
    count: u32,
    step: u32,
}

fn main() {
    // The whole struct must be mutable to change a field.
    let mut counter = Counter { count: 0, step: 5 };

    counter.count += counter.step;
    counter.count += counter.step;

    println!("Count: {}", counter.count);
    assert_eq!(counter.count, 10);
}
