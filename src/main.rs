mod exercises;

fn main() {
    println!("=== 季节的流转 ===");
    let (a, w) = exercises::current::exercise_fn();
    println!("夏去 -> {:?}", a);
    println!("冬尽（周而复始）-> {:?}", w);
}