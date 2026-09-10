use learn_rust::exercises::current::{exercise_fn, make_change};

// 测试说明：验证正好付款（边界情况，找零为 0）
#[test]
fn test_exact_payment() {
    assert_eq!(make_change(5.0, 5.0), Ok(0.0));
}

// 测试说明：验证付款充足，正常找零
#[test]
fn test_sufficient_payment() {
    assert_eq!(make_change(3.5, 10.0), Ok(6.5));
}

// 测试说明：验证付款不足时返回 Err 且包含提示信息
#[test]
fn test_insufficient_payment() {
    let err = make_change(10.0, 3.0);
    assert!(err.is_err(), "付款不足应返回 Err");
    let msg = err.unwrap_err();
    assert!(msg.contains("付款不足"), "错误信息应包含'付款不足'，实际: {}", msg);
    assert!(msg.contains("7.0"), "错误信息应包含差额 7.0，实际: {}", msg);
}

// 测试说明：验证 exercise_fn 正常订单的结算文本
#[test]
fn test_exercise_fn_normal() {
    let orders = [(3.5, 10.0), (5.0, 5.0)];
    let results = exercise_fn(&orders);
    assert_eq!(results, vec!["找零 6.5 元", "找零 0.0 元"]);
}

// 测试说明：验证 exercise_fn 遇到付款不足订单不崩溃并返回提示
#[test]
fn test_exercise_fn_with_error() {
    let orders = [(10.0, 3.0), (2.0, 2.0)];
    let results = exercise_fn(&orders);
    assert!(results[0].contains("无法完成"), "错误订单应输出'无法完成'");
    assert!(results[0].contains("付款不足"), "应保留错误信息");
    assert_eq!(results[1], "找零 0.0 元", "正常订单不受影响");
}

// 测试说明：验证 exercise_fn 处理空订单列表
#[test]
fn test_exercise_fn_empty() {
    let orders: [(f64, f64); 0] = [];
    assert!(exercise_fn(&orders).is_empty());
}