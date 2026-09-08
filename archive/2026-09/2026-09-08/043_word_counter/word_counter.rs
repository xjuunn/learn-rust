// ============================================
// 题目编号: ex043
// 知识点: HashMap 哈希表
// 难度: 基础
// 所属章节: 05_集合类型
// ============================================
//
// 请你从 0 编写一个「词频统计器」，使用 Rust 的 **HashMap** 统计一段文本中
// 每个单词出现的次数。这一题重点考察：
//   1. 如何创建 HashMap
//   2. 向 HashMap 插入 / 更新数据（entry + or_insert 是惯用写法）
//   3. 从 HashMap 按 key 取值
//
// 提示：HashMap 需要从 std::collections 引入。你可以用 std::collections::HashMap。

/// 统计 text 中每个单词的出现次数。
/// 单词：按空格（一个或多个）分隔得到的非空片段。
/// 返回一个 HashMap，key 是单词，value 是该单词出现的次数。
///
/// 例子：
///   word_count("the cat and the dog") ->
///     { "the": 2, "cat": 1, "and": 1, "dog": 1 }
///   word_count("") ->
///     { }（空表）
///
use std::collections::HashMap;
pub fn word_count(text: &str) -> std::collections::HashMap<String, usize> {
    // TODO: 遍历 text.split_whitespace() 得到的每个词，
    //       用 HashMap 统计出现次数
    let mut counts: HashMap<String, usize> = HashMap::new();
    for s in text.split_whitespace() {
        *counts.entry(s.to_string()).or_insert(0) += 1;
    }
    counts
}

/// 入口函数：对给定的 texts 数组，合并其中所有文本的单词统计。
/// 返回一个 HashMap，统计所有文本中每个单词的**总**出现次数。
///
/// 例子：
///   exercise_fn(&["a b", "b c"]) ->
///     { "a": 1, "b": 2, "c": 1 }
pub fn exercise_fn(texts: &[&str]) -> std::collections::HashMap<String, usize> {
    // TODO: 遍历 texts，把每一份文本的词频累计进同一个 HashMap
    let mut counts: HashMap<String, usize> = HashMap::new();
    for text in texts {
        // let m: HashMap<String, usize> = word_count(text);
        for (k,v) in word_count(text) {
            *counts.entry(k).or_insert(0) += v;
        }
    }

    counts
}
