mod exercises;

fn main() {
    println!("=== 全明星投票榜 ===");
    let board = exercises::current::exercise_fn(&[
        exercises::current::Vote { name: "小明".to_string(), votes: 102 },
        exercises::current::Vote { name: "小红".to_string(), votes: 230 },
        exercises::current::Vote { name: "小刚".to_string(), votes: 187 },
        exercises::current::Vote { name: "小美".to_string(), votes: 96 },
    ]);
    for line in board {
        println!("  {}", line);
    }
}