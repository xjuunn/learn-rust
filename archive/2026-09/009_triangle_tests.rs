use learn_rust::exercises::current::exercise_fn;

#[test]
fn test_equilateral() {
    // 测试说明：验证等边三角形
    assert_eq!(exercise_fn(3, 3, 3), "等边");
}

#[test]
fn test_isosceles() {
    // 测试说明：验证等腰三角形
    assert_eq!(exercise_fn(3, 3, 5), "等腰");
}

#[test]
fn test_scalene() {
    // 测试说明：验证不等边三角形
    assert_eq!(exercise_fn(3, 4, 5), "不等边");
}

#[test]
fn test_invalid_triangle() {
    // 测试说明：验证不能构成三角形的情况
    assert_eq!(exercise_fn(1, 1, 2), "无效", "1+1=2 不构成三角形");
}

#[test]
fn test_invalid_zero_side() {
    // 测试说明：验证存在 0 或负边
    assert_eq!(exercise_fn(0, 3, 3), "无效", "有边为 0");
    assert_eq!(exercise_fn(-1, 2, 2), "无效", "有边为负");
}
