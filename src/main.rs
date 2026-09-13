mod exercises;

fn main() {
    println!("=== 火车站车厢调度区 ===");
    let (top, popped, copy_len, is_empty) = exercises::current::exercise_fn();
    println!("栈顶车厢(只看不取): {}", top.map_or_else(|| "空".to_string(), |v| v.to_string()));
    println!("脱开(弹出)的车厢: {}", popped.map_or_else(|| "空".to_string(), |v| v.to_string()));
    println!("复制出来的车厢数: {}", copy_len);
    println!("原调度区是否已空: {}", is_empty);
    println!("不带 Clone 约束的 impl 没有 dup 方法，方法随约束分裂");
}