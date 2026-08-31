mod exercises;

fn main() {
    println!("=== 变量绑定演示 ===");
    let result = exercises::current::exercise_fn();
    println!("计算结果: {}", result);
}
