use learn_rust::exercises::current::exercise_fn;

#[test]
fn test_positive_case() {
    // 测试说明：验证正数数组求和
    let arr = [1, 2, 3, 4, 5];
    assert_eq!(exercise_fn(&arr), 15, "1+2+3+4+5 应该等于 15");
}

#[test]
fn test_mixed_signs() {
    // 测试说明：验证正负数混合求和
    let arr = [-1, 5, 3];
    assert_eq!(exercise_fn(&arr), 7, "-1+5+3 应该等于 7");
}

#[test]
fn test_empty_slice() {
    // 测试说明：验证空切片返回 0
    let arr: [i32; 0] = [];
    assert_eq!(exercise_fn(&arr), 0, "空切片应该返回 0");
}
