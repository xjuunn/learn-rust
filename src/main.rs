mod exercises;

fn main() {
    println!("=== 泛型结构体 Pair ===");
    let (a, b, c, d) = exercises::current::exercise_fn();
    println!("Pair<int>::first_ref() = {}", a);
    println!("Pair<f64> 的 second 字段 = {}", b);
    println!("Pair<str>::larger() = {}", c);
    println!("Pair<int>::swap().first = {}", d);
    println!("同一个 Pair<T>，为 int / f64 / str 三款类型而生");
}