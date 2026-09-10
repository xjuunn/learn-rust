use learn_rust::exercises::current::{exercise_fn, parse_record};

// 测试说明：验证正常解析一行成绩记录
#[test]
fn test_parse_valid() {
    assert_eq!(parse_record("小红:90"), Ok(("小红".to_string(), 90)));
}

// 测试说明：验证姓名和分数周围的空白会被去除（边界情况）
#[test]
fn test_parse_with_trim() {
    assert_eq!(parse_record(" 小刚 : 88 "), Ok(("小刚".to_string(), 88)));
}

// 测试说明：验证缺少冒号时返回格式错误
#[test]
fn test_parse_missing_colon() {
    let err = parse_record("小红").unwrap_err();
    assert!(err.contains("解析失败"), "应报告格式错误，实际: {}", err);
}

// 测试说明：验证分数为非法数字时通过 ? 传播 parse_score 的错误
#[test]
fn test_parse_bad_score() {
    let err = parse_record("小红:abc").unwrap_err();
    assert!(err.contains("分数必须是数字"), "应传播分数解析错误，实际: {}", err);
}

// 测试说明：验证出现多于一个冒号时返回格式错误
#[test]
fn test_parse_extra_colon() {
    let err = parse_record("小红:90:81").unwrap_err();
    assert!(err.contains("解析失败"), "多余冒号应报格式错误，实际: {}", err);
}

// 测试说明：验证 exercise_fn 正常行与错误行混合处理
#[test]
fn test_exercise_fn_mixed() {
    let results = exercise_fn(&["小红:90", "小明:abc"]);
    assert_eq!(results[0], "小红：90 分");
    assert!(results[1].contains("跳过"));
    assert!(results[1].contains("分数必须是数字"));
}

// 测试说明：验证 exercise_fn 全部正常
#[test]
fn test_exercise_fn_all_valid() {
    let results = exercise_fn(&["小红:90", "小刚:88", "小明:100"]);
    assert_eq!(results, vec!["小红：90 分", "小刚：88 分", "小明：100 分"]);
}

// 测试说明：验证 exercise_fn 处理空列表
#[test]
fn test_exercise_fn_empty() {
    let lines: [&str; 0] = [];
    assert!(exercise_fn(&lines).is_empty());
}