use learn_rust::exercises::current::exercise_fn;

// 测试说明：验证能返回出现次数最多的单词（高频在前）
#[test]
fn test_high_frequency_first() {
    let result = exercise_fn("a b c a b a", 2);
    assert_eq!(result, vec!["a", "b"]);
}

// 测试说明：验证 k 限制返回数量
#[test]
fn test_limit_by_k() {
    let result = exercise_fn("x y z", 1);
    assert_eq!(result.len(), 1);
}

// 测试说明：验证 k 大于单词总数时返回全部单词
#[test]
fn test_k_bigger_than_all() {
    let mut result = exercise_fn("apple banana cherry", 10);
    result.sort();
    assert_eq!(result.len(), 3);
    assert_eq!(result, vec!["apple", "banana", "cherry"]);
}

// 测试说明：验证 k 为 0 时返回空列表
#[test]
fn test_zero_k() {
    let result = exercise_fn("a b c", 0);
    assert!(result.is_empty(), "k=0 应返回空列表");
}

// 测试说明：验证空文本返回空列表
#[test]
fn test_empty_text() {
    let result = exercise_fn("", 5);
    assert!(result.is_empty(), "空文本应返回空列表");
}

// 测试说明：验证多词高频排序（每个出现次数不同，必须严格降序）
#[test]
fn test_num_ordering() {
    let result = exercise_fn("d b c b c c d b b", 2);
    assert_eq!(result, vec!["b", "c"]);
}