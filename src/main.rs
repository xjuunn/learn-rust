mod exercises;

fn main() {
    println!("=== 公会演武场战绩榜 Ranker<T> ===");
    let (best, worst, count) = exercises::current::exercise_fn();
    println!("当前登记成绩数: {}", count);
    println!("最高成绩: {}", best);
    println!("最低成绩: {}", worst);
    println!("通用榜单由 T: Clone + PartialOrd 边界支持，i32 / String 均可上榜");
}