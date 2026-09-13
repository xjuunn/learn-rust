mod exercises;

fn main() {
    println!("=== 竞技场开战前 ===");
    let (g, m, e) = exercises::current::exercise_fn();
    println!("[骑士] {}", g);
    println!("[法师] {}", m);
    println!("[敌人] {}", e);
    println!("invite 的入参限定为任意实现 Crier 的类型");
}