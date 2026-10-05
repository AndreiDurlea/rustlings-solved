use std::sync::atomic::{AtomicUsize, Ordering};

// TODO: This counter never goes up! Find out why and fix this declaration.
const CALLS: AtomicUsize = AtomicUsize::new(0);

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
