use learn_rust::exercises::current::{Item, cheaper, exercise_fn};

#[test]
fn test_new_and_describe() {
    // 测试说明：验证 new 与 describe 的格式
    let item = Item::new("剑", 100, 3);
    assert_eq!(item.describe(), "剑 x3 每件100金币", "describe 格式应为 名称 x库存 每件price金币");
}

#[test]
fn test_sell_success() {
    // 测试说明：验证库存足够时 sell 成功并扣减库存
    let mut item = Item::new("箭", 5, 10);
    assert!(item.sell(4), "库存足够时应返回 true");
    assert_eq!(item.stock, 6, "卖出 4 支后库存应为 6");
}

#[test]
fn test_sell_fail_keeps_stock() {
    // 测试说明：验证库存不足时 sell 失败且库存不变
    let mut item = Item::new("箭", 5, 2);
    assert!(!item.sell(3), "库存不足时应返回 false");
    assert_eq!(item.stock, 2, "失败时库存不应变化");
}

#[test]
fn test_total_value() {
    // 测试说明：验证库存总价值计算
    let item = Item::new("盾", 80, 5);
    assert_eq!(item.total_value(), 400, "80 * 5 = 400");
}

#[test]
fn test_cheaper() {
    // 测试说明：验证 cheaper 返回单价较低的引用
    let a = Item::new("剑", 100, 3);
    let b = Item::new("盾", 80, 5);
    let cheap = cheaper(&a, &b);
    assert_eq!(cheap.name, "盾", "80 < 100，便宜的是盾");
}

#[test]
fn test_cheaper_same_price() {
    // 测试说明：验证价格相同时返回第一个参数
    let a = Item::new("剑", 100, 3);
    let b = Item::new("矛", 100, 4);
    let cheap = cheaper(&a, &b);
    assert_eq!(cheap.name, "剑", "价格相同应返回第一个参数 a");
}

#[test]
fn test_exercise_fn() {
    // 测试说明：验证入口函数整体结果
    assert_eq!(exercise_fn(), ("盾".to_string(), 600), "卖出1把剑后总价值 100*2+80*5=600");
}