mod exercises;

fn main() {
    println!("=== 超市收银台 ===");
    let items = [
        exercises::current::CartItem {
            name: "苹果".to_string(),
            price_text: Some("10".to_string()),
            discount_text: Some("0.8".to_string()),
        },
        exercises::current::CartItem {
            name: "牛奶".to_string(),
            price_text: Some("15".to_string()),
            discount_text: None,
        },
        exercises::current::CartItem {
            name: "面包".to_string(),
            price_text: None,
            discount_text: Some("0.9".to_string()),
        },
        exercises::current::CartItem {
            name: "糖果".to_string(),
            price_text: Some("abc".to_string()),
            discount_text: None,
        },
    ];
    for line in exercises::current::exercise_fn(&items) {
        println!("  {}", line);
    }
}