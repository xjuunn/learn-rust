use learn_rust::exercises::current::{
    exercise_fn, find_account, transfer, Account, TransferError,
};

fn acct(name: &str, balance: f64) -> Account {
    Account { name: name.to_string(), balance }
}

// 测试说明：验证 find_account 找到账户返回下标
#[test]
fn test_find_account_found() {
    let wallets = [acct("A", 100.0), acct("B", 50.0)];
    assert_eq!(find_account(&wallets, "B"), Some(1));
}

// 测试说明：验证 find_account 找不到账户返回 None（边界情况）
#[test]
fn test_find_account_missing() {
    let wallets: [Account; 0] = [];
    assert_eq!(find_account(&wallets, "C"), None);
}

// 测试说明：验证正常转账并更新余额
#[test]
fn test_transfer_ok() {
    let mut wallets = [acct("A", 100.0), acct("B", 50.0)];
    assert_eq!(transfer(&mut wallets, "A", "B", 30.0), Ok(()));
    assert!((wallets[0].balance - 70.0).abs() < 1e-9);
    assert!((wallets[1].balance - 80.0).abs() < 1e-9);
}

// 测试说明：验证收款方不存在返回 AccountNotFound
#[test]
fn test_transfer_account_not_found() {
    let mut wallets = [acct("A", 100.0), acct("B", 50.0)];
    assert_eq!(
        transfer(&mut wallets, "A", "C", 30.0),
        Err(TransferError::AccountNotFound("C".to_string()))
    );
    assert!((wallets[0].balance - 100.0).abs() < 1e-9, "失败时余额不应改变");
    assert!((wallets[1].balance - 50.0).abs() < 1e-9);
}

// 测试说明：验证金额为 0 或负数时返回 InvalidAmount（边界情况）
#[test]
fn test_transfer_invalid_amount() {
    let mut wallets = [acct("A", 100.0), acct("B", 50.0)];
    assert_eq!(transfer(&mut wallets, "A", "B", 0.0), Err(TransferError::InvalidAmount(0.0)));
    assert_eq!(transfer(&mut wallets, "A", "B", -5.0), Err(TransferError::InvalidAmount(-5.0)));
    assert!((wallets[0].balance - 100.0).abs() < 1e-9, "失败时余额不应改变");
}

// 测试说明：验证余额不足返回 InsufficientBalance
#[test]
fn test_transfer_insufficient() {
    let mut wallets = [acct("A", 10.0), acct("B", 50.0)];
    assert_eq!(
        transfer(&mut wallets, "A", "B", 20.0),
        Err(TransferError::InsufficientBalance)
    );
    assert!((wallets[0].balance - 10.0).abs() < 1e-9, "失败时余额不应改变");
    assert!((wallets[1].balance - 50.0).abs() < 1e-9);
}

// 测试说明：验证 TransferError 的 Display 输出
#[test]
fn test_error_display() {
    assert_eq!(
        TransferError::AccountNotFound("C".to_string()).to_string(),
        "收款方账户 'C' 不存在"
    );
    assert_eq!(
        TransferError::InvalidAmount(0.0).to_string(),
        "转账金额必须大于 0，当前为 0"
    );
    assert_eq!(TransferError::InsufficientBalance.to_string(), "余额不足");
}

// 测试说明：验证 exercise_fn 混合成功与失败的多笔转账
#[test]
fn test_exercise_fn_mixed() {
    let mut wallets = [acct("A", 100.0), acct("B", 50.0)];
    let transfers: [(&str, &str, f64); 3] = [
        ("A", "B", 30.0),
        ("A", "C", 10.0),
        ("A", "B", 0.0),
    ];
    let results = exercise_fn(&mut wallets, &transfers);
    assert_eq!(results[0], "A -> B 转账 30.00 元成功");
    assert!(results[1].contains("失败") && results[1].contains("C"));
    assert!(results[2].contains("失败") && results[2].contains("必须大于 0"));
    assert!((wallets[0].balance - 70.0).abs() < 1e-9, "只有成功那笔生效");
    assert!((wallets[1].balance - 80.0).abs() < 1e-9);
}

// 测试说明：验证 exercise_fn 处理空转账列表（边界情况）
#[test]
fn test_exercise_fn_empty() {
    let mut wallets = [acct("A", 100.0)];
    let transfers: [(&str, &str, f64); 0] = [];
    assert!(exercise_fn(&mut wallets, &transfers).is_empty());
}