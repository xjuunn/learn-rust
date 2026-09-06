use learn_rust::exercises::current::{Dir, exercise_fn, step, walk};

#[test]
fn test_step_up() {
    // 测试说明：验证向上移动
    assert_eq!(step((0, 0), Dir::Up), (0, 1));
}

#[test]
fn test_step_down() {
    // 测试说明：验证向下移动
    assert_eq!(step((0, 0), Dir::Down), (0, -1));
}

#[test]
fn test_step_left_right() {
    // 测试说明：验证左右移动
    assert_eq!(step((0, 0), Dir::Left), (-1, 0));
    assert_eq!(step((0, 0), Dir::Right), (1, 0));
}

#[test]
fn test_walk_sequence() {
    // 测试说明：验证按多条指令依次移动
    let dirs = [Dir::Up, Dir::Right, Dir::Down];
    assert_eq!(walk((0, 0), &dirs), (1, 0));
}

#[test]
fn test_walk_empty() {
    // 测试说明：边界情况——空指令列表回到起点
    let dirs: [Dir; 0] = [];
    assert_eq!(walk((3, 4), &dirs), (3, 4));
}

#[test]
fn test_exercise_fn() {
    // 测试说明：验证入口函数整体结果
    assert_eq!(exercise_fn(), (1, -1));
}