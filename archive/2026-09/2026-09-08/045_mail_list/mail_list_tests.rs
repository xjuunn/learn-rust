use learn_rust::exercises::current::{count_len, exercise_fn, take_ownership, to_upper};

#[test]
fn test_count_len() {
    // 测试说明：验证只读借用统计字符总数，不移动列表
    let list = vec!["alice".to_string(), "bob".to_string(), "carol".to_string()];
    assert_eq!(count_len(&list), 13);
    assert_eq!(list.len(), 3, "借用不应移动或清空原列表");
}

#[test]
fn test_count_len_empty() {
    // 测试说明：边界情况——空列表字符总数为 0
    let list: Vec<String> = vec![];
    assert_eq!(count_len(&list), 0);
}

#[test]
fn test_to_upper() {
    // 测试说明：验证生成全大写新列表，原列表保持不变且仍可访问
    let list = vec!["alice".to_string(), "bob".to_string()];
    let upper = to_upper(&list);
    assert_eq!(upper, vec!["ALICE".to_string(), "BOB".to_string()]);
    assert_eq!(list, vec!["alice".to_string(), "bob".to_string()], "原列表不得被修改");
}

#[test]
fn test_take_ownership() {
    // 测试说明：验证按值接收列表后返回元素个数
    let list = vec!["x".to_string(), "y".to_string(), "z".to_string()];
    assert_eq!(take_ownership(list), 3);
}

#[test]
fn test_exercise_fn() {
    // 测试说明：验证完整流程返回 (总字符数, 昵称个数, 大写列表)
    let (total, count, upper) = exercise_fn();
    assert_eq!(total, 13);
    assert_eq!(count, 3);
    assert_eq!(upper, vec!["ALICE".to_string(), "BOB".to_string(), "CAROL".to_string()]);
}