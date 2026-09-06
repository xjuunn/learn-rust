mod exercises;

fn main() {
    println!("=== 今日天气预报 ===");
    for line in exercises::current::exercise_fn() {
        println!("- {}", line);
    }
}