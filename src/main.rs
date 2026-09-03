mod exercises;

fn main() {
    println!("=== 计数器：定义结构体与方法 ===");
    let result = exercises::current::compute();
    println!("依次累加 5 + 10 - 3，最终总数: {}", result);
}
