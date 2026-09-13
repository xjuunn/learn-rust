mod exercises;

fn main() {
    println!("=== 战棋角色情报面板 ===");
    let (knight_info, mage_info) = exercises::current::exercise_fn();
    println!("[骑士] {}", knight_info);
    println!("[法师] {}", mage_info);
    println!("不同类型实现同一 Trait，即可用同一方法名 describe");
}