// ============================================
// 题目编号: ex055
// 知识点: 错误传播
// 难度: 基础
// 所属章节: 06_错误处理
// ============================================
//
// 场景：一个「订单金额统计」工具。订单以文本形式保存，每行一个订单，
// 行内价格用逗号分隔（如 "10.5,20,3.25"）。工具需要把文本解析成金额。
//
// 这是 06 错误处理章节的最后一门知识点（错误传播），重点考察：
//   - `?` 运算符在多层调用链中的应用：错误自动逐层向上传播
//   - parse_price -> sum_order -> exercise_fn 三层结构
//   - 整洁的错误处理：用 ? 替代层层嵌套的 match
//
// 本题采用「定义函数」形式：
//   - parse_price 已经写好（单个价格解析）
//   - 你需要实现 sum_order（一行求和，用 ? 传播解析错误）
//   - 你需要实现 exercise_fn（多行求总计，用 ? 传播求和错误）
//
// 注意：请你用 `?` 运算符做错误传播（而不是 match 后跳过）。
// 行为特征：只要某一行或某个价格非法，整个汇总就应返回 Err，
// 而不是忽略坏数据继续算。

/// 解析单个价格为 f64。非法输入返回 Err("无法解析价格：'xxx'")
///
/// 例子：
///   parse_price("10.5")  -> Ok(10.5)
///   parse_price(" 20 ")  -> Ok(20.0)
///   parse_price("abc")   -> Err("无法解析价格：'abc'")
pub fn parse_price(text: &str) -> Result<f64, String> {
    text.trim()
        .parse::<f64>()
        .map_err(|_| format!("无法解析价格：'{}'", text.trim()))
}

/// 求一行订单的总金额。一行由逗号分隔的多个价格组成。
/// 用 `?` 把 parse_price 的错误直接向上传播。
///
/// 例子：
///   sum_order("10.5,20,3.25") -> Ok(33.75)
///   sum_order("")             -> Ok(0.0)
///   sum_order("10,abc")       -> Err("无法解析价格：'abc'")
pub fn sum_order(line: &str) -> Result<f64, String> {
    // TODO 1：遍历 line.split(',') 得到的每一段，逐段调用 parse_price，
    // 用 ? 传播错误，累加出 total，最后返回 Ok(total)
    let mut total: f64 = 0.0;
    for n in line.split(',') {
        if n.trim().is_empty() {
            continue;
        }
        total += parse_price(n)?;
    }
    Ok(total)
}

/// 汇总所有订单的总金额。每行一个订单，返回 "总金额：{:.2} 元"。
/// 用 `?` 把 sum_order 的错误直接向上传播。
///
/// 例子：
///   exercise_fn("10.5,20,3.25\n5\n0.5,0.5") ->
///     Ok("总金额：39.75 元")
///   exercise_fn("10,abc") ->
///     Err("无法解析价格：'abc'")
///   exercise_fn("") -> Ok("总金额：0.00 元")
pub fn exercise_fn(orders_text: &str) -> Result<String, String> {
    // TODO 2：遍历 orders_text.lines()，逐行调用 sum_order 累加，
    // 用 ? 传播错误，最后返回格式化文本
    let mut sum = 0.0;
    for item in orders_text.lines() {
        sum += sum_order(item)?;
    }
    Ok(format!("总金额：{:.2} 元",sum))
}