mod exercises;

fn main() {
    println!("=== 调色师的配方 ===");
    let a = exercises::current::Color(60, 200, 120);
    let b = exercises::current::Color(140, 30, 220);
    let mixed = a.mix(&b);
    println!("{} 与 {} 调和 -> RGB({}, {}, {})", show(&a), show(&b), mixed.0, mixed.1, mixed.2);
    println!("混合色是否明亮: {}", mixed.is_bright());

    let (r, g, bb) = exercises::current::exercise_fn();
    println!("入口函数返回: RGB({}, {}, {})", r, g, bb);
}

fn show(c: &exercises::current::Color) -> String {
    format!("RGB({}, {}, {})", c.0, c.1, c.2)
}