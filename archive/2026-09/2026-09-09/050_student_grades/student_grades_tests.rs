use learn_rust::exercises::current::{exercise_fn, calculate_average};

// 测试说明：验证基本的平均分计算
#[test]
fn test_basic_average() {
    let scores = vec![("Alice", 85.0), ("Bob", 92.0)];
    let result = calculate_average(scores);
    assert_eq!(result.get("Alice"), Some(&85.0));
    assert_eq!(result.get("Bob"), Some(&92.0));
}

// 测试说明：验证同一学生多次成绩的平均分计算
#[test]
fn test_multiple_scores() {
    let scores = vec![("Alice", 80.0), ("Alice", 90.0), ("Alice", 100.0)];
    let result = calculate_average(scores);
    assert_eq!(result.get("Alice"), Some(&90.0));
}

// 测试说明：验证单个学生的情况
#[test]
fn test_single_student() {
    let scores = vec![("Bob", 75.0)];
    let result = calculate_average(scores);
    assert_eq!(result.get("Bob"), Some(&75.0));
}

// 测试说明：验证 exercise_fn 返回最高分和最低分学生
#[test]
fn test_exercise_fn_basic() {
    let scores = vec![("Alice", 85.0), ("Bob", 92.0), ("Charlie", 78.0)];
    let (top, bottom) = exercise_fn(scores);
    assert_eq!(top, "Bob");
    assert_eq!(bottom, "Charlie");
}

// 测试说明：验证 exercise_fn 处理有并列最高/最低分的情况
#[test]
fn test_exercise_fn_tie() {
    let scores = vec![("Alice", 90.0), ("Bob", 90.0), ("Charlie", 80.0)];
    let (top, bottom) = exercise_fn(scores);
    assert!(top == "Alice" || top == "Bob", "最高分应该是 Alice 或 Bob");
    assert_eq!(bottom, "Charlie");
}

// 测试说明：验证空数据的情况
#[test]
fn test_exercise_fn_empty() {
    let scores: Vec<(&str, f64)> = vec![];
    let result = exercise_fn(scores);
    assert_eq!(result.0, "");
    assert_eq!(result.1, "");
}
