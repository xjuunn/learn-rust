use learn_rust::exercises::current::{exercise_fn, number_lines};

// 测试说明：验证基本的行号生成（行号从 1 开始）
#[test]
fn test_basic_numbering() {
    let result = number_lines("hello\nworld\nrust");
    assert_eq!(result, vec!["1: hello", "2: world", "3: rust"]);
}

// 测试说明：验证单行文本
#[test]
fn test_single_line() {
    let result = number_lines("only one");
    assert_eq!(result, vec!["1: only one"]);
}

// 测试说明：验证空文本返回空列表
#[test]
fn test_empty_text() {
    let result = number_lines("");
    assert!(result.is_empty(), "空文本应返回空列表");
}

// 测试说明：验证空行也被编号保留
#[test]
fn test_blank_line_kept() {
    let result = number_lines("a\n\nb");
    assert_eq!(result, vec!["1: a", "2: ", "3: b"]);
}

// 测试说明：验证结尾多余换行被忽略（lines 行为）
#[test]
fn test_trailing_newline() {
    let result = number_lines("a\nb\n");
    assert_eq!(result, vec!["1: a", "2: b"]);
}

// 测试说明：验证 exercise_fn 会加标题行
#[test]
fn test_exercise_fn_with_header() {
    let result = exercise_fn("a\nb");
    assert_eq!(result, vec!["共 2 行:", "1: a", "2: b"]);
}

// 测试说明：验证 exercise_fn 空文本返回仅标题行
#[test]
fn test_exercise_fn_empty() {
    let result = exercise_fn("");
    assert_eq!(result, vec!["共 0 行:"]);
}