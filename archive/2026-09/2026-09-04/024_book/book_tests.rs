use learn_rust::exercises::current::{Book, exercise_fn};

#[test]
fn test_discounted_price() {
    // 测试说明：验证 8 折价格
    let b = Book::new("Rust编程", 100.0, 5);
    assert!((b.discounted_price(0.2) - 80.0).abs() < 1e-9, "100 打 8 折 = 80");
}

#[test]
fn test_buy_success() {
    // 测试说明：验证库存足够时购买成功并扣减库存
    let mut b = Book::new("Rust编程", 100.0, 5);
    assert!(b.buy(2), "库存5足够买2本");
    assert!((b.stock == 3), "剩余库存应为3");
}

#[test]
fn test_buy_fail() {
    // 测试说明：验证库存不足时购买失败且库存不变
    let mut b = Book::new("Rust编程", 100.0, 5);
    assert!(!b.buy(10), "库存5不足买10本");
    assert!((b.stock == 5), "失败时库存不应改变");
}

#[test]
fn test_low_stock() {
    // 测试说明：验证库存小于10为低库存
    let b = Book::new("Rust编程", 100.0, 5);
    assert!(b.is_low_stock(), "库存5小于10应为低库存");
    let b2 = Book::new("Rust编程", 100.0, 10);
    assert!(!b2.is_low_stock(), "库存10不低于10不应为低库存");
}

#[test]
fn test_exercise_fn() {
    // 测试说明：验证入口函数返回 (80.0, true)
    let (price, low) = exercise_fn();
    assert!((price - 80.0).abs() < 1e-9, "价格为80");
    assert!(low, "库存5为低库存");
}
