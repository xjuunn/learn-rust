mod exercises;

fn main() {
    println!("=== 词语评分器 ===");
    for word in ["abc", "CAT", "a b", "rust"] {
        let score = exercises::current::exercise_fn(word);
        println!("\"{}\" 的评分为 {}", word, score);
    }
}
