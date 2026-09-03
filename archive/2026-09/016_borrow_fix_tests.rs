use learn_rust::exercises::current::exercise_fn;

#[test]
fn test_normal_case() {
    // 测试说明：验证把一个元素翻倍并追加
    let mut v = vec![2, 3, 4];
    let result = exercise_fn(&mut v);
    assert_eq!(result, vec![4, 3, 4, 4], "首元素2翻倍为4，末尾追加4");
    // 原 v 也被修改（first_mut + push 直接作用于 v）
    assert_eq!(v, vec![4, 3, 4, 4], "原 v 也被就地修改");
}

#[test]
fn test_single_element() {
    // 测试说明：验证单元素向量
    let mut v = vec![5];
    let result = exercise_fn(&mut v);
    assert_eq!(result, vec![10, 10], "5翻倍为10并追加10");
}

#[test]
fn test_empty() {
    // 测试说明：验证空向量
    let mut v: Vec<i32> = vec![];
    let result = exercise_fn(&mut v);
    assert_eq!(result, vec![], "空向量返回空");
}
