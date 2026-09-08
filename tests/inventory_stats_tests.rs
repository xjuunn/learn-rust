use learn_rust::exercises::current::{exercise_fn, StockItem};
use std::collections::HashMap;

// 测试说明：验证同名商品的数量会累计
#[test]
fn test_same_name_accumulates() {
    let items = vec![
        StockItem { name: "苹果".to_string(), qty: 3 },
        StockItem { name: "苹果".to_string(), qty: 5 },
    ];
    let result = exercise_fn(&items);
    assert_eq!(result.get("苹果"), Some(&8));
}

// 测试说明：验证不同商品各计各的数量
#[test]
fn test_different_names() {
    let items = vec![
        StockItem { name: "苹果".to_string(), qty: 3 },
        StockItem { name: "香蕉".to_string(), qty: 2 },
    ];
    let result = exercise_fn(&items);
    assert_eq!(result.get("苹果"), Some(&3));
    assert_eq!(result.get("香蕉"), Some(&2));
}

// 测试说明：验证空记录表返回空 HashMap
#[test]
fn test_empty_items() {
    let result = exercise_fn(&[]);
    assert!(result.is_empty(), "空输入应返回空 HashMap");
}

// 测试说明：验证多条同名记录跨多条累计到同一个 key
#[test]
fn test_three_entries_accumulate() {
    let items = vec![
        StockItem { name: "笔".to_string(), qty: 2 },
        StockItem { name: "纸".to_string(), qty: 1 },
        StockItem { name: "笔".to_string(), qty: 4 },
        StockItem { name: "笔".to_string(), qty: 3 },
    ];
    let result = exercise_fn(&items);
    assert_eq!(result.get("笔"), Some(&9));
    assert_eq!(result.get("纸"), Some(&1));
    assert_eq!(result.len(), 2);
}

// 测试说明：验证单条记录也正确
#[test]
fn test_single_item() {
    let items = vec![StockItem { name: "牛奶".to_string(), qty: 7 }];
    let result = exercise_fn(&items);
    assert_eq!(result.get("牛奶"), Some(&7));
}