#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        BankAccount {
            balance: if initial_balance < 0.0 { 0.0 } else { initial_balance },
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method
        if amount >= 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
        if amount >= 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        // Implement this method
        self.balance
    }

    pub fn apply_interest(&mut self, rate: f64) {
        if rate >= 0.0 {
            self.balance += self.balance * rate;
        }
    }
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPS
    }


    #[test]
    fn test_new_account_positive_balance() {
        let account = BankAccount::new(100.0);
        assert!(approx_eq(account.balance(), 100.0),
            "Expected 100.0, got {}", account.balance());
    }

    #[test]
    fn test_new_account_zero_balance() {
        let account = BankAccount::new(0.0);
        assert!(approx_eq(account.balance(), 0.0));
    }


    // Deposit
    #[test]
    fn test_deposit_increases_balance() {
        let mut account = BankAccount::new(100.0);
        account.deposit(50.0);
        assert!(approx_eq(account.balance(), 150.0));
    }
 
    #[test]
    fn test_deposit_zero_leaves_balance_unchanged() {
        let mut account = BankAccount::new(100.0);
        account.deposit(0.0);
        assert!(approx_eq(account.balance(), 100.0));
    }


    // Withdraw
    #[test]
    fn test_withdraw_decreases_balance() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(40.0);
        assert!(approx_eq(account.balance(), 60.0));
    }
 
    #[test]
    fn test_withdraw_entire_balance() {
        let mut account = BankAccount::new(100.0);
        account.withdraw(100.0);
        assert!(approx_eq(account.balance(), 0.0));
    }


    // Apply Interest
    #[test]
    fn test_apply_interest_increases_balance() {
        let mut account = BankAccount::new(1000.0);
        account.apply_interest(0.05); //5% interest
        assert!(approx_eq(account.balance(), 1050.0));
    }
 
    #[test]
    fn test_apply_interest_zero_rate_unchanged() {
        let mut account = BankAccount::new(1000.0);
        account.apply_interest(0.0);
        assert!(approx_eq(account.balance(), 1000.0));
    }
 
    #[test]
    fn test_apply_interest_on_zero_balance() {
        let mut account = BankAccount::new(0.0);
        account.apply_interest(0.10);
        assert!(approx_eq(account.balance(), 0.0));
    }
}