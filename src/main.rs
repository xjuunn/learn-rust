mod exercises;

fn main() {
    println!("=== 两点距离 ===");
    let d = exercises::current::exercise_fn(0.0, 0.0, 3.0, 4.0);
    println!("(0,0) 到 (3,4) 的直线距离: {}", d);
}
