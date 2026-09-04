use learn_rust::exercises::current::exercise_fn;

#[test]
fn test_basic_case() {
    // 测试说明：验证基本的混合类型加法
    let result = exercise_fn(3, 2.5, 2);
    assert_eq!(result, 7.5, "3 + 2.5 + 2 应该等于 7.5");
}

#[test]
fn test_zero_case() {
    // 测试说明：验证全零输入
    let result = exercise_fn(0, 0.0, 0);
    assert_eq!(result, 0.0, "全零输入应该返回 0.0");
}

#[test]
fn test_negative_case() {
    // 测试说明：验证负数参与运算
    let result = exercise_fn(-5, 1.5, 10);
    assert_eq!(result, 6.5, "-5 + 1.5 + 10 应该等于 6.5");
}
