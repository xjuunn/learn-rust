mod exercises;

fn main() {
    println!("=== 冒险者的魔法背包 Bag<T> ===");
    let (popped, left, last) = exercises::current::exercise_fn();
    println!("从背包取出: 物品价值 {}", popped);
    println!("背包剩余件数: {}", left);
    println!("加工后最后一件: 价值 {}", last);
    println!("泛型容器把 i32 物品进来，又加工成了 i32 新物品（Vec<T> 内嵌）");
}