use learn_rust::exercises::current::exercise_fn;

// 测试说明：验证基本的拼接功能
#[test]
fn test_greet_basic() {
    let result = exercise_fn("Alice", "hello", "hi");
    assert_eq!(result[0], "你好, Alice");
}

// 测试说明：验证中文昵称也能正确拼接
#[test]
fn test_greet_chinese() {
    let result = exercise_fn("小明", "hi", "hey");
    assert_eq!(result[0], "你好, 小明");
}

// 测试说明：验证字符计数会忽略空格
#[test]
fn test_count_chars_ignores_spaces() {
    let result = exercise_fn("alien", "hello world", "x");
    assert_eq!(result[1], "10");
}

// 测试说明：验证中文字符按字符计数且空格被忽略
#[test]
fn test_count_chars_chinese() {
    let result = exercise_fn("a", "你好 世界", "b");
    assert_eq!(result[1], "4");
}

// 测试说明：验证空文本计数为 0
#[test]
fn test_count_chars_empty() {
    let result = exercise_fn("a", "", "b");
    assert_eq!(result[1], "0");
}

// 测试说明：验证单词转大写并追加感叹号
#[test]
fn test_shout_uppercase() {
    let result = exercise_fn("a", "b", "hi");
    assert_eq!(result[2], "HI!");
}

// 测试说明：验证边界情况——单个字符也正确大写
#[test]
fn test_shout_single_char() {
    let result = exercise_fn("a", "b", "z");
    assert_eq!(result[2], "Z!");
}
