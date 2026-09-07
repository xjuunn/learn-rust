// ============================================
// 题目编号: ex039
// 知识点: Option 综合（越界与范围匹配）
// 难度: 基础
// 所属章节: 04_枚举与模式匹配
// ============================================

// 题目要求：
// 教务系统需要把分数转换为评级。本题练习用 Option 表达"非法输入"，
// 以及用 match 的数值范围模式处理边界。
//
// 步骤：
// 1. 定义成绩枚举（pub，顶部加 derive Debug、PartialEq、Clone、Copy）：
//      pub enum Grade { A, B, C, D, F }

#[derive(Debug, PartialEq)]
pub enum Grade {
    A,
    B,
    C,
    D,
    F,
}

// 2. 实现评分函数（pub）：
//      pub fn grade(score: u32) -> Option<Grade>
//    - 90..=100 => Some(A)    75..=89 => Some(B)
//    - 60..=74  => Some(C)    50..=59 => Some(D)
//    - 0..=49   => Some(F)
//    - 其它（score > 100）=> None（分数不可能高于 100）
pub fn grade(score: u32) -> Option<Grade> {
    match score {
        90..=100 => Some(Grade::A),
        75..=89 => Some(Grade::B),
        60..=74 => Some(Grade::C),
        50..=59 => Some(Grade::D),
        0..=49 => Some(Grade::F),
        _ => None,
    }
}

// 3. 实现入口函数（pub），供 main 与测试调用：
//      pub fn exercise_fn() -> Vec<Option<Grade>>
//    - 对成绩 [95, 80, 62, 55, 30, 120] 逐项调 grade，返回结果列表
//
// 示例：
//   grade(95) -> Some(Grade::A)
//   grade(120) -> None
//   exercise_fn() -> [Some(A), Some(B), Some(C), Some(D), Some(F), None]
//
// 提示：
// - 范围模式写法：match score { 90..=100 => Some(Grade::A), ... }
// - 注意边界值：90 属于 A，89 属于 B；75 属于 B，74 属于 C（值域含端点）
// - score > 100 的情况用通配 _ => None 兜底
// - 枚举与函数都要 pub，否则集成测试访问报 E0603

pub fn exercise_fn() -> Vec<Option<Grade>> {
    // TODO: 对成绩 [95, 80, 62, 55, 30, 120] 逐项调用 grade 并返回结果
    vec![
        grade(95),
        grade(80),
        grade(62),
        grade(55),
        grade(30),
        grade(120),
    ]
}
