use learn_rust::exercises::current::{Student, exercise_fn};

#[test]
fn test_derive_debug() {
    // 测试说明：验证 derive(Debug) 后可格式化打印
    let s = Student::new("Alice", 95);
    let dbg = format!("{:?}", s);
    assert!(dbg.contains("Alice") && dbg.contains("95"), "Debug输出应含姓名和分数: {}", dbg);
}

#[test]
fn test_derive_clone() {
    // 测试说明：验证 derive(Clone) 后可克隆
    let s = Student::new("Alice", 95);
    let s2 = s.clone();
    assert_eq!(s2.name, "Alice");
}

#[test]
fn test_derive_partial_eq() {
    // 测试说明：验证 derive(PartialEq) 后可比较相等
    let a = Student::new("Bob", 80);
    let b = Student::new("Bob", 80);
    assert!(a == b, "相同字段的结构体应相等");
    let c = Student::new("Bob", 90);
    assert!(a != c, "分数不同则不相等");
}

#[test]
fn test_describe() {
    // 测试说明：验证 describe 返回 "姓名-分数"
    let s = Student::new("Alice", 95);
    assert_eq!(s.describe(), "Alice-95", "describe 应返回 Alice-95");
}

#[test]
fn test_is_top() {
    // 测试说明：验证 is_top 边界判断
    let s = Student::new("Alice", 95);
    assert!(s.is_top(90), "95>=90 应 true");
    assert!(!s.is_top(100), "95>=100 应 false");
}

#[test]
fn test_exercise_fn() {
    // 测试说明：验证入口函数输出
    assert_eq!(exercise_fn(), "Alice-95 top=? true");
}
