mod exercises;

fn main() {
    println!("=== 数组安全访问器 ===");
    let arr = [10, 20, 30, 40, 50];
    let indices = [0, 2, 4];
    println!("数组: {:?}", arr);
    println!("访问索引: {:?}", indices);
    let results = exercises::current::exercise_fn(&arr, &indices);
    println!("访问结果: {:?}", results);
}
