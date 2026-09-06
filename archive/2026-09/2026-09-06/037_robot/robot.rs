// ============================================
// 题目编号: ex037
// 知识点: 枚举综合应用（从 0 定义）
// 难度: 基础
// 所属章节: 04_枚举与模式匹配
// ============================================

// 题目要求：
// 机器人在网格地图上按指令移动。本题需要你**从 0 定义**一个方向枚举，
// 并实现单步移动与整条路径。
//
// 步骤：
// 1. 定义方向枚举（顶部 derive Debug、PartialEq、Clone、Copy）：
//      pub enum Dir { Up, Down, Left, Right }
#[derive(Copy, Clone)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

// 2. 实现单步移动函数（pub）：
//      pub fn step(pos: (i32, i32), dir: Dir) -> (i32, i32)
//    - Up    -> (x, y + 1)     Down  -> (x, y - 1)
//    - Left  -> (x - 1, y)     Right -> (x + 1, y)
pub fn step(pos: (i32, i32), dir: Dir) -> (i32, i32) {
    let (x, y) = pos;
    match dir {
        Dir::Up => (x, y + 1),
        Dir::Down => (x, y - 1),
        Dir::Left => (x - 1, y),
        Dir::Right => (x + 1, y),
    }
}

// 3. 实现路径函数（pub）：
//      pub fn walk(start: (i32, i32), dirs: &[Dir]) -> (i32, i32)
//    - 从 start 出发，按 dirs 顺序依次移动，返回最终位置
pub fn walk(start: (i32, i32), dirs: &[Dir]) -> (i32, i32) {
    let mut pos = start;
    for &dir in dirs {
        pos = step(pos, dir);
    }
    pos
}

// 4. 实现入口函数（pub），供 main 与测试调用：
//      pub fn exercise_fn() -> (i32, i32)
//    - 从 (0, 0) 出发，按 [Up, Right, Down, Down] 顺序走
//
// 示例：
//   (0,0) --Up--> (0,1) --Right--> (1,1) --Down--> (1,0) --Down--> (1,-1)
//   期望返回: (1, -1)
//
// 提示：
// - 枚举用 enum 定义；既然从 0 定义，就要自己写 derive 和 varient 列表
// - step 里 match dir { Dir::Up => (pos.0, pos.1 + 1), ... }
// - walk 可用 for d in dirs { cur = step(cur, *d); } （dirs 是 &[Dir]，d 是 &Dir）
// - 入口直接 vec![...] 或数组传给 walk
// - 类型与函数都要 pub，否则集成测试访问报 E0603

pub fn exercise_fn() -> (i32, i32) {
    // TODO: 从 (0,0) 出发，按 [Up, Right, Down, Down] 依次走，返回终点
    walk((0, 0), &vec![Dir::Up, Dir::Right, Dir::Down, Dir::Down])
}
