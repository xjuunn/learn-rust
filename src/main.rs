mod exercises;

fn main() {
    println!("=== 移动语义：Vec 求和 ===");
    let v = vec![1, 2, 3];
    let sum = exercises::current::exercise_fn(v);
    println!("Vec 元素之和: {}", sum);
    // 注意：v 的所有权已移动到函数内，这里不能再使用 v
}
