mod bank_account;
use bank_account::BankAccount;

fn main() {
    println!("=== Bank Account ===\n");

    //Open a new account 
    let mut account = BankAccount::new(1000.0);
    println!("Opened account with initial balance: ${:.2}", account.balance());

    //Deposits
    account.deposit(300.0);
    println!("Deposited $300.00  → balance: ${:.2}", account.balance());

    account.deposit(-60.0);
    println!("Tried to deposit -$50.00 (ignored) → balance: ${:.2}", account.balance());

    //Withdrawals
    account.withdraw(100.0);
    println!("Withdrew $100.00   → balance: ${:.2}", account.balance());

    account.withdraw(3000.0);
    println!("Tried to withdraw $3000.00 (ignored, exceeds balance) → balance: ${:.2}",
             account.balance());

    account.withdraw(-30.0);
    println!("Tried to withdraw -$30.00 (ignored) → balance: ${:.2}", account.balance());

    //Interest
    account.apply_interest(0.05);
    println!("Applied 5% interest → balance: ${:.2}", account.balance());

    println!("\nFinal balance: ${:.2}", account.balance());
}