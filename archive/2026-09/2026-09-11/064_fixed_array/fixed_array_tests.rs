use learn_rust::exercises::current::{exercise_fn, identity, FixedArray};

// 测试说明：验证 filled 用统一值填满容器
#[test]
fn test_filled() {
    let a: FixedArray<i32, 3> = FixedArray::filled(7);
    assert_eq!(a.items, [7, 7, 7]);
}

// 测试说明：验证 filled 对不同长度 N 生效（N=4）
#[test]
fn test_filled_n4() {
    let a: FixedArray<i32, 4> = FixedArray::filled(0);
    assert_eq!(a.items, [0, 0, 0, 0]);
    assert_eq!(a.len(), 4);
}

// 测试说明：验证不同 N 是两个不同类型且长度正确（边界：N=1）
#[test]
fn test_len() {
    let a: FixedArray<f64, 5> = FixedArray::filled(1.5);
    assert_eq!(a.len(), 5);
    let one: FixedArray<i32, 1> = FixedArray::filled(9);
    assert_eq!(one.len(), 1);
}

// 测试说明：验证 get 在合法下标返回元素引用
#[test]
fn test_get_valid() {
    let a: FixedArray<i32, 3> = FixedArray::filled(7);
    assert_eq!(*a.get(0).unwrap(), 7);
    assert_eq!(*a.get(2).unwrap(), 7);
}

// 测试说明：验证 get 越界下标返回 None（边界情况，不 panic）
#[test]
fn test_get_out_of_range() {
    let a: FixedArray<i32, 3> = FixedArray::filled(7);
    assert!(a.get(3).is_none());
    assert!(a.get(100).is_none());
}

// 测试说明：验证 get 对 N=0 空容器任何下标都返回 None（极端边界）
#[test]
fn test_get_empty() {
    let a: FixedArray<i32, 0> = FixedArray::filled(0);
    assert_eq!(a.len(), 0);
    assert!(a.get(0).is_none());
}

// 测试说明：验证 identity 生成下标数组
#[test]
fn test_identity() {
    assert_eq!(identity::<4>(), [0, 1, 2, 3]);
}

// 测试说明：验证 identity 对 N=1（边界）与不同长度生成不同大小数组
#[test]
fn test_identity_size() {
    assert_eq!(identity::<1>(), [0]);
    assert_eq!(identity::<2>(), [0, 1]);
}

// 测试说明：验证入口综合不同长度/类型的定长容器结果
#[test]
fn test_exercise_fn() {
    let (a, b, c, d) = exercise_fn();
    assert_eq!(a, 7);
    assert_eq!(b, 3);
    assert_eq!(c, 5);
    assert_eq!(d, 2);
}