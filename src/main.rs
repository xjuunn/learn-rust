mod exercises;

fn main() {
    println!("=== 精灵保险箱 Vault<T>：泛型方法变换类型 ===");
    let (name, count, rating) = exercises::current::exercise_fn();
    println!("箱内宝物: {}", name);
    println!("累计开箱次数: {}", count);
    println!("加工后宝物评级: {}", rating);
    println!("泛型方法使 Vault<String> 变成了 Vault<usize>");
}