mod exercises;

fn main() {
    println!("=== 词频统计器 ===");
    let stats = exercises::current::exercise_fn(&[
        "the quick brown fox",
        "the lazy dog",
        "the fox jumps",
    ]);
    println!("单词出现次数:");
    for (word, count) in &stats {
        println!("  {:<8} x{}", word, count);
    }
}
