static APP_NAME: &str = "rustlings";
static MAX_USERS: u32 = 1000;

fn main() {
    println!("Welcome to {APP_NAME}! Up to {MAX_USERS} users allowed.");
    assert_eq!(APP_NAME, "rustlings");
    assert_eq!(MAX_USERS, 1000);
}
