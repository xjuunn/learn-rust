mod exercises;

fn main() {
    println!("=== 借用：去除空白 ===");
    let original = "  hello  ";
    let trimmed = exercises::current::exercise_fn(original);
    println!("原始: {:?}", original);
    println!("去空白: {:?}", trimmed);
    // original 仍可用，因为只是借用
}
