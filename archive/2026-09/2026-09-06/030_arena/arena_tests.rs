use learn_rust::exercises::current::{PlayerScore, exercise_fn, total_score};

#[test]
fn test_new_initial_zero() {
    // 测试说明：验证新玩家初始分数为 0
    let p = PlayerScore::new("汉克");
    assert_eq!(p.name, "汉克");
    assert_eq!(p.score, 0, "初始分数应为 0");
}

#[test]
fn test_add_points() {
    // 测试说明：验证 add_points 可变借用累加分数
    let mut p = PlayerScore::new("格罗特");
    p.add_points(10);
    p.add_points(15);
    assert_eq!(p.score, 25);
}

#[test]
fn test_show_format() {
    // 测试说明：验证 show 输出 "名字: 分数" 格式
    let mut p = PlayerScore::new("兰博");
    p.add_points(42);
    assert_eq!(p.show(), "兰博: 42");
}

#[test]
fn test_total_score() {
    // 测试说明：验证 total_score 借切片求和
    let mut a = PlayerScore::new("A");
    let mut b = PlayerScore::new("B");
    a.add_points(20);
    b.add_points(35);
    let players = vec![a, b];
    assert_eq!(total_score(&players), 55);
}

#[test]
fn test_borrow_not_move() {
    // 测试说明：验证借用不转移所有权——求和后 players 仍可使用
    let players = vec![PlayerScore::new("X"), PlayerScore::new("Y")];
    let total = total_score(&players);
    assert_eq!(total, 0);
    assert_eq!(players.len(), 2, "借用后原 Vec 仍归调用方所有");
}

#[test]
fn test_exercise_fn() {
    // 测试说明：验证入口函数整体结果
    assert_eq!(exercise_fn(), ("汉克: 30".to_string(), 55));
}