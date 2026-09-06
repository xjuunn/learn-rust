mod exercises;

fn main() {
    println!("=== 快递驿站出货 ===");
    let taken = exercises::current::exercise_fn();
    println!("卸下的包裹（从上到下）：");
    for p in taken {
        println!("  - {}", p);
    }
}