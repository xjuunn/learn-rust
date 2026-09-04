use learn_rust::exercises::current::exercise_fn;

#[test]
fn test_basic() {
    // 测试说明：验证基本平方和
    let data = [1, 2, 3];
    assert_eq!(exercise_fn(&data), 14, "1+4+9 = 14");
}

#[test]
fn test_empty() {
    // 测试说明：验证空切片
    let data: [i32; 0] = [];
    assert_eq!(exercise_fn(&data), 0, "空切片平方和为 0");
}

#[test]
fn test_negative() {
    // 测试说明：验证负数平方（负负得正）
    let data = [-2, 5];
    assert_eq!(exercise_fn(&data), 29, "4+25 = 29");
}

#[test]
fn test_single() {
    // 测试说明：验证单元素
    let data = [7];
    assert_eq!(exercise_fn(&data), 49, "7*7 = 49");
}
