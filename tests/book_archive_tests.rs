use learn_rust::exercises::current::{archive, exercise_fn};
use std::collections::HashMap;

// 测试说明：验证同一分类的多本书被归档到同一书单，保持原顺序
#[test]
fn test_group_by_category() {
    let books = &[("三体", "科幻"), ("活着", "文学"), ("沙丘", "科幻")];
    let result = archive(books);
    assert_eq!(result.get("科幻"), Some(&vec!["三体".to_string(), "沙丘".to_string()]));
    assert_eq!(result.get("文学"), Some(&vec!["活着".to_string()]));
    assert_eq!(result.len(), 2);
}

// 测试说明：验证单分类多书记录原顺序
#[test]
fn test_order_preserved() {
    let books = &[("a", "x"), ("b", "x"), ("c", "x")];
    let result = archive(books);
    assert_eq!(result.get("x"), Some(&vec!["a".to_string(), "b".to_string(), "c".to_string()]));
}

// 测试说明：验证空输入返回空表
#[test]
fn test_empty_books() {
    let result = archive(&[]);
    assert!(result.is_empty(), "空输入应返回空 HashMap");
}

// 测试说明：验证 exercise_fn 返回每个分类的书本数量
#[test]
fn test_count_by_category() {
    let books = &[("科幻", "三体"), ("文学", "活着"), ("科幻", "沙丘")];
    let result = exercise_fn(books);
    assert_eq!(result.get("科幻"), Some(&2));
    assert_eq!(result.get("文学"), Some(&1));
}

// 测试说明：验证同一分类的书数量正确累计
#[test]
fn test_count_accumulates() {
    let books = &[("x", "1"), ("x", "2"), ("y", "3")];
    let result = exercise_fn(books);
    assert_eq!(result.get("x"), Some(&2));
    assert_eq!(result.get("y"), Some(&1));
}