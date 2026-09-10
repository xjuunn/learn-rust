mod exercises;

fn main() {
    println!("=== 订单金额统计 ===");
    let orders_text = "10.5,20,3.25\n5\n0.5,0.5";
    println!("原始订单数据:");
    for line in orders_text.lines() {
        println!("  {}", line);
    }
    match exercises::current::exercise_fn(orders_text) {
        Ok(text) => println!("统计结果: {}", text),
        Err(msg) => println!("统计失败: {}", msg),
    }
}