//! 命令行工具：git 提交热力图
//!
//! 用法：
//!   cargo run --bin hotmap                # 读取真实 git 提交记录
//!   cargo run --bin hotmap -- --mock      # 使用一个月模拟数据查看效果

fn main() {
    let mock = std::env::args().any(|a| a == "--mock");
    let code = learn_rust::tools::hotmap::run(mock);
    std::process::exit(code);
}
