use learn_rust::exercises::current::exercise_fn;

#[test]
fn test_basic() {
    // 测试说明：验证普通 Vec 求和
    assert_eq!(exercise_fn(vec![1, 2, 3]), 6, "1+2+3 = 6");
}

#[test]
fn test_large_numbers() {
    // 测试说明：验证较大数字求和
    assert_eq!(exercise_fn(vec![10, 20, 30]), 60, "10+20+30 = 60");
}

#[test]
fn test_negative() {
    // 测试说明：验证包含负数的 Vec
    assert_eq!(exercise_fn(vec![-1, 5, -2]), 2, "-1+5-2 = 2");
}

#[test]
fn test_empty() {
    // 测试说明：验证空 Vec 返回 0
    assert_eq!(exercise_fn(vec![]), 0, "空 Vec 求和为 0");
}
