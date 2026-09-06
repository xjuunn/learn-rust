mod exercises;

fn main() {
    println!("=== 竞技场计分榜 ===");
    let (first, total) = exercises::current::exercise_fn();
    println!("第一名：{}", first);
    println!("全场总得分：{}", total);
    println!("（total_score 借用玩家列表求和，之后再借仍可用，未转移所有权）");
}