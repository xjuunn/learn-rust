use learn_rust::exercises::current::{config_bool, config_number, exercise_fn, parse_line, ConfigError};

// 测试说明：验证正常拆分 key=value
#[test]
fn test_parse_line_normal() {
    assert!(matches!(parse_line("speed=100"), Ok(("speed", "100"))));
}

// 测试说明：验证无 '=' 的行返回错误
#[test]
fn test_parse_line_no_equal() {
    assert!(matches!(parse_line("badline"), Err(ConfigError::BadLine(_))));
}

// 测试说明：验证 key 或 value 为空时返回错误（边界情况）
#[test]
fn test_parse_line_empty_part() {
    assert!(matches!(parse_line("=100"), Err(ConfigError::BadLine(_))));
    assert!(matches!(parse_line("key="), Err(ConfigError::BadLine(_))));
}

// 测试说明：验证布尔值正常解析（含去空白）
#[test]
fn test_config_bool_ok() {
    assert!(matches!(config_bool("true"), Ok(true)));
    assert!(matches!(config_bool(" false "), Ok(false)));
}

// 测试说明：验证非法布尔值返回 BadBool
#[test]
fn test_config_bool_err() {
    assert!(matches!(config_bool("yes"), Err(ConfigError::BadBool(_))));
}

// 测试说明：验证整数正常解析（含正负号）
#[test]
fn test_config_number_ok() {
    assert!(matches!(config_number("100"), Ok(100)));
    assert!(matches!(config_number("-5"), Ok(-5)));
}

// 测试说明：验证非法整数返回 BadNumber
#[test]
fn test_config_number_err() {
    assert!(matches!(config_number("abc"), Err(ConfigError::BadNumber(_))));
}

// 测试说明：验证 exercise_fn 综合处理四种情况（bool/整数/文本/格式错误行）
#[test]
fn test_exercise_fn_mixed() {
    let text = "speed=100\nverbose=true\nname=opencode\nbadline";
    let results = exercise_fn(text);
    assert_eq!(
        results,
        vec![
            "speed: 整数 100",
            "verbose: bool 值 true",
            "name: 文本 opencode",
            "跳过：格式错误：'badline'",
        ]
    );
}

// 测试说明：验证 exercise_fn 处理空文本（边界情况）
#[test]
fn test_exercise_fn_empty() {
    assert!(exercise_fn("").is_empty());
}