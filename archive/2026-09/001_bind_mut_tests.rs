use learn_rust::exercises::current::exercise_fn;

#[test]
fn test_basic_case() {
    // 测试说明：验证基本的变量绑定和可变性
    let result = exercise_fn();
    assert_eq!(result, 15, "x=5, y=10, y = x + y 应该等于 15");
}

#[test]
fn test_x_and_y_values() {
    // 测试说明：验证 x 和 y 的值是否正确使用
    // 这个测试确保函数内部正确使用了 x 和 y 的值
    let result = exercise_fn();
    assert!(result > 10, "结果应该大于 y 的初始值 10");
    assert!(result < 20, "结果应该小于 20");
}

#[test]
fn test_return_type() {
    // 测试说明：验证返回类型是 i32
    let result: i32 = exercise_fn();
    assert_eq!(result, 15);
}
