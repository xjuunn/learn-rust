mod exercises;

fn main() {
    println!("=== 班级成绩录入系统 ===");
    let lines = ["小红:90", "小明:abc", " 小刚 : 88 ", "小美", "小强:100"];
    for line in exercises::current::exercise_fn(&lines) {
        println!("  {}", line);
    }
}