mod exercises;

fn main() {
    println!("=== 泛型比较函数 ===");
    let (a, b, c) = exercises::current::exercise_fn();
    println!("整数比较: max_of(3, 9) = {}", a);
    println!("浮点限制: clamp(7.5, 0.0, 5.0) = {}", b);
    println!("字符串比较: max_of(\"rust\", \"python\") = {}", c);
    println!("同样的代码，三种类型都能用——这就是泛型");
}