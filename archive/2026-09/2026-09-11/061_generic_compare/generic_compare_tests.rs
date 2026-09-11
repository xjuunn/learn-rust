use learn_rust::exercises::current::{clamp, exercise_fn, max_of};

// 测试说明：验证 max_of 对 i32 类型取较大值
#[test]
fn test_max_of_i32() {
    assert_eq!(max_of(3, 5), 5);
    assert_eq!(max_of(9, 4), 9);
}

// 测试说明：验证 max_of 对 f64 类型取较大值
#[test]
fn test_max_of_f64() {
    assert_eq!(max_of(2.5, 1.5), 2.5);
    assert_eq!(max_of(0.2, 0.8), 0.8);
}

// 测试说明：验证 max_of 对 &str 类型按字典序取较大值
#[test]
fn test_max_of_str() {
    assert_eq!(max_of("app", "banana"), "banana");
    assert_eq!(max_of("zebra", "apple"), "zebra");
}

// 测试说明：验证 max_of 对相等值返回 a（边界情况）
#[test]
fn test_max_of_equal() {
    assert_eq!(max_of(7, 7), 7);
}

// 测试说明：验证 clamp 值在范围内返回原值
#[test]
fn test_clamp_inside() {
    assert_eq!(clamp(5, 0, 10), 5);
}

// 测试说明：验证 clamp 低于下限返回下限（边界情况）
#[test]
fn test_clamp_lower() {
    assert_eq!(clamp(-3, 0, 10), 0);
    assert_eq!(clamp(10, 20, 30), 20);
}

// 测试说明：验证 clamp 高于上限返回上限（边界情况）
#[test]
fn test_clamp_upper() {
    assert_eq!(clamp(15, 0, 10), 10);
    assert_eq!(clamp(9, 1, 3), 3);
}

// 测试说明：验证 clamp 对 f64 类型同样生效（泛型适用不同浮点类型）
#[test]
fn test_clamp_f64() {
    assert_eq!(clamp(7.5, 0.0, 5.0), 5.0);
    assert_eq!(clamp(1.5, 0.0, 5.0), 1.5);
}

// 测试说明：验证 exercise_fn 跨三种类型的综合调用结果
#[test]
fn test_exercise_fn() {
    let (a, b, c) = exercise_fn();
    assert_eq!(a, 9);
    assert_eq!(b, 5.0);
    assert_eq!(c, "rust");
}