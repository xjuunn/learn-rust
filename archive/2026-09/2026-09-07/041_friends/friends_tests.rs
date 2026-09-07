use learn_rust::exercises::current::{add_friend, exercise_fn, make_list};

#[test]
fn test_make_list() {
    // 测试说明：验证初始名单的三个好友
    assert_eq!(make_list(), vec!["小红".to_string(), "小明".to_string(), "小刚".to_string()]);
}

#[test]
fn test_add_friend_success() {
    // 测试说明：向名单正常追加一个新好友，返回 true
    let mut list = make_list();
    assert!(add_friend(&mut list, "小美"));
    assert_eq!(list, vec!["小红", "小明", "小刚", "小美"].iter().map(|s| s.to_string()).collect::<Vec<_>>());
}

#[test]
fn test_add_duplicate_rejected() {
    // 测试说明：重复好友被拒绝，返回 false 且名单长度不变
    let mut list = make_list();
    assert!(!add_friend(&mut list, "小明"));
    assert_eq!(list.len(), 3);
}

#[test]
fn test_add_empty_name_rejected() {
    // 测试说明：非法空名 "空" 被拒绝，返回 false
    let mut list = make_list();
    assert!(!add_friend(&mut list, "空"));
    assert_eq!(list.len(), 3);
}

#[test]
fn test_exercise_fn() {
    // 测试说明：验证完整流程后名单长度为 5（小美、大壮添加成功，重复与空名被拒）
    assert_eq!(exercise_fn(), 5);
}