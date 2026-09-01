mod exercises;

fn main() {
    println!("=== 混合类型运算 ===");
    let a = 3;
    let b = 2.5;
    let c = 2;
    let result = exercises::current::exercise_fn(a, b, c);
    println!("{} + {} + {} = {}", a, b, c, result);
}
