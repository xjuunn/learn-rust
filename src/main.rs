mod exercises;

fn main() {
    println!("=== 值班调度中心 ===");
    let (top, count, codes) = exercises::current::exercise_fn();
    println!("今日最高优先级任务: {}", top);
    println!("\"battle\" 任务出现次数: {}", count);
    println!("去重排序后的任务编号: {:?}", codes);
    println!("调度函数不关心任务类型，只要求类型满足约束");
}