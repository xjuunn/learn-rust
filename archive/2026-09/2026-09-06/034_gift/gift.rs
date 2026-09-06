// ============================================
// 题目编号: ex034
// 知识点: if let 简写
// 难度: 基础
// 所属章节: 04_枚举与模式匹配
// ============================================

// 题目要求：
// 平安夜的礼物篮中，若只想处理"某一种特定礼物"，match 需要写出所有分支，
// 用 if let 更简洁。本题练习 if let 的简写语法。
//
// 步骤：
// 1. 枚举 Gift 已给出（无需修改）：
//      pub enum Gift { Flower, Apple(u32), Gold }
//    Apple(u32) 里的数字表示苹果个数。
// 2. 实现函数（pub）：
//      pub fn apple_count(gift: &Gift) -> u32
//    - 当 gift 是 Gift::Apple(n) 时返回 n
//    - 其他情况（Flower / Gold）返回 0
// 3. 实现入口函数（pub），供 main 与测试调用：
//      pub fn exercise_fn() -> (u32, u32)
//    - 返回 (apple_count(&Gift::Apple(5)), apple_count(&Gift::Flower))
//
// 示例：
//   apple_count(&Gift::Apple(5)) -> 5
//   apple_count(&Gift::Flower)   -> 0
//   期望返回: (5, 0)
//
// 提示：
// - if let 语法：if let Gift::Apple(n) = gift { n } else { 0 }
// - Apple(u32) 是带数据的变体，n 会绑定到"苹果个数"
// - gift 是引用，if let Gift::Apple(n) = gift 会把 n 绑定成 &u32，
//   返回时用 *n 解引用，或改为 if let Gift::Apple(n) = *gift
// - 函数入口都要 pub，否则集成测试访问报 E0603

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Gift {
    Flower,
    Apple(u32),
    Gold,
}

// 2. 实现函数（pub）：
//      pub fn apple_count(gift: &Gift) -> u32
//    - 当 gift 是 Gift::Apple(n) 时返回 n
//    - 其他情况（Flower / Gold）返回 0
pub fn apple_count(gift: &Gift) -> u32 {
    if let Gift::Apple(n) = gift {
        return *n;
    } else {
        0
    }
}

// 3. 实现入口函数（pub），供 main 与测试调用：
//      pub fn exercise_fn() -> (u32, u32)
//    - 返回 (apple_count(&Gift::Apple(5)), apple_count(&Gift::Flower))
//
pub fn exercise_fn() -> (u32, u32) {
    // TODO: 返回 (apple_count(&Gift::Apple(5)), apple_count(&Gift::Flower))
    (apple_count(&Gift::Apple(5)), apple_count(&Gift::Flower))
}
