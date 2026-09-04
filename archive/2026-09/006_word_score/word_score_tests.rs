use learn_rust::exercises::current::exercise_fn;

#[test]
fn test_lowercase() {
    // 测试说明：验证小写字母评分
    assert_eq!(exercise_fn("abc"), 6, "a=1 b=2 c=3，和为 6");
}

#[test]
fn test_uppercase() {
    // 测试说明：验证大写字母不区分大小写
    assert_eq!(exercise_fn("CAT"), 24, "C=3 A=1 T=20，和为 24");
}

#[test]
fn test_with_space() {
    // 测试说明：验证空格不计分
    assert_eq!(exercise_fn("a b"), 3, "a=1 b=2，空格不计分，和为 3");
}

#[test]
fn test_empty() {
    // 测试说明：验证空字符串返回 0
    assert_eq!(exercise_fn(""), 0, "空字符串评分为 0");
}
