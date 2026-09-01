mod exercises;

fn main() {
    println!("=== 生命周期：较长字符串 ===");
    let a = String::from("hello");
    let b = String::from("rust");
    let longer = exercises::current::exercise_fn(&a, &b);
    println!("{:?} 和 {:?} 中较长的是 {:?}", a, b, longer);
}
