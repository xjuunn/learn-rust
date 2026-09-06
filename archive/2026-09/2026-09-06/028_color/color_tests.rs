use learn_rust::exercises::current::{Color, exercise_fn};

#[test]
fn test_mix_basic() {
    // 测试说明：验证 mix 按分量平均混合
    let a = Color(10, 20, 30);
    let b = Color(20, 40, 60);
    let m = a.mix(&b);
    assert_eq!((m.0, m.1, m.2), (15, 30, 45));
}

#[test]
fn test_mix_keep_originals() {
    // 测试说明：验证 mix 不修改参与混合的两个颜色
    let a = Color(60, 200, 120);
    let b = Color(140, 30, 220);
    let _ = a.mix(&b);
    assert_eq!((a.0, a.1, a.2), (60, 200, 120), "self 不应被修改");
    assert_eq!((b.0, b.1, b.2), (140, 30, 220), "other 不应被修改");
}

#[test]
fn test_mix_down_round() {
    // 测试说明：验证平均时向下取整（和为奇数时）
    let a = Color(1, 1, 5);
    let b = Color(2, 4, 6);
    let m = a.mix(&b);
    assert_eq!((m.0, m.1, m.2), (1, 2, 5), "(1+2)/2=1, (1+4)/2=2, (5+6)/2=5");
}

#[test]
fn test_is_bright_true() {
    // 测试说明：验证平均亮度大于 128 时返回 true
    let c = Color(200, 200, 100);
    assert!(c.is_bright(), "平均亮度 (200+200+100)/3=166 > 128");
}

#[test]
fn test_is_bright_false_boundary() {
    // 测试说明：边界情况——平均恰好 128 时返回 false
    let c = Color(128, 128, 128);
    assert!(!c.is_bright(), "平均 128，要求 > 128 才为 true");
}

#[test]
fn test_exercise_fn() {
    // 测试说明：验证入口函数整体结果
    assert_eq!(exercise_fn(), (100, 115, 170), "混合 (60,200,120) 与 (140,30,220)");
}