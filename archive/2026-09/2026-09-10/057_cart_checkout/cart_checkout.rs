// ============================================
// 题目编号: ex057
// 知识点: Result 综合消费（购物车折扣结算）
// 难度: 基础
// 所属章节: 06_错误处理
// ============================================
//
// 场景：一个「超市收银台」在结算购物车。每件商品有原价文本和折扣文本，
// 原价可能缺失或非法，折扣缺失时按无折扣（原价）计算。
//
// 这是 06 错误处理章节的第七题，重点考察对 Result/Option 的"消费"技巧：
//   - unwrap_or / unwrap_or_else：给缺失的值一个默认值
//   - as_deref()：把 Option<String> 变成 Option<&str>
//   - 用 Err 表达失败而非 panic
//
// 本题采用「修复 bug」形式：
//   - parse_price / parse_discount 已经写好（正确）
//   - exercise_fn 入口已经写好（正确）
//   - item_final 里有 2 处 bug 需要你找出并修复（已用 // BUG 标注）

/// 一件待结算的商品
pub struct CartItem {
    pub name: String,
    /// 原价文本，可能缺失
    pub price_text: Option<String>,
    /// 折扣率文本（如 "0.8" 表示八折），缺失时按原价
    pub discount_text: Option<String>,
}

/// 解析价格文本为 f64。非法返回 Err("价格非法：'xxx'")
///
/// 例子：parse_price("12.5") -> Ok(12.5)；parse_price("abc") -> Err(...)
pub fn parse_price(text: &str) -> Result<f64, String> {
    text.trim()
        .parse::<f64>()
        .map_err(|_| format!("价格非法：'{}'", text.trim()))
}

/// 解析折扣率。必须介于 0.0 ~ 1.0，否则 Err("折扣比例非法：'xxx'")
///
/// 例子：parse_discount("0.8") -> Ok(0.8)；parse_discount("2") -> Err(...)
pub fn parse_discount(text: &str) -> Result<f64, String> {
    let d = text
        .trim()
        .parse::<f64>()
        .map_err(|_| format!("折扣比例非法：'{}'", text.trim()))?;
    if !(0.0..=1.0).contains(&d) {
        return Err(format!("折扣比例非法：'{}'", d));
    }
    Ok(d)
}

/// 计算一件商品的最终价并返回结算文本。
///
/// 成功 -> Ok("{name} 最终价 {:.2} 元（原价 {:.2} 元）")
/// 原价缺失 -> Err("跳过：{name} 缺价格")
/// 原价/折扣非法 -> Err("{具体的解析错误}")
///
/// 例子：
///   item_final(CartItem { name: "苹果", price_text: Some("10"), discount_text: Some("0.8") }) ->
///     Ok("苹果 最终价 8.00 元（原价 10.00 元）")
///   item_final(CartItem { name: "牛奶", price_text: Some("15"), discount_text: None }) ->
///     Ok("牛奶 最终价 15.00 元（原价 15.00 元）")
pub fn item_final(item: &CartItem) -> Result<String, String> {
    // BUG 1：price_text 缺失时这里会 panic，应为返回 Err
    let price_text = item.price_text.as_deref().ok_or(format!("跳过：{} 缺价格",item.name))?;
    let price = parse_price(price_text)?;

    // BUG 2：折扣缺失时应按原价（默认 1.0），下面默认值写错了
    // 提示：折扣率是乘法因子，无折扣应为 1.0（不是 0.0；0.0 会让商品变 0 元）
    let discount_text = item.discount_text.as_deref().unwrap_or("1.0");
    let discount = parse_discount(discount_text)?;

    Ok(format!(
        "{} 最终价 {:.2} 元（原价 {:.2} 元）",
        item.name,
        price * discount,
        price
    ))
}

/// 入口函数：结算整批商品，按顺序返回每件的结果文本。
/// 正常输出结算文本，失败输出错误文本。不 panic。
pub fn exercise_fn(items: &[CartItem]) -> Vec<String> {
    items
        .iter()
        .map(|item| match item_final(item) {
            Ok(text) => text,
            Err(msg) => msg,
        })
        .collect()
}