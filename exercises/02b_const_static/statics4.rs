use std::sync::atomic::{AtomicU32, Ordering};

// A safe global counter without `static mut` and without `unsafe`.
static VISITS: AtomicU32 = AtomicU32::new(0);

fn visit() {
    // TODO: Increase `VISITS` by 1.
}

fn main() {
    visit();
    visit();
    visit();

    let visits = VISITS.load(Ordering::Relaxed);
    println!("Visits: {visits}");
    assert_eq!(visits, 3);
}
