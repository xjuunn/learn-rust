mod exercises;

fn main() {
    println!("=== 遮蔽演示 ===");
    println!("字符串 \"hello\" 经过遮蔽和类型转换：");
    let result = exercises::current::exercise_fn();
    println!("最终结果: {}", result);
}
