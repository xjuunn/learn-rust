mod exercises;

fn main() {
    println!("=== 乘法表总和 ===");
    for n in 1..=5 {
        let sum = exercises::current::exercise_fn(n);
        println!("1..={} 乘法表总和: {}", n, sum);
    }
}
