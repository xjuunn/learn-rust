mod exercises;

fn main() {
    println!("=== 程序配置加载 ===");
    let config_text = "speed=100\nverbose=true\nname=opencode\ndebug=badline";
    println!("读取配置文件：");
    for line in config_text.lines() {
        println!("  {}", line);
    }
    println!("解析结果:");
    for result in exercises::current::exercise_fn(config_text) {
        println!("  {}", result);
    }
}