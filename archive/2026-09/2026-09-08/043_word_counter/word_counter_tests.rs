use learn_rust::exercises::current::exercise_fn;
use std::collections::HashMap;

// 测试说明：验证单个文本的词频统计
#[test]
fn test_single_text() {
    let result = exercise_fn(&["the cat and the dog"]);
    let mut expect = HashMap::new();
    expect.insert("the".to_string(), 2);
    expect.insert("cat".to_string(), 1);
    expect.insert("and".to_string(), 1);
    expect.insert("dog".to_string(), 1);
    assert_eq!(result, expect);
}

// 测试说明：验证多个文本的词频会合并累计
#[test]
fn test_merge_across_texts() {
    let result = exercise_fn(&["a b", "b c", "c a"]);
    let mut expect = HashMap::new();
    expect.insert("a".to_string(), 2);
    expect.insert("b".to_string(), 2);
    expect.insert("c".to_string(), 2);
    assert_eq!(result, expect);
}

// 测试说明：验证无文本时返回空表
#[test]
fn test_empty() {
    let result = exercise_fn(&[]);
    assert!(result.is_empty(), "空输入应返回空 HashMap");
}

// 测试说明：验证空字符串文本不产生任何统计
#[test]
fn test_empty_string() {
    let result = exercise_fn(&["", "  "]);
    assert!(result.is_empty(), "纯空格/空串应无单词");
}

// 测试说明：验证连续多个空格仍被正确分隔
#[test]
fn test_extra_spaces() {
    let result = exercise_fn(&["a  b   a"]);
    assert_eq!(result.get("a"), Some(&2));
    assert_eq!(result.get("b"), Some(&1));
}

// 测试说明：验证完全不重复的单词各计 1 次
#[test]
fn test_all_unique() {
    let result = exercise_fn(&["x y z"]);
    assert_eq!(result.len(), 3);
    assert_eq!(result.get("x"), Some(&1));
    assert_eq!(result.get("z"), Some(&1));
}
