mod exercises;

fn main() {
    println!("=== 书店库存 ===");
    let (price, low) = exercises::current::exercise_fn();
    println!("《Rust编程》8折价: {} , 是否低库存: {}", price, low);
}
