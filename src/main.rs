mod exercises;

fn main() {
    println!("=== 周计划日程表 ===");
    let lines = exercises::current::exercise_fn(&[
        exercises::current::Task { day: "周一".to_string(), desc: "写学习报告".to_string(), urgency: 5 },
        exercises::current::Task { day: "周二".to_string(), desc: "修复线上 bug".to_string(), urgency: 9 },
        exercises::current::Task { day: "周三".to_string(), desc: "参加项目评审".to_string(), urgency: 7 },
        exercises::current::Task { day: "周四".to_string(), desc: "整理文档".to_string(), urgency: 2 },
        exercises::current::Task { day: "周五".to_string(), desc: "代码走查".to_string(), urgency: 4 },
    ]);
    for line in lines {
        println!("  {}", line);
    }
}