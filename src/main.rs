mod exercises;

fn main() {
    println!("=== 仓库入库汇总 ===");
    let stats = exercises::current::exercise_fn(&[
        exercises::current::StockItem { name: "苹果".to_string(), qty: 3 },
        exercises::current::StockItem { name: "香蕉".to_string(), qty: 2 },
        exercises::current::StockItem { name: "苹果".to_string(), qty: 5 },
        exercises::current::StockItem { name: "牛奶".to_string(), qty: 7 },
        exercises::current::StockItem { name: "香蕉".to_string(), qty: 4 },
    ]);
    for (name, total) in &stats {
        println!("  {} x{}", name, total);
    }
}