use learn_rust::exercises::current::{Grade, exercise_fn, grade};

#[test]
fn test_grade_top_levels() {
    // 测试说明：验证 A/B 边界（90 与 89 相邻）
    assert_eq!(grade(95), Some(Grade::A));
    assert_eq!(grade(90), Some(Grade::A));
    assert_eq!(grade(89), Some(Grade::B));
}

#[test]
fn test_grade_middle_levels() {
    // 测试说明：验证 C/D 边界（75/74、60/59 均为相邻边界）
    assert_eq!(grade(75), Some(Grade::B));
    assert_eq!(grade(74), Some(Grade::C));
    assert_eq!(grade(60), Some(Grade::C));
    assert_eq!(grade(59), Some(Grade::D));
}

#[test]
fn test_grade_low_and_zero() {
    // 测试说明：验证最低档 F，含 0 分端点
    assert_eq!(grade(30), Some(Grade::F));
    assert_eq!(grade(0), Some(Grade::F));
}

#[test]
fn test_grade_out_of_range() {
    // 测试说明：验证超 100 分返回 None（非法输入，不是 F）
    assert_eq!(grade(101), None);
    assert_eq!(grade(200), None);
    assert_eq!(grade(100), Some(Grade::A));
}

#[test]
fn test_exercise_fn() {
    // 测试说明：验证入口函数整体结果（含最后一个 None）
    let expected = vec![
        Some(Grade::A),
        Some(Grade::B),
        Some(Grade::C),
        Some(Grade::D),
        Some(Grade::F),
        None,
    ];
    assert_eq!(exercise_fn(), expected);
}