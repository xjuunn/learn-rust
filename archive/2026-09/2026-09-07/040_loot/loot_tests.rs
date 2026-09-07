use learn_rust::exercises::current::{Loot, exercise_fn, settle};

#[test]
fn test_gold_binding() {
    // 测试说明：验证 Gold 变体携带的金币数量被绑定并格式化
    assert_eq!(settle(Loot::Gold(50)), "获得 50 金币");
    assert_eq!(settle(Loot::Gold(120)), "获得 120 金币");
}

#[test]
fn test_item_binding() {
    // 测试说明：验证 Item 变体携带的物品名称被绑定并格式化
    assert_eq!(settle(Loot::Item("铁剑".to_string())), "获得物品：铁剑");
    assert_eq!(settle(Loot::Item("回复药水".to_string())), "获得物品：回复药水");
}

#[test]
fn test_nothing() {
    // 测试说明：验证 Nothing 变体输出空手而归的文案
    assert_eq!(settle(Loot::Nothing), "一无所获");
}

#[test]
fn test_gold_edge_case_zero() {
    // 测试说明：边界情况——掉落了 0 金币也应正确输出
    assert_eq!(settle(Loot::Gold(0)), "获得 0 金币");
}

#[test]
fn test_exercise_fn() {
    // 测试说明：验证入口函数对整组掉落物的结算顺序
    let expected = vec![
        "获得 50 金币".to_string(),
        "获得物品：铁剑".to_string(),
        "一无所获".to_string(),
        "获得 120 金币".to_string(),
        "获得物品：回复药水".to_string(),
    ];
    assert_eq!(exercise_fn(), expected);
}