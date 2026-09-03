use learn_rust::exercises::current::exercise_fn;

#[test]
fn test_normal_case() {
    // 测试说明：验证元素翻倍并返回原长度
    let mut v = vec![1, 2, 3];
    let len = exercise_fn(&mut v);
    assert_eq!(len, 3, "原长度应为 3");
    assert_eq!(v, vec![2, 4, 6], "元素应翻倍为 [2,4,6]");
}

#[test]
fn test_single_element() {
    // 测试说明：验证单元素
    let mut v = vec![7];
    let len = exercise_fn(&mut v);
    assert_eq!(len, 1);
    assert_eq!(v, vec![14]);
}

#[test]
fn test_empty() {
    // 测试说明：验证空向量
    let mut v: Vec<i32> = vec![];
    let len = exercise_fn(&mut v);
    assert_eq!(len, 0, "空向量原长度为 0");
}

#[test]
fn test_negative_numbers() {
    // 测试说明：验证负数翻倍
    let mut v = vec![-1, 0, 5];
    let len = exercise_fn(&mut v);
    assert_eq!(len, 3);
    assert_eq!(v, vec![-2, 0, 10]);
}
