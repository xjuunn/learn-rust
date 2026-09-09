use learn_rust::exercises::current::{exercise_fn, Task};

// 测试说明：验证按紧急度从高到低排序（高紧急度在前）
#[test]
fn test_sort_by_urgency() {
    let result = exercise_fn(&[
        Task { day: "周一".into(), desc: "写报告".into(), urgency: 5 },
        Task { day: "周三".into(), desc: "开会".into(), urgency: 9 },
        Task { day: "周五".into(), desc: "归档".into(), urgency: 1 },
    ]);
    assert_eq!(result[0], "周三: 开会（紧急度 9）");
    assert_eq!(result[1], "周一: 写报告（紧急度 5）");
    assert_eq!(result[2], "周五: 归档（紧急度 1）");
}

// 测试说明：验证输出格式完全一致（含括号与紧急度）
#[test]
fn test_format_exact() {
    let result = exercise_fn(&[
        Task { day: "周二".into(), desc: "复习".into(), urgency: 3 },
    ]);
    assert_eq!(result, vec!["周二: 复习（紧急度 3）"]);
}

// 测试说明：验证多个任务、边界最紧急 10
#[test]
fn test_multiple_and_max() {
    let result = exercise_fn(&[
        Task { day: "周日".into(), desc: "休息".into(), urgency: 0 },
        Task { day: "周六".into(), desc: "紧急修复".into(), urgency: 10 },
    ]);
    assert_eq!(result[0], "周六: 紧急修复（紧急度 10）");
    assert_eq!(result[1], "周日: 休息（紧急度 0）");
}

// 测试说明：验证输入为空时返回空列表
#[test]
fn test_empty() {
    let result = exercise_fn(&[]);
    assert!(result.is_empty(), "空输入应返回空列表");
}

// 测试说明：验证相同紧急度时顺序相对稳定（不要求特定顺序，但都能出现）
#[test]
fn test_equal_urgency_both_present() {
    let result = exercise_fn(&[
        Task { day: "一".into(), desc: "甲".into(), urgency: 4 },
        Task { day: "二".into(), desc: "乙".into(), urgency: 4 },
    ]);
    assert_eq!(result.len(), 2);
    // 两条记录都出现在结果中（不关心先后，因为排序不稳定也不影响）
    let joined = result.join("|");
    assert!(joined.contains("甲") && joined.contains("乙"));
}