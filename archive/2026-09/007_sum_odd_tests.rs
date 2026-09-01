use learn_rust::exercises::current::exercise_fn;

#[test]
fn test_even_n() {
    // 测试说明：验证偶数的 n 时奇数之和
    assert_eq!(exercise_fn(10), 25, "1+3+5+7+9 = 25");
}

#[test]
fn test_odd_n() {
    // 测试说明：验证奇数 n 时包含 n 本身
    assert_eq!(exercise_fn(7), 16, "1+3+5+7 = 16");
}

#[test]
fn test_zero() {
    // 测试说明：验证 n=0 返回 0
    assert_eq!(exercise_fn(0), 0, "n=0 返回 0");
}

#[test]
fn test_negative() {
    // 测试说明：验证负数返回 0
    assert_eq!(exercise_fn(-5), 0, "负数返回 0");
}
