mod exercises;

fn main() {
    println!("=== 自动售货机结算 ===");
    let orders = [(3.5, 10.0), (2.0, 2.0), (10.0, 3.0), (5.5, 5.0)];
    for line in exercises::current::exercise_fn(&orders) {
        println!("  {}", line);
    }
}