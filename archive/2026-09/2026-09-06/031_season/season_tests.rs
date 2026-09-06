use learn_rust::exercises::current::{Season, exercise_fn};

#[test]
fn test_next_spring() {
    // 测试说明：验证春季下一季是夏季
    assert_eq!(Season::Spring.next(), Season::Summer);
}

#[test]
fn test_next_summer_autumn() {
    // 测试说明：验证夏季下一季是秋季
    assert_eq!(Season::Summer.next(), Season::Autumn);
}

#[test]
fn test_next_autumn_winter() {
    // 测试说明：验证秋季下一季是冬季
    assert_eq!(Season::Autumn.next(), Season::Winter);
}

#[test]
fn test_next_winter_circular() {
    // 测试说明：验证 Winter 下一季循环回 Spring
    assert_eq!(Season::Winter.next(), Season::Spring);
}

#[test]
fn test_next_cycle() {
    // 测试说明：验证沿 cycle 连续调用 next 四次回到原点
    let mut s = Season::Spring;
    for _ in 0..4 {
        s = s.next();
    }
    assert_eq!(s, Season::Spring, "四步应回到春季");
}

#[test]
fn test_exercise_fn() {
    // 测试说明：验证入口函数整体结果
    assert_eq!(exercise_fn(), (Season::Autumn, Season::Spring));
}