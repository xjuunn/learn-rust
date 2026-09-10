use learn_rust::exercises::current::{exercise_fn, parse_price, sum_order};

// 测试说明：验证单个价格正常解析
#[test]
fn test_parse_price_ok() {
    assert_eq!(parse_price("10.5"), Ok(10.5));
}

// 测试说明：验证价格周围的空白会被去除
#[test]
fn test_parse_price_trim() {
    assert_eq!(parse_price(" 20 "), Ok(20.0));
}

// 测试说明：验证非法价格解析失败
#[test]
fn test_parse_price_err() {
    let err = parse_price("abc").unwrap_err();
    assert!(err.contains("无法解析价格"), "实际: {}", err);
}

// 测试说明：验证一行订单正常求和
#[test]
fn test_sum_order_normal() {
    assert_eq!(sum_order("10.5,20,3.25"), Ok(33.75));
}

// 测试说明：验证空行求和为 0（边界情况）
#[test]
fn test_sum_order_empty() {
    assert_eq!(sum_order(""), Ok(0.0));
}

// 测试说明：验证一行中某个价格非法时错误传播到 sum_order
#[test]
fn test_sum_order_err_propagates() {
    let err = sum_order("10,abc").unwrap_err();
    assert!(err.contains("无法解析价格"), "错误应传播到上层，实际: {}", err);
}

// 测试说明：验证多行订单正常汇总
#[test]
fn test_exercise_fn_normal() {
    let result = exercise_fn("10.5,20,3.25\n5\n0.5,0.5").unwrap();
    assert_eq!(result, "总金额：39.75 元");
}

// 测试说明：验证空文本汇总为 0.00（边界情况）
#[test]
fn test_exercise_fn_empty() {
    let result = exercise_fn("").unwrap();
    assert_eq!(result, "总金额：0.00 元");
}

// 测试说明：验证某行非法时错误传播到 exercise_fn（不跳过坏数据）
#[test]
fn test_exercise_fn_err_propagates() {
    let err = exercise_fn("10,20\nabc").unwrap_err();
    assert!(err.contains("无法解析价格"), "错误应传播到最上层，实际: {}", err);
}