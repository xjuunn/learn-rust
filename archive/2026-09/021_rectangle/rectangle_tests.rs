use learn_rust::exercises::current::{Rectangle, exercise_fn};

#[test]
fn test_rectangle_basic() {
    // 测试说明：验证矩形面积和周长
    let r = Rectangle { width: 3, height: 4 };
    assert_eq!(r.area(), 12, "面积 3*4=12");
    assert_eq!(r.perimeter(), 14, "周长 (3+4)*2=14");
}

#[test]
fn test_square() {
    // 测试说明：验证正方形（宽高相等）
    let r = Rectangle { width: 5, height: 5 };
    assert_eq!(r.area(), 25, "面积 5*5=25");
    assert_eq!(r.perimeter(), 20, "周长 (5+5)*2=20");
}

#[test]
fn test_exercise_fn() {
    // 测试说明：验证入口函数返回 (面积, 周长) 元组
    assert_eq!(exercise_fn(3, 4), (12, 14));
    assert_eq!(exercise_fn(10, 2), (20, 24));
}
