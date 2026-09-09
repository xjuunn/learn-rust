mod exercises;

fn main() {
    println!("=== 书架归档 ===");
    let shelf = exercises::current::archive(&[
        ("三体", "科幻"),
        ("活着", "文学"),
        ("沙丘", "科幻"),
        ("百年孤独", "文学"),
        ("小王子", "文学"),
    ]);
    for (category, books) in &shelf {
        println!("【{}】 {} 本:", category, books.len());
        for b in books {
            println!("  - {}", b);
        }
    }
}