use learn_rust::exercises::current::{Gift, apple_count, exercise_fn};

#[test]
fn test_apple_some() {
    // 测试说明：验证 Apple(n) 返回苹果个数
    assert_eq!(apple_count(&Gift::Apple(5)), 5);
}

#[test]
fn test_apple_other_count() {
    // 测试说明：验证不同数量
    assert_eq!(apple_count(&Gift::Apple(3)), 3);
}

#[test]
fn test_flower_zero() {
    // 测试说明：验证 Flower 返回 0
    assert_eq!(apple_count(&Gift::Flower), 0);
}

#[test]
fn test_gold_zero() {
    // 测试说明：验证 Gold 返回 0
    assert_eq!(apple_count(&Gift::Gold), 0);
}

#[test]
fn test_exercise_fn() {
    // 测试说明：验证入口函数整体结果
    assert_eq!(exercise_fn(), (5, 0));
}