mod exercises;

fn main() {
    println!("=== 矩形计算 ===");
    let (area, perimeter) = exercises::current::exercise_fn(3, 4);
    println!("宽3 高4 的矩形 -> 面积: {}, 周长: {}", area, perimeter);
}
