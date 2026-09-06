mod exercises;

fn main() {
    println!("=== 期末成绩单 ===");
    for (score, g) in [95u32, 80, 62, 55, 30, 120]
        .into_iter()
        .zip(exercises::current::exercise_fn())
    {
        match g {
            Some(grade) => println!("{} 分 -> {:?}", score, grade),
            None => println!("{} 分 -> 非法成绩", score),
        }
    }
}