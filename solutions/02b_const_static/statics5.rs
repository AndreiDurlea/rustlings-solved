use std::sync::atomic::{AtomicUsize, Ordering};

// A `static` has a single memory location, a `const` is copied at each use.
static CALLS: AtomicUsize = AtomicUsize::new(0);

fn tracked_function() {
    CALLS.fetch_add(1, Ordering::Relaxed);
}

fn main() {
    tracked_function();
    tracked_function();

    let calls = CALLS.load(Ordering::Relaxed);
    println!("The function was called {calls} times");
    assert_eq!(calls, 2);
}
