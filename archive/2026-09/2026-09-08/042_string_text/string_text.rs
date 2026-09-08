// ============================================
// 题目编号: ex042
// 知识点: String 字符串
// 难度: 基础
// 所属章节: 05_集合类型
// ============================================
//
// 请你从 0 编写几个处理 Rust **String** 字符串的小函数。
// String 是可增长的 UTF-8 文本，这一题重点考察：
//   1. 字符串的拼接（&str 与 String 的配合）
//   2. 字符串遍历（按 char）与统计
//   3. 字符串的可变性（&mut String）
//
// 无需复制/粘贴任何代码，请按下面要求自行定义函数并在 exercise_fn 中调用。

/// 拼接：把 name 和逗号、greeting 组合成一句问候语。
/// 要求返回形如 "你好, Alice" 的 String。
/// 例子：greet("Alice") -> "你好, Alice"
pub fn greet(name: &str) -> String {
    // TODO: 使用 String 拼接，返回 "你好, " 与 name 的组合
    format!("你好, {}", name)
}

/// 统计：给定一段文本，返回其中「非空格字符」的总个数（按 char 计数）。
/// 例子：count_chars("hello world") -> 10  （空格不算）
/// 例子：count_chars("你好 世界")   -> 6   （中文按字符计数，共 4 个字 + 去掉空格）
pub fn count_chars(text: &str) -> usize {
    // TODO: 遍历 text，只统计非空格的字符
    let mut sum = 0;
    for c in text.chars() {
        if c == ' ' {
            continue;
        }
        sum += 1;
    }
    sum
}

/// 修饰：把 word 中的每个字符都转成大写，并追加一个感叹号，返回新字符串。
/// 提示：char 有 to_uppercase() 方法，但要注意它返回的是迭代器，
///       可以通过 .next().unwrap() 取得单个大写字符。
/// 例子：shout("hi") -> "HI!"
pub fn shout(word: &str) -> String {
    // TODO: 遍历字符，逐个大写，末尾加 '!'
    format!("{}!", word.to_uppercase())
}

/// 入口函数：依次调用上面你定义的三个函数，验证它们可用。
/// 返回一个 Vec<String>，依次存放 greet 结果、count_chars 结果转字符串、shout 结果。
/// 例子：exercise_fn("Alice", "hello world", "hi")
///       返回 ["你好, Alice", "10", "HI!"]
pub fn exercise_fn(name: &str, text: &str, word: &str) -> Vec<String> {
    // TODO: 依次调用 greet / count_chars / shout
    // 注意：count_chars 返回 usize，需转成 String 再放进 Vec
    vec![greet(name), count_chars(text).to_string(), shout(word)]
}
