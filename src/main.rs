mod exercises;

fn main() {
    println!("=== 平方和 ===");
    let data = [1, 2, 3];
    let result = exercises::current::exercise_fn(&data);
    println!("{:?} 各元素平方和: {}", data, result);
}
