mod exercises;

fn main() {
    println!("=== 闰年判断 ===");
    for year in [2000, 1900, 2024, 2023] {
        let result = exercises::current::exercise_fn(year);
        println!("{}年: {}", year, if result { "是闰年" } else { "不是闰年" });
    }
}
