use learn_rust::exercises::current::Point;
use learn_rust::exercises::current::exercise_fn;

const EPSILON: f64 = 1e-9;

fn assert_close(a: f64, b: f64, msg: &str) {
    assert!((a - b).abs() < EPSILON, "{}: 期望 {}, 实际 {}", msg, b, a);
}

#[test]
fn test_distance_pythagorean() {
    // 测试说明：验证勾股数 3-4-5 的距离
    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(3.0, 4.0);
    assert_close(p1.distance(&p2), 5.0, "距离应为 5");
}

#[test]
fn test_same_point() {
    // 测试说明：验证同一点距离为 0
    let p1 = Point::new(1.0, 1.0);
    let p2 = Point::new(1.0, 1.0);
    assert_close(p1.distance(&p2), 0.0, "同点距离为 0");
}

#[test]
fn test_negative_coords() {
    // 测试说明：验证负坐标下的距离
    let p1 = Point::new(-1.0, -1.0);
    let p2 = Point::new(2.0, 3.0);
    assert_close(p1.distance(&p2), 5.0, "(-1,-1)到(2,3)距离为5");
}

#[test]
fn test_exercise_fn() {
    // 测试说明：验证入口函数返回距离
    assert_close(exercise_fn(0.0, 0.0, 3.0, 4.0), 5.0, "勾股距离5");
    assert_close(exercise_fn(1.0, 1.0, 1.0, 1.0), 0.0, "同点距离0");
}
