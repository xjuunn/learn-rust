// ============================================
// 题目编号: ex046
// 知识点: 集合遍历
// 难度: 基础
// 所属章节: 05_集合类型
// ============================================
//
// 下面的 top_words 函数本意是：统计一段文本中每个单词出现的次数，
// 然后返回**出现次数最多的前 k 个单词**（按次数从高到低）。
//
// 但它有一个 bug，导致结果顺序不对、返回的不是高频词。
// 请你：
//   1. 找出 bug 并修复（让它优先返回出现次数多的单词）
//   2. 补全缺失的排序条件（有一处被 /* ____ */ 挡住了）
//
// 考察点：HashMap 遍历、Vec 排序（sort_by）、迭代器 take 截断。
//
// 例子：
//   top_words("a b c a b a", 2) -> ["a", "b"]   （a 出现 3 次，b 出现 2 次）

use std::collections::HashMap;

/// 返回文本中出现次数最多的前 k 个单词（高频在前）。
pub fn top_words(text: &str, k: usize) -> Vec<String> {
    // 第一步：统计每个单词出现次数（这段代码是正确的，你无需修改）
    let mut counts: HashMap<String, usize> = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word.to_string()).or_insert(0) += 1;
    }

    // 第二步：把词频表转成 Vec，每一项是 (&单词, &次数)
    let mut pairs: Vec<(&String, &usize)> = counts.iter().collect();

    // 第三步：排序。BUG 在这里——当前按次数「升序」排列，
    //        但题目要求返回次数「最多」的词，应改为降序。
    //        请把这里的升序改为降序（次数高的排前面）。
    pairs.sort_by(|a, b| b.1.cmp(a.1));

    // 第四步：取前 k 个单词返回（无需修改）
    pairs.into_iter().take(k).map(|(w, _)| w.clone()).collect()
}

/// 入口函数：对一段文本输出高频词排行榜，返回前 k 个词。
pub fn exercise_fn(text: &str, k: usize) -> Vec<String> {
    top_words(text, k)
}