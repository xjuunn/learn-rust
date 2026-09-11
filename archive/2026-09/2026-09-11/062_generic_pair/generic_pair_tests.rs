use learn_rust::exercises::current::{exercise_fn, Pair};

// 测试说明：验证 new 关联函数构造 Pair
#[test]
fn test_new() {
    let p = Pair::new(1, 2);
    assert_eq!(p.first, 1);
    assert_eq!(p.second, 2);
}

// 测试说明：验证 new 对 &str 类型也适用（泛型结构体多类型）
#[test]
fn test_new_str() {
    let p = Pair::new("rust", "python");
    assert_eq!(p.first, "rust");
}

// 测试说明：验证 swap 交换两个值并返回新 Pair，原值不动
#[test]
fn test_swap() {
    let p = Pair::new(1, 2);
    let swapped = p.swap();
    assert_eq!(swapped.first, 2);
    assert_eq!(swapped.second, 1);
}

// 测试说明：验证 first_ref 返回引用且不移动值
#[test]
fn test_first_ref() {
    let p = Pair::new(10, 20);
    assert_eq!(*p.first_ref(), 10);
    assert_eq!(p.first, 10, "first_ref 不应移动 first");
}

// 测试说明：验证 larger 返回较大值（i32）
#[test]
fn test_larger_i32() {
    let p = Pair::new(3, 5);
    assert_eq!(*p.larger(), 5);
    let p2 = Pair::new(9, 4);
    assert_eq!(*p2.larger(), 9);
}

// 测试说明：验证 larger 对 &str 按字典序返回较大值（方法级约束生效）
#[test]
fn test_larger_str() {
    let p = Pair::new("rust", "python");
    assert_eq!(*p.larger(), "rust");
}

// 测试说明：验证 larger 对 f64 生效
#[test]
fn test_larger_f64() {
    let p = Pair::new(2.5, 1.5);
    assert_eq!(*p.larger(), 2.5);
}

// 测试说明：验证入口综合四种构造的结果
#[test]
fn test_exercise_fn() {
    let (a, b, c, d) = exercise_fn();
    assert_eq!(a, 10);
    assert_eq!(b, 2.5);
    assert_eq!(c, "rust");
    assert_eq!(d, 3);
}