use learn_rust::exercises::current::{BankAccount, exercise_fn};

#[test]
fn test_deposit_withdraw() {
    // 测试说明：验证存 50 取 30 后余额为 120
    assert!((exercise_fn() - 120.0).abs() < 1e-9, "100+50-30 = 120");
}

#[test]
fn test_deposit() {
    // 测试说明：验证 deposit 增加余额
    let mut acc = BankAccount::new(100.0);
    acc.deposit(50.0);
    assert!((acc.balance() - 150.0).abs() < 1e-9, "存款后应为 150");
}

#[test]
fn test_withdraw() {
    // 测试说明：验证 withdraw 减少余额
    let mut acc = BankAccount::new(100.0);
    acc.withdraw(30.0);
    assert!((acc.balance() - 70.0).abs() < 1e-9, "取款后应为 70");
}

#[test]
fn test_new_balance() {
    // 测试说明：验证 new 设置初始余额
    let acc = BankAccount::new(500.0);
    assert!((acc.balance() - 500.0).abs() < 1e-9, "初始余额应为 500");
}
