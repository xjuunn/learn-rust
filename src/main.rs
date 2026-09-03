mod exercises;

fn main() {
    println!("=== 所有权：修复借用冲突 ===");
    let mut v = vec![2, 3, 4];
    let result = exercises::current::exercise_fn(&mut v);
    println!("处理前过程: 首元素翻倍并追加");
    println!("结果副本: {:?}", result);
    println!("原向量: {:?}", v);
}
