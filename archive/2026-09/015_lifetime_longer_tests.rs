use learn_rust::exercises::current::exercise_fn;

#[test]
fn test_first_longer() {
    // 测试说明：验证第一个字符串更长时返回 first
    let first = String::from("abc");
    let second = String::from("de");
    assert_eq!(exercise_fn(&first, &second), "abc");
}

#[test]
fn test_second_longer() {
    // 测试说明：验证第二个字符串更长时返回 second
    let first = String::from("a");
    let second = String::from("bcd");
    assert_eq!(exercise_fn(&first, &second), "bcd");
}

#[test]
fn test_equal_length() {
    // 测试说明：验证长度相同时返回 first
    let first = String::from("xy");
    let second = String::from("ab");
    assert_eq!(exercise_fn(&first, &second), "xy");
}
