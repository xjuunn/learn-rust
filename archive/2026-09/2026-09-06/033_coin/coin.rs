// ============================================
// 题目编号: ex033
// 知识点: match 表达式
// 难度: 基础
// 所属章节: 04_枚举与模式匹配
// ============================================

// 题目要求：
// 存钱罐里的硬币各有面值。本题聚焦 match 表达式：把每种硬币映射到分值。
//
// 步骤：
// 1. 枚举 Coin 已在下文给出（无需修改）：
//      pub enum Coin { Penny, Nickel, Dime, Quarter }
// 2. 实现函数（pub），用 match 把每种硬币映射为分值：
//      pub fn value_in_cents(coin: Coin) -> u32
//    - Penny = 1，Nickel = 5，Dime = 10，Quarter = 25
// 3. 实现入口函数（pub），供 main 与测试调用：
//      pub fn exercise_fn() -> Vec<u32>
//    - 依次对四种硬币调用 value_in_cents
//    - 返回分值列表，顺序为 [Penny, Nickel, Dime, Quarter] 各自的值
//
// 示例：
//   value_in_cents(Coin::Dime)  -> 10
//   value_in_cents(Coin::Quarter) -> 25
//   期望返回: vec![1, 5, 10, 25]
//
// 提示：
// - match 语法：match coin { Coin::Penny => 1, Coin::Nickel => 5, ... }
// - 每个分支返回一个值，四个变体都要覆盖（穷尽性检查），遗漏会编译报错
// - Coin 已 derive，测试可直接比较；函数要 pub 否则集成测试报 E0603
// - Vec 写法：vec![value_in_cents(Coin::Penny), ...]

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// 2. 实现函数（pub），用 match 把每种硬币映射为分值：
//      pub fn value_in_cents(coin: Coin) -> u32
//    - Penny = 1，Nickel = 5，Dime = 10，Quarter = 25
pub fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

// 3. 实现入口函数（pub），供 main 与测试调用：
//      pub fn exercise_fn() -> Vec<u32>
//    - 依次对四种硬币调用 value_in_cents
//    - 返回分值列表，顺序为 [Penny, Nickel, Dime, Quarter] 各自的值
//
pub fn exercise_fn() -> Vec<u32> {
    // TODO: 依次对四种硬币取分值，返回 vec![1, 5, 10, 25]
    let penny = value_in_cents(Coin::Penny);
    let nickel = value_in_cents(Coin::Nickel);
    let dime = value_in_cents(Coin::Dime);
    let quarter = value_in_cents(Coin::Quarter);
    vec![penny, nickel, dime, quarter]
}
