mod exercises;

fn main() {
    println!("=== 泛型枚举 Maybe<T> ===");
    let (a, b, c, d) = exercises::current::exercise_fn();
    println!("Just(5).is_just() = {}", a);
    println!("Just(5).is_nothing() = {}", b);
    println!("Nothing.unwrap_or(3.14) = {}", c);
    println!("Just(\"hi\").clone_or(\"empty\") = {}", d);
    println!("迷你 Option 的四种用法，体验泛型枚举");
}