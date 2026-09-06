mod exercises;

fn main() {
    println!("=== 存钱罐点钞 ===");
    let coins = exercises::current::exercise_fn();
    println!("四种硬币的面值：{:?}", coins);
    let total: u32 = coins.iter().sum();
    println!("合计：{} 美分", total);
}