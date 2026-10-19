struct Account {
    owner: &'static str,
    balance: u32,
}

fn balance(account: &Account) -> u32 {
    account.balance
}

fn deposit(account: &mut Account, amount: u32) {
    account.balance += amount;
}

// Withdraws `amount` if there is enough money and returns `true`.
// Otherwise, nothing changes and `false` is returned.
fn withdraw(account: &mut Account, amount: u32) -> bool {
    if account.balance < amount {
        return false;
    }

    account.balance -= amount;
    true
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

        deposit(&mut account, 50);
        assert_eq!(balance(&account), 150);

        assert!(withdraw(&mut account, 120));
        assert_eq!(balance(&account), 30);

        assert!(!withdraw(&mut account, 31));
        assert_eq!(balance(&account), 30);
        assert_eq!(account.owner, "Ana");
    }
}
