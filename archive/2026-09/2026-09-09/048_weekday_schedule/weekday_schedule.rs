// ============================================
// 题目编号: ex048
// 知识点: 集合综合应用（排序与格式化）
// 难度: 基础
// 所属章节: 05_集合类型
// ============================================
//
// 设计一个「周计划日程表」：一周每天有一个任务，请你把任务按**优先级从高到低**
// 排好队，并输出成美观的日程行文本。
//
// 这一题综合考察：
//   - Vec 排序（sort_by 自定义规则）
//   - String 拼接与格式化（format!）
//   - 结构体与字段访问
//
// 下面的骨架代码中，有 3 处关键逻辑被 todo!() 挡住了，请你把它们补全。

/// 一个日程项：星期几、任务描述、优先级（数字越大越紧急）
pub struct Task {
    pub day: String,
    pub desc: String,
    pub urgency: u32,
}

/// 把一周日程按「紧急度」从高到低排序，返回格式化后的文本列表。
/// 每行格式："周X: <任务>（紧急度 N）"，
/// 其中周X 用 day 原本的字符串（如 "周一"、"周二"）。
///
/// 例子：
///   sort_schedule(&[
///       Task { day: "周一".into(), desc: "写报告".into(), urgency: 5 },
///       Task { day: "周三".into(), desc: "开会".into(),   urgency: 9 },
///   ]) ->
///     ["周三: 开会（紧急度 9）", "周一: 写报告（紧急度 5）"]
pub fn sort_schedule(tasks: &[Task]) -> Vec<String> {
    // 第一步：把 tasks 复制一份可变列表，以便排序
    let mut work: Vec<&Task> = tasks.iter().collect();

    // 第二步：按 urgency 降序排序（紧急度高的排前面）
    // 填空 1：这里当前是「升序」，请改为按 urgency 降序比较
    work.sort_by(|a, b| b.urgency.cmp(&a.urgency));

    // 第三步：把排序后的每个任务格式化成一行文本
    // 填空 2：这里的占位只是返回 day，请改为用 format! 生成完整行
    let lines: Vec<String> = work
        .iter()
        .map(|t| format!("{}: {}（紧急度 {}）", t.day,t.desc,t.urgency)) // TODO: 改成 format!("{}: {}（紧急度 {}）", t.day, t.desc, t.urgency)
        .collect();
    lines
}

/// 入口函数：接收任务列表，返回格式化后的日程文本行（已按紧急度排序）。
pub fn exercise_fn(tasks: &[Task]) -> Vec<String> {
    // 填空 3：直接调用 sort_schedule 并把结果返回
    sort_schedule(tasks)
}