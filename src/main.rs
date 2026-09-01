mod exercises;

fn main() {
    println!("=== 函数调用与数组求和 ===");
    let data = [1, 2, 3, 4, 5];
    let result = exercises::current::exercise_fn(&data);
    println!("对数组 {:?} 求和的结果: {}", data, result);
}
