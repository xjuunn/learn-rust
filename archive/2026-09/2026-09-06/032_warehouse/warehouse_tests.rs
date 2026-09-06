use learn_rust::exercises::current::{exercise_fn, locate};

#[test]
fn test_found_first() {
    // 测试说明：验证能找到第一个货架
    let items = [(101, "A3"), (102, "B1")];
    assert_eq!(locate(&items, 101), Some("A3".to_string()));
}

#[test]
fn test_found_later() {
    // 测试说明：验证能找到非首位的货架
    let items = [(101, "A3"), (102, "B1")];
    assert_eq!(locate(&items, 102), Some("B1".to_string()));
}

#[test]
fn test_not_found() {
    // 测试说明：验证找不到返回 None
    let items = [(101, "A3"), (102, "B1")];
    assert_eq!(locate(&items, 999), None);
}

#[test]
fn test_empty_slice() {
    // 测试说明：边界情况——空表必然返回 None
    let items: [(u32, &str); 0] = [];
    assert_eq!(locate(&items, 1), None);
}

#[test]
fn test_longer_table() {
    // 测试说明：验证多条目表中命中中间项
    let items = [(1, "C1"), (2, "C2"), (3, "C3")];
    assert_eq!(locate(&items, 2), Some("C2".to_string()));
}

#[test]
fn test_exercise_fn() {
    // 测试说明：验证入口函数整体结果
    assert_eq!(exercise_fn(), (Some("A3".to_string()), None));
}