mod exercises;

fn main() {
    println!("=== 所有权：String 长度 ===");
    let words = vec!["hello".to_string(), "世界".to_string(), String::new()];
    for w in words {
        let len = exercises::current::exercise_fn(w);
        println!("长度: {} 字节", len);
    }
}
