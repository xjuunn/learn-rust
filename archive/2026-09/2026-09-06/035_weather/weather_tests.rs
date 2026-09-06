use learn_rust::exercises::current::{Weather, exercise_fn, report};

#[test]
fn test_sunny() {
    // 测试说明：验证晴天播报
    assert_eq!(report(Weather::Sunny), "晴天");
}

#[test]
fn test_windy() {
    // 测试说明：验证刮风播报并绑定风力等级
    assert_eq!(report(Weather::Windy(6)), "刮风，风力 6 级");
}

#[test]
fn test_windy_other_level() {
    // 测试说明：验证不同风力等级绑定正确
    assert_eq!(report(Weather::Windy(3)), "刮风，风力 3 级");
}

#[test]
fn test_rainy() {
    // 测试说明：验证下雨播报并绑定降雨量
    assert_eq!(report(Weather::Rainy(30)), "下雨，降雨 30 毫米");
}

#[test]
fn test_exercise_fn() {
    // 测试说明：验证入口函数整体结果
    assert_eq!(
        exercise_fn(),
        vec!["晴天", "刮风，风力 6 级", "下雨，降雨 30 毫米"]
    );
}