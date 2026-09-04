use learn_rust::exercises::current::exercise_fn;

#[test]
fn test_n2() {
    // 测试说明：验证 n=2 的乘法表和
    assert_eq!(exercise_fn(2), 9, "1*1+1*2+2*1+2*2 = 9");
}

#[test]
fn test_n3() {
    // 测试说明：验证 n=3 的乘法表和
    assert_eq!(exercise_fn(3), 36, "1+2+3=6, 6*6=36");
}

#[test]
fn test_n1() {
    // 测试说明：验证 n=1 的边界情况
    assert_eq!(exercise_fn(1), 1, "1*1 = 1");
}

#[test]
fn test_n5() {
    // 测试说明：验证较大的 n 值
    // 1+2+3+4+5=15, 15*15=225
    assert_eq!(exercise_fn(5), 225);
}
