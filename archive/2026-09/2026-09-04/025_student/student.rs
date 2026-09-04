// ============================================
// 题目编号: ex025
// 知识点: 结构体 derive 派生与格式化
// 难度: 基础
// 所属章节: 03_结构体
// ============================================

// 题目要求：
// 下面定义了一个学生结构体 Student，但代码有**几处被挖空**（用 /* ____ */ 标记），
// 请你补全它们，使代码能编译并通过测试。
//
// Student 的功能：
// - 字段 name: String、score: u32
// - 通过 #[derive] 派生 Debug（可 println!("{:?}") 打印）、
//   Clone（可克隆）、PartialEq（可比较相等）
// - 方法 describe(&self) -> String：返回 "姓名-分数" 格式字符串
// - 方法 is_top(&self, threshold: u32) -> bool：分数 >= threshold 返回 true
//
// 请补全以下空白（可能配合推导）：
// 1. 结构体上方的派生属性 `#[derive(/* ____ */)]`
// 2. describe 方法中拼接字符串的代码
// 3. 可能是可见性或缺失的字段/方法
//
// 示例（补全后）：
// Student { name: "Alice", score: 95 }
//   describe() → "Alice-95"
//   is_top(90) → true
//
// 提示：
// - derive 需要 Debug、Clone、PartialEq 三个 trait（用逗号分隔）
// - describe 可用 format!("{}-{}", self.name, self.score)
// - 注意字段名和类型的匹配

use std::fmt::format;

// TODO: 补全下面的派生属性
#[derive(Debug, Clone, PartialEq)]
pub struct Student {
    pub name: String,
    pub score: u32,
}

impl Student {
    pub fn new(name: &str, score: u32) -> Self {
        Self {
            name: name.to_string(),
            score,
        }
    }

    // 返回 "姓名-分数" 格式
    pub fn describe(&self) -> String {
        format!("{}-{}", self.name, self.score).to_string()
    }

    // 分数 >= threshold 返回 true
    pub fn is_top(&self, threshold: u32) -> bool {
        self.score >= threshold
    }
}

pub fn exercise_fn() -> String {
    let s = Student::new("Alice", 95);
    format!("{} top=? {}", s.describe(), s.is_top(90))
}
