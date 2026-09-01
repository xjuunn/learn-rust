mod exercises;

fn main() {
    println!("=== 奇数求和 ===");
    for n in [10, 7, 0, 20] {
        let sum = exercises::current::exercise_fn(n);
        println!("1 到 {} 的奇数之和: {}", n, sum);
    }
}
