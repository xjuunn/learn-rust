// ============================================
// 题目编号: ex049
// 知识点: 集合综合应用（字符串行处理）
// 难度: 基础
// 所属章节: 05_集合类型
// ============================================
//
// 实现一个简单的「文本行处理器」：给一段多行文本，按行拆开，
// 并为每行生成带行号的文本。这是 05 集合章节的收尾综合题，
// 重点考察：
//   - 用 String 的 lines() 分割多行文本
//   - Vec<String> 收集与转换
//   - format! 拼接编号与内容
//
// 请看下面的骨架：3 处逻辑需要你补全。

/// 给定一段多行文本，返回每行带行号的新文本列表。
/// 行号从 1 开始。
///
/// 例子：
///   number_lines("hello\nworld\nrust") ->
///     ["1: hello", "2: world", "3: rust"]
///
/// 注意：空行也算一行（保留编号）；字符串结尾若有多余换行，lines() 会自然忽略。
pub fn number_lines(text: &str) -> Vec<String> {
    // 填空 1：遍历 text.lines()，对每行给出行号（从 1 开始），
    // 用 format! 生成 "行号: 内容"，收集进 Vec 并返回
    // text.lines()
    //     .map(|f| format!("{}: {}", 1, f.to_string()))
    //     .collect()
    let mut list: Vec<String> = Vec::new();
    for (i, item) in text.lines().enumerate() {
        list.push(format!("{}: {}", i + 1, item));
    }
    list
}

/// 入口函数：把输入的多行文本加行号，然后在最前面加一行标题
/// "共 N 行:"，返回完整列表。
///
/// 例子：
///   exercise_fn("a\nb") ->
///     ["共 2 行:", "1: a", "2: b"]
pub fn exercise_fn(text: &str) -> Vec<String> {
    // 填空 2：调用 number_lines 得到有行号的列表，
    // 再在开头插入标题行（标题行内容是 "共 N 行:"，N 为行数），返回
    let mut list = number_lines(text);
    list.insert(0, format!("共 {} 行:", list.len()));
    list
}
