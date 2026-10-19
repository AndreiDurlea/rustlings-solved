// This quiz tests:
// - References (`&` and `&mut`)
// - Borrowing rules
// - Structs, enums and arrays
//
// A tiny bank. Implement the following functions:
//
// - `total`: The sum of the balances of all accounts.
// - `richest`: A reference to the account with the highest balance
//   (the first one in case of a tie).
// - `apply`: Applies a transaction to an account and returns `true`.
//   - `Deposit(amount)` adds the amount.
//   - `Withdraw(amount)` subtracts the amount, but only if the balance is big
//     enough. Otherwise, nothing changes and `false` is returned.
//   - A frozen account (`frozen == true`) rejects every transaction
//     (return `false`).
// - `transfer`: Moves `amount` from one account to another. It only succeeds
//   (returns `true`) if withdrawing from `from` and depositing into `to` both
//   succeed. If the deposit fails, the money must go back to `from`!

struct Account {
    owner: &'static str,
    balance: u32,
    frozen: bool,
}

enum Transaction {
    Deposit(u32),
    Withdraw(u32),
}

const N: usize = 4;

// TODO: Implement the functions below.

fn total(accounts: &[Account; N]) -> u32 {
    0
}

fn richest(accounts: &[Account; N]) -> &Account {
    &accounts[N - 1]
}

fn apply(account: &mut Account, transaction: Transaction) -> bool {
    false
}

fn transfer(from: &mut Account, to: &mut Account, amount: u32) -> bool {
    false
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    fn bank() -> [Account; N] {
        [
            Account {
                owner: "Ana",
                balance: 500,
                frozen: false,
            },
            Account {
                owner: "Bogdan",
                balance: 1200,
                frozen: false,
            },
            Account {
                owner: "Carmen",
                balance: 1200,
                frozen: false,
            },
            Account {
                owner: "Dan",
                balance: 50,
                frozen: true,
            },
        ]
    }

    #[test]
    fn test_total() {
        assert_eq!(total(&bank()), 2950);
    }

    #[test]
    fn test_richest() {
        let accounts = bank();
        assert_eq!(richest(&accounts).owner, "Bogdan");
    }

    #[test]
    fn test_apply() {
        let mut accounts = bank();

        assert!(apply(&mut accounts[0], Transaction::Deposit(100)));
        assert_eq!(accounts[0].balance, 600);

        assert!(apply(&mut accounts[0], Transaction::Withdraw(600)));
        assert_eq!(accounts[0].balance, 0);

        assert!(!apply(&mut accounts[0], Transaction::Withdraw(1)));
        assert_eq!(accounts[0].balance, 0);

        assert!(!apply(&mut accounts[3], Transaction::Deposit(10)));
        assert_eq!(accounts[3].balance, 50);
    }

    #[test]
    fn test_transfer() {
        let [mut ana, mut bogdan, _, mut dan] = bank();

        assert!(transfer(&mut bogdan, &mut ana, 200));
        assert_eq!((ana.balance, bogdan.balance), (700, 1000));

        assert!(!transfer(&mut ana, &mut bogdan, 701));
        assert_eq!((ana.balance, bogdan.balance), (700, 1000));

        // Dan's account is frozen, the money must go back to Ana.
        assert!(!transfer(&mut ana, &mut dan, 100));
        assert_eq!((ana.balance, dan.balance), (700, 50));
    }
}
