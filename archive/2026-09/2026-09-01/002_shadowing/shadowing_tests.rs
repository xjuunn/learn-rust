use learn_rust::exercises::current::exercise_fn;

#[test]
fn test_basic_case() {
    // 测试说明：验证遮蔽和类型转换
    let result = exercise_fn();
    assert_eq!(result, 5.0, "\"hello\" 的长度是 5，转换为 f64 后应该是 5.0");
}

#[test]
fn test_string_length() {
    // 测试说明：验证字符串长度计算正确
    let result = exercise_fn();
    assert!(result > 0.0, "结果应该大于 0");
    assert!(result < 10.0, "结果应该小于 10");
}

#[test]
fn test_return_type() {
    // 测试说明：验证返回类型是 f64
    let result: f64 = exercise_fn();
    assert_eq!(result, 5.0);
}
