use learn_rust::exercises::current::{exercise_fn, Maybe};

// 测试说明：验证 Just 状态的 is_just 返回 true
#[test]
fn test_is_just_some() {
    assert!(Maybe::Just(5).is_just());
    assert!(Maybe::Just("hi").is_just());
}

// 测试说明：验证 Nothing 状态的 is_just 返回 false
#[test]
fn test_is_just_none() {
    assert!(!Maybe::<i32>::Nothing.is_just());
}

// 测试说明：验证 Nothing 状态的 is_nothing 返回 true
#[test]
fn test_is_nothing_none() {
    assert!(Maybe::<i32>::Nothing.is_nothing());
}

// 测试说明：验证 Just 状态的 is_nothing 返回 false
#[test]
fn test_is_nothing_some() {
    assert!(!Maybe::Just(5).is_nothing());
}

// 测试说明：验证 unwrap_or 有值时取出内部值
#[test]
fn test_unwrap_or_some() {
    assert_eq!(Maybe::Just(5).unwrap_or(0), 5);
    assert_eq!(Maybe::Just(2.5).unwrap_or(0.0), 2.5);
}

// 测试说明：验证 unwrap_or 无值时返回默认值
#[test]
fn test_unwrap_or_none() {
    assert_eq!(Maybe::<i32>::Nothing.unwrap_or(42), 42);
    assert_eq!(Maybe::<f64>::Nothing.unwrap_or(3.14), 3.14);
}

// 测试说明：验证 clone_or 有值返回副本（&str 类型）
#[test]
fn test_clone_or_some() {
    assert_eq!(Maybe::Just("hi").clone_or("empty"), "hi");
}

// 测试说明：验证 clone_or 无值返回默认值，且原 Maybe 仍可用
#[test]
fn test_clone_or_none() {
    let m: Maybe<i32> = Maybe::Nothing;
    assert_eq!(m.clone_or(7), 7);
    assert!(m.is_nothing(), "clone_or 之后原值仍可继续使用");
}

// 测试说明：验证入口跨多类型的综合结果
#[test]
fn test_exercise_fn() {
    let (a, b, c, d) = exercise_fn();
    assert!(a);
    assert!(!b);
    assert_eq!(c, 3.14);
    assert_eq!(d, "hi");
}