mod exercises;

fn main() {
    println!("=== 学生档案 ===");
    let s = exercises::current::Student::new("Alice", 95);
    println!("{:?}", s);
    println!("{}", s.describe());
}
