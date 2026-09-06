use learn_rust::exercises::current::{Knight, exercise_fn};

#[test]
fn test_new() {
    // 测试说明：验证 new 关联函数构造的字段
    let k = Knight::new("亚瑟", 100, 30);
    assert_eq!(k.name, "亚瑟");
    assert_eq!((k.hp, k.atk), (100, 30));
}

#[test]
fn test_take_damage() {
    // 测试说明：验证受伤后生命扣减
    let mut k = Knight::new("兰斯洛特", 50, 20);
    k.take_damage(20);
    assert_eq!(k.hp, 30, "50 - 20 = 30");
}

#[test]
fn test_take_damage_not_negative() {
    // 测试说明：验证超量伤害时生命最低为 0，不出现负数
    let mut k = Knight::new("高文", 10, 5);
    k.take_damage(99);
    assert_eq!(k.hp, 0, "生命不能小于 0");
}

#[test]
fn test_clone_independent() {
    // 测试说明：验证克隆副本与原本相互独立
    let original = Knight::new("崔斯坦", 80, 25);
    let mut copy = original.clone();
    copy.take_damage(30);
    assert_eq!(original.hp, 80, "修改副本不应影响原本");
    assert_eq!(copy.hp, 50, "副本受伤后生命为 80 - 30 = 50");
}

#[test]
fn test_exercise_fn() {
    // 测试说明：验证入口函数整体结果
    assert_eq!(exercise_fn(), ("亚瑟".to_string(), 100, 75), "原本 100，副本 75");
}