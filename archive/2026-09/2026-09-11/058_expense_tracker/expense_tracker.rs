// ============================================
// 题目编号: ex058
// 知识点: 错误处理边界与综合（每日开支统计）
// 难度: 基础
// 所属章节: 06_错误处理
// ============================================
//
// 场景：一个「个人记账本」要统计每日开支。每条记录包含日期和金额文本，
// 金额可能非法（不是数字）、为零或为负。记账本需要把这些坏记录挑出来，
// 计算合法记录的总金额，并把非法记录写成错误提示。
//
// 这是 06 错误处理章节的第八题，重点考察边界情况的处理：
//   - 金额为 0 或负数应视为非法（业务规则边界）
//   - 非法记录不能 panic，要收集成错误列表
//   - 合法记录正常累加
//
// 本题采用「定义函数」形式：
//   - 你需要从 0 实现 parse_amount 和 exercise_fn 两个函数
//   - 函数签名已给出，不可更改；行为规则见下方注释
//
// 再次提醒：函数需要 `pub`（集成测试只能访问 pub 项）。

/// 一条每日开支记录
pub struct DailyRecord {
    pub date: String,
    /// 金额文本，如 "25.5"
    pub amount_text: String,
}

/// 解析金额文本为 f64。
///
/// 规则（边界）：
///   - 必须是合法的浮点数文本（允许小数），否则 Err("金额非法：'{text}'")
///   - 必须 > 0；等于 0 或为负数同样非法，错误信息同上
///
/// 例子：
///   parse_amount("25.5") -> Ok(25.5)
///   parse_amount("8")    -> Ok(8.0)
///   parse_amount("0")    -> Err("金额非法：'0'")
///   parse_amount("-3")   -> Err("金额非法：'-3'")
///   parse_amount("abc")  -> Err("金额非法：'abc'")
pub fn parse_amount(text: &str) -> Result<f64, String> {
    // TODO 1：解析并校验金额（用 trim() 去掉首尾空白再解析）
    // 注意：错误文本里的 text 用原始入参（带引号），例如 "金额非法：'  abc'"
    let amount:f64 = match text.trim().parse() {
        Ok(f) => f,
        Err(_) => return Err(format!("金额非法：'{text}'"))
    };
    if amount > 0.0 {
        return Ok(amount);
    }
    Err(format!("金额非法：'{text}'"))
}

/// 汇总每日开支。
///
/// 规则：
///   - 合法记录金额累加到 total
///   - 非法记录追加一条错误文本："{date} 金额非法：'{amount_text}'"
///     （注意：这里错误文本的引号内使用返回的 Err 消息，直接拼入）
///   - 返回 (total, 错误列表)，不 panic
///
/// 例子：
///   exercise_fn(&[
///       DailyRecord { date: "09-01".into(), amount_text: "25.5".into() },
///       DailyRecord { date: "09-02".into(), amount_text: " 8 ".into() },
///       DailyRecord { date: "09-03".into(), amount_text: "-3".into() },
///       DailyRecord { date: "09-04".into(), amount_text: "abc".into() },
///   ]) ->
///     (33.5,
///      ["09-03 金额非法：'-3'",
///       "09-04 金额非法：'abc'"])
pub fn exercise_fn(records: &[DailyRecord]) -> (f64, Vec<String>) {
    // TODO 2：遍历 records，用 parse_amount 分类处理
    // 提示：match parse_amount(&record.amount_text) { Ok(amount) => 累加, Err(msg) => 收集 }
    let mut sum: f64 = 0.0;
    let mut errs: Vec<String> = Vec::new();
    for item in records {
        match parse_amount(&item.amount_text) {
            Ok(amount) => sum += amount,
            Err(err) => errs.push(format!("{} {err}",item.date)),
        }
    }
    (sum,errs)
}