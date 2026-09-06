mod exercises;

fn main() {
    println!("=== 路口信号灯 ===");
    let mut light = exercises::current::Light::Red;
    for _ in 0..4 {
        println!("当前：{:?}", light);
        light = exercises::current::next_light(light);
    }
    let (l, d) = exercises::current::exercise_fn();
    println!("入口演示：next(Red)={:?}，黄灯时长 {}s", l, d);
}