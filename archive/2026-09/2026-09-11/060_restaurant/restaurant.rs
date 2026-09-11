// ============================================
// 题目编号: ex060
// 知识点: 错误处理章节收官（餐厅结账系统）
// 难度: 基础
// 所属章节: 06_错误处理
// ============================================
//
// 场景：一个「餐厅结账系统」。菜单里有菜品（名字 + 单价），
// 顾客下订单（每份订单含若干菜品项）。结账时校验每个菜品项，
// 任何一项有问题整单返回错误；全部合法则计算订单总价。
//
// 这是 06 错误处理章节的收官题，综合本章全部能力，并复习第 05 章集合：
//   - 自定义错误类型 OrderError（带数据变体）
//   - 用 Result 聚合多次可能失败的操作（? 或 提前 return）
//   - 边界校验（点菜数量为 0）
//   - 在集合（&[Dish]）中查找（find / position）
//
// 本题采用「定义函数」形式：
//   - Dish、OrderItem、OrderError 及其 Display/Error 已给出
//   - 你需要实现 place_order 和 exercise_fn 两个函数

use std::fmt;

/// 菜单中的一道菜
pub struct Dish {
    pub name: String,
    pub price: f64,
}

/// 订单中的一项（点哪道菜、要几份）
pub struct OrderItem {
    pub name: String,
    pub count: u32,
}

/// 下单时的自定义错误类型
#[derive(Debug, PartialEq)]
pub enum OrderError {
    /// 菜品不在菜单中，携带菜名
    DishNotFound(String),
    /// 数量不能为 0，携带数量
    InvalidCount(u32),
}

impl fmt::Display for OrderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderError::DishNotFound(name) => write!(f, "菜单里没有《{}》", name),
            OrderError::InvalidCount(count) => write!(f, "数量必须大于 0，当前为 {}", count),
        }
    }
}

impl std::error::Error for OrderError {}

/// 为一单订单结账：校验所有菜品项，返回订单总价。
///
/// 按顺序逐项校验，遇到第一个问题即返回 Err（整体失败）：
///   1. 菜品不在菜单中 -> Err(DishNotFound(菜名))
///   2. count == 0     -> Err(InvalidCount(count))
/// 全部合法 -> Ok(总价)，总价 = 各项 price * count 之和。
///
/// 例子：
///   place_order(&[宫保鸡丁:28, 米饭:3], &[宫保鸡丁 x2, 米饭 x3]) ->
///     Ok(28*2 + 3*3 = 65)
///   place_order(&[宫保鸡丁:28], &[红烧肉 x1]) -> Err(DishNotFound("红烧肉"))
///   place_order(&[宫保鸡丁:28], &[宫保鸡丁 x0]) -> Err(InvalidCount(0))
pub fn place_order(menu: &[Dish], items: &[OrderItem]) -> Result<f64, OrderError> {
    // TODO 1：遍历 items，逐项校验并累加总价（用 ? 或提前 return）
    // 提示：用 menu.iter().find(|d| d.name == item.name)
    let mut sum = 0.0;
    for item in items {
        if item.count <= 0 {
            return Err(OrderError::InvalidCount(item.count));
        }
        match menu.iter().find(|d| d.name == item.name) {
            Some(dish) => {
                sum += dish.price * item.count as f64;
            }
            None => return Err(OrderError::DishNotFound(item.name.clone())),
        }
    }
    Ok(sum)
}

/// 入口函数：处理多个订单，返回每个订单的结账文本。
/// 成功 -> "订单合计 {:.2} 元"，失败 -> "订单失败：{错误}"。
///
/// 例子：
///   exercise_fn(&[宫保鸡丁:28, 米饭:3], &[
///       [宫保鸡丁 x2, 米饭 x3],
///       [红烧肉 x1],
///       [宫保鸡丁 x0],
///   ]) ->
///     ["订单合计 65.00 元",
///      "订单失败：菜单里没有《红烧肉》",
///      "订单失败：数量必须大于 0，当前为 0"]
pub fn exercise_fn(menu: &[Dish], orders: &[&[OrderItem]]) -> Vec<String> {
    // TODO 2：遍历 orders，对每份订单调用 place_order 并转成文本
    let mut list: Vec<String> = Vec::new();
    for order in orders {
        match place_order(menu, *order) {
            Ok(num) => list.push(format!("订单合计 {:.2} 元", num)),
            Err(err) => list.push(format!("订单失败：{err}")),
        }
    }
    list
}
