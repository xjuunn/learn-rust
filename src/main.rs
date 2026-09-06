mod exercises;

fn main() {
    println!("=== 铭牌收藏册 ===");
    let (desc, year) = exercises::current::exercise_fn();
    println!("① {}（{} 年）", desc, year);
    let extra = exercises::current::Label::new("山间小径", 1955);
    println!("② {}", extra.describe());
    println!("（登记卡借用字符串而非复制，原字符串仍归调用方所有）");
}