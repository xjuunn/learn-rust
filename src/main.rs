mod exercises;

fn main() {
    println!("=== const 泛型：定长容器 ===");
    let (a, b, c, d) = exercises::current::exercise_fn();
    println!("FixedArray<i32, 3> 的第 3 号元素 = {}", a);
    println!("容器 A 长度 = {}", b);
    println!("FixedArray<f64, 5> 长度 = {}", c);
    println!("identity::<4>() 的下标数组索引 2 = {}", d);
    println!("长度 N 是类型的一部分，N=3 与 N=5 是不同的类型");
}