use learn_rust::exercises::current::exercise_fn;

#[test]
fn test_ascii() {
    // 测试说明：验证 ASCII 字符串长度
    let s = String::from("hello");
    assert_eq!(exercise_fn(s), 5, "\"hello\" 是 5 字节");
}

#[test]
fn test_utf8_chinese() {
    // 测试说明：验证中文字符串按字节计算
    let s = String::from("世界");
    assert_eq!(exercise_fn(s), 6, "\"世界\" 每个汉字 3 字节，共 6 字节");
}

#[test]
fn test_empty_string() {
    // 测试说明：验证空字符串
    let s = String::from("");
    assert_eq!(exercise_fn(s), 0, "空字符串为 0 字节");
}
