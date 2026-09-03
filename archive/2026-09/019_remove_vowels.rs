// ============================================
// 题目编号: ex019
// 知识点: 所有权综合（实现/扩展：字符串处理）
// 难度: 基础
// 所属章节: 02_所有权
// ============================================

// 题目要求：
// 下面已经提供了一个辅助函数 `is_vowel`，用于判断一个字符是否为英文元音字母
// （不分大小写：a e i o u 及其大写形式）。
//
// 请你实现入口函数 `exercise_fn`：
// 接收一个 String 的所有权，返回一个新的 String，其中**去除所有元音字母**，
// 保留其他字符（包括辅音、数字、空格、标点）。
//
// 注意：
// - 函数接收 String（有所有权），但返回的是新 String，原 String 会被消耗
// - 遍历字符时用 `c.chars()`，然后用 `collect()` 重新收集成 String
// - 参考辅助函数 is_vowel（可直接在 exercise_fn 中调用它来判断每个字符）
//
// 示例：
// 输入："hello" → 输出："hll"
// 输入："Rust语言" → 输出："Rst语言"（R,s,t 保留；u 是元音被去除；中文保留）
// 输入："AEIOU" → 输出：""（全元音）
//
// 提示：
// - 用 `s.chars().filter(...).collect()` 一行实现最简洁
// - 或者用循环 + 累加字符串
// - is_vowel 的签名是 `fn is_vowel(c: char) -> bool`

fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}

pub fn exercise_fn(s: String) -> String {
    // TODO: 实现去除元音的逻辑
    let mut chars:Vec<char> = Vec::new();
    for c in s.chars() {
        if !is_vowel(c) {
            chars.push(c);
        }
    }
    chars.iter().collect()
}
