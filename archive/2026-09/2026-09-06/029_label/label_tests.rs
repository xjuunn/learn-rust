use learn_rust::exercises::current::{Label, exercise_fn};

#[test]
fn test_new_fields() {
    // 测试说明：验证 new 关联函数构造的字段
    let l = Label::new("湖畔山居", 1900);
    assert_eq!(l.title, "湖畔山居");
    assert_eq!(l.year, 1900);
}

#[test]
fn test_describe_format() {
    // 测试说明：验证 describe 输出《标题》- 年份格式
    let l = Label::new("山间小径", 1955);
    assert_eq!(l.describe(), "《山间小径》 - 1955");
}

#[test]
fn test_borrow_not_copy() {
    // 测试说明：验证结构体借用字符串后，原字符串仍归调用方所有，可继续使用
    let owned = String::from("森林驿站");
    let label = Label::new(&owned, 2001);
    assert_eq!(label.title, "森林驿站");
    assert_eq!(owned, "森林驿站", "借用不应转移所有权");
}

#[test]
fn test_different_inputs() {
    // 测试说明：验证不同输入都能正确构造与描述
    let l = Label::new("远眺之塔", 1888);
    assert_eq!(l.describe(), "《远眺之塔》 - 1888");
}

#[test]
fn test_exercise_fn() {
    // 测试说明：验证入口函数整体结果
    assert_eq!(exercise_fn(), ("《静谧湖面》 - 1892".to_string(), 1892));
}