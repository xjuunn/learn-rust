mod exercises;

fn main() {
    println!("=== 每日开支统计 ===");
    let records = [
        exercises::current::DailyRecord { date: "09-01".to_string(), amount_text: "25.5".to_string() },
        exercises::current::DailyRecord { date: "09-02".to_string(), amount_text: " 8 ".to_string() },
        exercises::current::DailyRecord { date: "09-03".to_string(), amount_text: "-3".to_string() },
        exercises::current::DailyRecord { date: "09-04".to_string(), amount_text: "0".to_string() },
        exercises::current::DailyRecord { date: "09-05".to_string(), amount_text: "abc".to_string() },
    ];
    let (total, errors) = exercises::current::exercise_fn(&records);
    println!("本周期合法支出合计: {:.2} 元", total);
    println!("非法记录 {} 条：", errors.len());
    for line in &errors {
        println!("  {}", line);
    }
}