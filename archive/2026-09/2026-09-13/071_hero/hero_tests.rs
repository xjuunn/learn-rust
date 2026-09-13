use learn_rust::exercises::current::{exercise_fn, Describe, Knight, Mage};

#[test]
fn test_knight_describe() {
    // 测试说明：骑士 describe 输出 "骑士{name} HP{hp}"
    let k = Knight {
        name: String::from("阿德"),
        hp: 120,
    };
    assert_eq!(k.describe(), "骑士阿德 HP120");
}

#[test]
fn test_mage_describe() {
    // 测试说明：法师 describe 输出 "法师{name} MP{mp}"
    let m = Mage {
        name: String::from("露娜"),
        mp: 80,
    };
    assert_eq!(m.describe(), "法师露娜 MP80");
}

#[test]
fn test_describe_edge_empty_name() {
    // 测试说明：空名字与零值也能正常拼接，不 panic
    let k = Knight {
        name: String::new(),
        hp: 0,
    };
    assert_eq!(k.describe(), "骑士 HP0");
    let m = Mage {
        name: String::new(),
        mp: 0,
    };
    assert_eq!(m.describe(), "法师 MP0");
}

#[test]
fn test_same_trait_two_types() {
    // 测试说明：两种不同类型都能调用同一 Describe trait 方法
    let knight = Knight {
        name: String::from("贝拉"),
        hp: 90,
    };
    let mage = Mage {
        name: String::from("赛斯"),
        mp: 150,
    };
    assert!(knight.describe().starts_with("骑士"));
    assert!(mage.describe().starts_with("法师"));
}

#[test]
fn test_exercise_fn_demo() {
    // 测试说明：综合演示返回两位角色的情报
    let (knight_info, mage_info) = exercise_fn();
    assert_eq!(knight_info, "骑士阿德 HP120");
    assert_eq!(mage_info, "法师露娜 MP80");
}