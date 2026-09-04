mod exercises;

fn main() {
    println!("=== 银行账户 ===");
    let mut acc = exercises::current::BankAccount::new(100.0);
    acc.deposit(50.0);
    acc.withdraw(30.0);
    println!("存入 50、取出 30 后，余额: {}", acc.balance());
}
