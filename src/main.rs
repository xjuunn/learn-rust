mod exercises;

fn main() {
    println!("=== 机器人巡检轨迹 ===");
    let (x, y) = exercises::current::exercise_fn();
    println!("执行 [Up, Right, Down, Down] 后到达：({}, {})", x, y);
    let left = exercises::current::step((1, 1), exercises::current::Dir::Left);
    println!("从 (1,1) 向左一步到达：({}, {})", left.0, left.1);
}