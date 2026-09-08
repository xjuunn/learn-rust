// ============================================
// 题目编号: ex044
// 知识点: 集合遍历
// 难度: 基础
// 所属章节: 05_集合类型
// ============================================
//
// 请你从 0 编写一个「库存统计器」：给定一件件入库记录，统计每种商品的总数量。
// 这一题重点考察集合的综合应用：
//   1. 结构体 Vec 存放记录
//   2. 用 HashMap 对同名商品累计数量（复习 entry + or_insert）
//   3. 集合遍历与数据整理
//
// 你需要先定义一个结构体能描述「一次入库」：
//   struct StockItem {
//       name: String,      // 商品名
//       qty: usize,        // 本次入库数量
//   }
//
// 然后实现下面的函数。

use std::collections::HashMap;

/// 定义一次入库记录（请保持字段名与上面一致，并声明为 pub）
pub struct StockItem {
    pub name: String,
    pub qty: usize,
}

/// 汇总：把整个仓库的入库记录合并成「每种商品的总数量」。
/// 返回 HashMap<String, usize>，key 为商品名，value 为该商品**累计**总数量。
///
/// 例子：
///   inventory(&[
///       StockItem { name: "苹果".to_string(), qty: 3 },
///       StockItem { name: "香蕉".to_string(), qty: 2 },
///       StockItem { name: "苹果".to_string(), qty: 5 },
///   ]) ->
///     { "苹果": 8, "香蕉": 2 }
pub fn inventory(items: &[StockItem]) -> HashMap<String, usize> {
    // TODO: 遍历 items，对每个 item 用 entry(item.name ...) 累计数量
    todo!()
}

/// 入口函数：给定多批记录，返回每种商品的总数量。
/// 这里直接透传给你 `inventory`，你也可以在里面做额外处理（比如去空名）。
/// 返回的规则与 inventory 相同。
pub fn exercise_fn(items: &[StockItem]) -> HashMap<String, usize> {
    // TODO: 调用你实现的 inventory，把结果返回
    todo!()
}