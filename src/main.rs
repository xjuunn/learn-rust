mod exercises;

fn main() {
    println!("=== 填空：翻倍并返回原长度 ===");
    let mut v = vec![1, 2, 3];
    let len = exercises::current::exercise_fn(&mut v);
    println!("元素个数: {}, 翻倍后: {:?}", len, v);
}
