use learn_rust::exercises::current::exercise_fn;

#[test]
fn test_trim_ascii() {
    // 测试说明：验证去掉 ASCII 空白
    let s = "  hello  ";
    assert_eq!(exercise_fn(s), "hello");
}

#[test]
fn test_trim_chinese() {
    // 测试说明：验证去掉中文空白
    let s = "  世界  ";
    assert_eq!(exercise_fn(s), "世界");
}

#[test]
fn test_all_whitespace() {
    // 测试说明：验证全空白字符串
    let s = "     ";
    assert_eq!(exercise_fn(s), "");
}

#[test]
fn test_no_whitespace() {
    // 测试说明：验证无空白时原样返回
    let s = "rust";
    assert_eq!(exercise_fn(s), "rust");
}
