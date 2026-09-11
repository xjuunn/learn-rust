use learn_rust::exercises::current::{exercise_fn, parse_amount, DailyRecord};

// 测试说明：验证正常金额解析
#[test]
fn test_parse_amount_ok() {
    assert!(matches!(parse_amount("25.5"), Ok(25.5)));
}

// 测试说明：验证金额周围空白会被去除（边界情况）
#[test]
fn test_parse_amount_trim() {
    assert!(matches!(parse_amount(" 8 "), Ok(8.0)));
}

// 测试说明：验证金额为 0 时视为非法（业务边界）
#[test]
fn test_parse_amount_zero() {
    let err = parse_amount("0").unwrap_err();
    assert!(err.contains("金额非法"), "实际: {}", err);
}

// 测试说明：验证负数金额视为非法（业务边界）
#[test]
fn test_parse_amount_negative() {
    let err = parse_amount("-3").unwrap_err();
    assert!(err.contains("金额非法"), "实际: {}", err);
}

// 测试说明：验证非数字文本非法
#[test]
fn test_parse_amount_bad_text() {
    let err = parse_amount("abc").unwrap_err();
    assert!(err.contains("金额非法"), "实际: {}", err);
}

// 测试说明：验证 exercise_fn 混合统计（合法累加 + 非法收集）
#[test]
fn test_exercise_fn_mixed() {
    fn rec(date: &str, amount: &str) -> DailyRecord {
        DailyRecord { date: date.to_string(), amount_text: amount.to_string() }
    }
    let records = [
        rec("09-01", "25.5"),
        rec("09-02", " 8 "),
        rec("09-03", "-3"),
        rec("09-04", "0"),
        rec("09-05", "abc"),
    ];
    let (total, errors) = exercise_fn(&records);
    assert_eq!(total, 33.5);
    assert_eq!(errors.len(), 3);
    assert!(errors[0].contains("09-03") && errors[0].contains("-3"));
    assert!(errors[1].contains("09-04"));
    assert!(errors[2].contains("09-05"));
}

// 测试说明：验证全非法时总金额为 0
#[test]
fn test_exercise_fn_all_bad() {
    fn rec(amount: &str) -> DailyRecord {
        DailyRecord { date: "x".to_string(), amount_text: amount.to_string() }
    }
    let records = [rec("-1"), rec("0"), rec("abc")];
    let (total, errors) = exercise_fn(&records);
    assert_eq!(total, 0.0);
    assert_eq!(errors.len(), 3);
}

// 测试说明：验证空记录列表（边界情况）
#[test]
fn test_exercise_fn_empty() {
    let records: [DailyRecord; 0] = [];
    let (total, errors) = exercise_fn(&records);
    assert_eq!(total, 0.0);
    assert!(errors.is_empty());
}

// 测试说明：验证全部合法时错误列表为空
#[test]
fn test_exercise_fn_all_ok() {
    fn rec(amount: &str) -> DailyRecord {
        DailyRecord { date: "d".to_string(), amount_text: amount.to_string() }
    }
    let records = [rec("1.5"), rec("2"), rec("3.25")];
    let (total, errors) = exercise_fn(&records);
    assert_eq!(total, 6.75);
    assert!(errors.is_empty());
}