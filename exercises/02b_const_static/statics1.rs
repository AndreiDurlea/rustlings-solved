// TODO: Declare two statics so that the program compiles:
// - `APP_NAME`, a string slice with the value "rustlings"
// - `MAX_USERS`, an unsigned 32-bit integer with the value 1000

fn main() {
    println!("Welcome to {APP_NAME}! Up to {MAX_USERS} users allowed.");
    assert_eq!(APP_NAME, "rustlings");
    assert_eq!(MAX_USERS, 1000);
}
