use learn_rust::exercises::current::exercise_fn;

#[test]
fn test_basic() {
    // 测试说明：验证元素翻倍
    assert_eq!(exercise_fn(vec![1, 2, 3]), vec![2, 4, 6]);
}

#[test]
fn test_single_element() {
    // 测试说明：验证单元素
    assert_eq!(exercise_fn(vec![5]), vec![10]);
}

#[test]
fn test_negative() {
    // 测试说明：验证负数翻倍
    assert_eq!(exercise_fn(vec![-1, 0, 3]), vec![-2, 0, 6]);
}

#[test]
fn test_empty() {
    // 测试说明：验证空 Vec
    assert_eq!(exercise_fn(vec![]), vec![]);
}
