use learn_rust::exercises::current::exercise_fn;

#[test]
fn test_basic() {
    // 测试说明：验证去除英文小写元音
    let s = String::from("hello");
    assert_eq!(exercise_fn(s), "hll");
}

#[test]
fn test_mixed_case_vowels() {
    // 测试说明：验证大小写元音都被去除
    let s = String::from("AEIOU");
    assert_eq!(exercise_fn(s), "", "全元音应被清空");
}

#[test]
fn test_keep_consonants_and_other() {
    // 测试说明：验证保留辅音、中文等非元音字符
    let s = String::from("Rust语言");
    assert_eq!(exercise_fn(s), "Rst语言", "u 为元音去除，其余保留");
}

#[test]
fn test_special_chars() {
    // 测试说明：验证保留数字、空格、标点，仅去除元音
    let s = String::from("a1 b2 c,d,");
    assert_eq!(exercise_fn(s), "1 b2 c,d,", "仅去除 a，数字空格标点及辅音 b/c/d 保留");
}
