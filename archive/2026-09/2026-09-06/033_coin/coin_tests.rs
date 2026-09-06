use learn_rust::exercises::current::{Coin, exercise_fn, value_in_cents};

#[test]
fn test_penny() {
    // 测试说明：验证便士分值
    assert_eq!(value_in_cents(Coin::Penny), 1);
}

#[test]
fn test_nickel() {
    // 测试说明：验证五分镍币分值
    assert_eq!(value_in_cents(Coin::Nickel), 5);
}

#[test]
fn test_dime() {
    // 测试说明：验证一角分值
    assert_eq!(value_in_cents(Coin::Dime), 10);
}

#[test]
fn test_quarter() {
    // 测试说明：验证二十五分分值
    assert_eq!(value_in_cents(Coin::Quarter), 25);
}

#[test]
fn test_all_coins() {
    // 测试说明：验证所有硬币映射覆盖
    let values: Vec<u32> = [
        Coin::Penny,
        Coin::Nickel,
        Coin::Dime,
        Coin::Quarter,
    ]
    .into_iter()
    .map(value_in_cents)
    .collect();
    assert_eq!(values, vec![1, 5, 10, 25]);
}

#[test]
fn test_exercise_fn() {
    // 测试说明：验证入口函数整体结果
    assert_eq!(exercise_fn(), vec![1, 5, 10, 25]);
}