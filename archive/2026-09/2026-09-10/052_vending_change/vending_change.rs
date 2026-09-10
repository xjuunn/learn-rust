// ============================================
// 题目编号: ex052
// 知识点: Result 可恢复错误
// 难度: 基础
// 所属章节: 06_错误处理
// ============================================
//
// 场景：一个「自动售货机」在处理订单。每个订单由 (货品价格, 顾客付款) 组成，
// 需要计算找零。当付款不足时，返回 Err 而不是负数。
//
// 这是 06 错误处理章节的第二题，重点考察：
//   - Result<T, E> 枚举的构造（Ok / Err）
//   - Result 的安全解构（match），避免 unwrap 直接 panic
//   - 错误的格式化信息
//
// 下面的代码有 2 处 bug 需要你修复（题目已用 // BUG 标注行号），
// 修复后保证文档注释中列出的行为都能满足。

/// 计算找零。付款不足时返回 Err，错误信息格式：
///     Err("付款不足，还差 {:.1} 元")
///
/// 例子：
///   make_change(3.5, 10.0) -> Ok(6.5)
///   make_change(10.0, 3.0) -> Err("付款不足，还差 7.0 元")
///   make_change(5.0, 5.0)  -> Ok(0.0)
pub fn make_change(price: f64, paid: f64) -> Result<f64, String> {
    if paid < price {
        // BUG 1：这里本应返回 Err，却返回了 Ok(负数找零)，请修复
        // Ok(paid - price)
        Err(format!("付款不足，还差 {:.1} 元", price - paid))
    } else {
        Ok(paid - price)
    }
}

/// 处理一批订单，返回每个订单的结算文本。
///
/// 例子：
///   exercise_fn(&[(3.5, 10.0), (10.0, 3.0)]) ->
///     ["找零 6.5 元", "无法完成：付款不足，还差 7.0 元"]
///
/// 说明：付款不足的订单应输出 "无法完成：{错误信息}"，
/// 不能令程序崩溃（panic）。
pub fn exercise_fn(orders: &[(f64, f64)]) -> Vec<String> {
    orders
        .iter()
        .map(|&(price, paid)| {
            // BUG 2：match 分支没有写完整，目前错误订单会 panic，请补全
            match make_change(price, paid) {
                Ok(change) => format!("找零 {:.1} 元", change),
                Err(error) => format!("无法完成：{}", error)
            }
            
        })
        .collect()
}