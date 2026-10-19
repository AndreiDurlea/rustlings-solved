struct Account {
    owner: &'static str,
    balance: u32,
}

fn balance(account: &Account) -> u32 {
    account.balance
}

// TODO: Fix the compiler error.
fn deposit(account: &Account, amount: u32) {
    account.balance += amount;
}

// TODO: Fix the function signature and implement the function.
// Withdraws `amount` if there is enough money and returns `true`.
// Otherwise, nothing changes and `false` is returned.
fn withdraw(account: Account, amount: u32) -> bool {
    false
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deposit_and_withdraw() {
        let mut account = Account {
            owner: "Ana",
            balance: 100,
        };

        // TODO: Fix the calls.
        deposit(&account, 50);
        assert_eq!(balance(&account), 150);

        assert!(withdraw(account, 120));
        assert_eq!(balance(&account), 30);

        assert!(!withdraw(account, 31));
        assert_eq!(balance(&account), 30);
        assert_eq!(account.owner, "Ana");
    }
}
