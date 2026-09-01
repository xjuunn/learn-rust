use learn_rust::exercises::current::exercise_fn;

#[test]
fn test_secure_password() {
    // 测试说明：验证满足全部条件的密码
    assert_eq!(exercise_fn("Passw0rd"), true, "长度8、含大小写和数字，应安全");
}

#[test]
fn test_no_uppercase() {
    // 测试说明：验证缺少大写字母
    assert_eq!(exercise_fn("password"), false, "没有大写字母，不安全");
}

#[test]
fn test_no_lowercase() {
    // 测试说明：验证缺少小写字母
    assert_eq!(exercise_fn("PASSWORD"), false, "没有小写字母，不安全");
}

#[test]
fn test_short() {
    // 测试说明：验证长度不足
    assert_eq!(exercise_fn("Ab1"), false, "长度不足 8，不安全");
}

#[test]
fn test_no_digit() {
    // 测试说明：验证缺少数字
    assert_eq!(exercise_fn("Password"), false, "没有数字，不安全");
}
