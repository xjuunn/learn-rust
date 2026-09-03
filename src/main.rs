mod exercises;

fn main() {
    println!("=== 去除元音字母 ===");
    for word in ["hello".to_string(), "Rust语言".to_string(), "AEIOU".to_string()] {
        let result = exercises::current::exercise_fn(word);
        println!("去除元音后: {:?}", result);
    }
}
