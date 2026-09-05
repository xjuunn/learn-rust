mod exercises;

fn main() {
    println!("=== 铁匠铺今日账目 ===");
    let sword = exercises::current::Item::new("剑", 100, 3);
    println!("商品：{}", sword.describe());
    let (cheap_name, total) = exercises::current::exercise_fn();
    println!("今日平价推荐：{}", cheap_name);
    println!("库存总价值：{} 金币", total);
}