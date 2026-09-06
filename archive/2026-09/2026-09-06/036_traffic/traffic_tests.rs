use learn_rust::exercises::current::{Light, duration, exercise_fn, next_light};

#[test]
fn test_next_red_to_yellow() {
    // 测试说明：验证红灯后应切黄灯
    assert_eq!(next_light(Light::Red), Light::Yellow);
}

#[test]
fn test_next_yellow_to_green() {
    // 测试说明：验证黄灯后应切绿灯
    assert_eq!(next_light(Light::Yellow), Light::Green);
}

#[test]
fn test_next_green_to_red() {
    // 测试说明：验证绿灯后回到红灯（循环）
    assert_eq!(next_light(Light::Green), Light::Red);
}

#[test]
fn test_duration_values() {
    // 测试说明：验证三种信号灯时长
    assert_eq!(duration(Light::Red), 30);
    assert_eq!(duration(Light::Yellow), 5);
    assert_eq!(duration(Light::Green), 35);
}

#[test]
fn test_exercise_fn() {
    // 测试说明：验证入口函数整体结果
    assert_eq!(exercise_fn(), (Light::Yellow, 5));
}