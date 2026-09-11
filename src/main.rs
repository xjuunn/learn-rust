mod exercises;

fn main() {
    println!("=== 电子钱包转账 ===");
    let mut wallets = vec![
        exercises::current::Account { name: "小明".to_string(), balance: 100.0 },
        exercises::current::Account { name: "小红".to_string(), balance: 50.0 },
    ];
    let transfers: [(&str, &str, f64); 3] =
        [("小明", "小红", 30.0), ("小明", "小刚", 10.0), ("小明", "小红", 0.0)];

    println!("初始账户:");
    for a in &wallets {
        println!("  {}: {:.2} 元", a.name, a.balance);
    }
    println!("执行转账:");
    for line in exercises::current::exercise_fn(&mut wallets, &transfers) {
        println!("  {}", line);
    }
    println!("最终账户:");
    for a in &wallets {
        println!("  {}: {:.2} 元", a.name, a.balance);
    }
}