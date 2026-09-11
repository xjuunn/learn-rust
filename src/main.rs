mod exercises;

fn main() {
    println!("=== 餐厅结账系统 ===");
    let menu = [
        exercises::current::Dish { name: "宫保鸡丁".to_string(), price: 28.0 },
        exercises::current::Dish { name: "麻婆豆腐".to_string(), price: 18.0 },
        exercises::current::Dish { name: "米饭".to_string(), price: 3.0 },
    ];
    let orders: Vec<Vec<exercises::current::OrderItem>> = vec![
        vec![
            exercises::current::OrderItem { name: "宫保鸡丁".to_string(), count: 2 },
            exercises::current::OrderItem { name: "米饭".to_string(), count: 3 },
        ],
        vec![
            exercises::current::OrderItem { name: "红烧肉".to_string(), count: 1 },
        ],
        vec![
            exercises::current::OrderItem { name: "麻婆豆腐".to_string(), count: 0 },
        ],
    ];
    let refs: Vec<&[exercises::current::OrderItem]> = orders.iter().map(|v| v.as_slice()).collect();
    for line in exercises::current::exercise_fn(&menu, &refs) {
        println!("  {}", line);
    }
}