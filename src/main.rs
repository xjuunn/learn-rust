mod exercises;

fn main() {
    println!("=== 克隆：元素翻倍 ===");
    let original = vec![1, 2, 3];
    let doubled = exercises::current::exercise_fn(original.clone());
    println!("原始: {:?}", original);
    println!("翻倍: {:?}", doubled);
    // original 仍可被使用，因为传的是克隆
}
