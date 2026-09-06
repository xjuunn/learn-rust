mod exercises;

fn main() {
    println!("=== 平安夜的礼物篮 ===");
    let (a, b) = exercises::current::exercise_fn();
    println!("Apple(5) 的苹果数：{}", a);
    println!("Flower 的苹果数：{}", b);
    let gold = exercises::current::Gift::Gold;
    println!("Gold 的苹果数：{}", exercises::current::apple_count(&gold));
    println!("（if let 只处理 Apple 变体，其余静默按 0 计）");
}