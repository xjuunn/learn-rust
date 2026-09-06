use learn_rust::exercises::current::{exercise_fn, unload};

#[test]
fn test_unload_multiple() {
    // 测试说明：验证多个包裹按后进先出取出
    let mut shelf = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    assert_eq!(unload(&mut shelf), vec!["C", "B", "A"]);
}

#[test]
fn test_unload_single() {
    // 测试说明：验证单个包裹
    let mut shelf = vec!["X".to_string()];
    assert_eq!(unload(&mut shelf), vec!["X"]);
}

#[test]
fn test_unload_empty() {
    // 测试说明：边界情况——空货架返回空列表
    let mut shelf: Vec<String> = vec![];
    let result = unload(&mut shelf);
    assert!(result.is_empty(), "空货架应返回空结果");
}

#[test]
fn test_shelf_empty_after() {
    // 测试说明：验证卸载后原货架被取空
    let mut shelf = vec!["P".to_string(), "Q".to_string()];
    unload(&mut shelf);
    assert!(shelf.is_empty(), "卸载后货架应为空");
}

#[test]
fn test_exercise_fn() {
    // 测试说明：验证入口函数整体结果
    assert_eq!(exercise_fn(), vec!["C3", "B2", "A1"]);
}