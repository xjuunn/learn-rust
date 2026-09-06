mod exercises;

fn main() {
    println!("=== 仓库寻货 ===");
    let (a, b) = exercises::current::exercise_fn();
    println!("编号 101 的货架：{:?}", a);
    println!("编号 999 的货架：{:?}", b);
    println!("（Some 表示命中，None 表示缺货）");
}