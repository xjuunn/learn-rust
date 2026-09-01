mod exercises;

fn main() {
    println!("=== 三角形类型判断 ===");
    let cases = [(3, 3, 3), (3, 3, 5), (3, 4, 5), (1, 1, 2)];
    for (a, b, c) in cases {
        let t = exercises::current::exercise_fn(a, b, c);
        println!("边 ({}, {}, {}) -> {}", a, b, c, t);
    }
}
