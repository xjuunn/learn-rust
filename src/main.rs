mod exercises;

fn main() {
    println!("=== 图书馆借阅系统 ===");
    let catalog = ["活着", "小王子", "百年孤独", "围城"];
    let borrowed = ["小王子"];
    let to_borrow = ["活着", "小王子", "三体"];
    for line in exercises::current::exercise_fn(&catalog, &borrowed, &to_borrow) {
        println!("  {}", line);
    }
}