mod exercises;

fn main() {
    println!("=== 邮件订阅系统 ===");
    let (total, count, upper) = exercises::current::exercise_fn();
    println!("订阅昵称数量: {}", count);
    println!("昵称总字符数: {}", total);
    println!("大写昵称列表: {:?}", upper);
}