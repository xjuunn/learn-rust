// ============================================
// 题目编号: ex031
// 知识点: 枚举定义
// 难度: 基础
// 所属章节: 04_枚举与模式匹配
// ============================================

// 题目要求：
// 四季周而复始。本题需要你**从 0 定义**一个表示季节的枚举，
// 并实现"下一个季节"。
//
// 步骤：
// 1. 定义枚举（顶部加 derive，包含 Debug、PartialEq、Clone、Copy）：
//      #[derive(Debug, PartialEq, Clone, Copy)]
//      pub enum Season { Spring, Summer, Autumn, Winter }
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

// 2. 为 Season 实现方法（pub）：
//      pub fn next(&self) -> Season
//      返回顺时针的下一季；Winter 的下一季回到 Spring（循环）
impl Season {
    pub fn next(&self) -> Season {
        match self {
            Season::Spring => Self::Summer,
            Season::Summer => Self::Autumn,
            Season::Autumn => Self::Winter,
            _ => Self::Spring,
        }
    }
}

// 3. 实现入口函数（pub），供 main 与测试调用：
//      pub fn exercise_fn() -> (Season, Season)
//    - 返回 (Season::Summer.next(), Season::Winter.next())
//
// 示例：
//   Summer.next() -> Autumn
//   Winter.next() -> Spring（绕回第一季）
//   期望返回: (Autumn, Spring)
//
// 提示：
// - 枚举用 enum 定义，变体之间用逗号分隔；枚举不自动带 Copy/Debug，
//   要由 derive 声明，测试断言比较枚举值时依赖 PartialEq
// - next 里用 match self { ... }，每个变体返回它的后一季
// - 枚举与方法都要 pub，否则集成测试访问报 E0603

pub fn exercise_fn() -> (Season, Season) {
    // TODO: 返回 (Season::Summer.next(), Season::Winter.next())
    (Season::Summer.next(), Season::Winter.next())
}
