// ============================================
// 题目编号: ex047
// 知识点: 集合综合应用
// 难度: 基础
// 所属章节: 05_集合类型
// ============================================
//
// 模拟一个「书架归档系统」：一批书的信息以 (书名, 分类) 的元组集合给出，
// 请你用 Rust 集合把书按**分类**归档。
//
// 这一题综合考察 05 章节的三大集合：
//   - String：书名的存储
//   - Vec：每个分类下的书单
//   - HashMap：按分类分组的归档表
//
// 例子：
//   archive(&[
//       ("三体", "科幻"),
//       ("活着", "文学"),
//       ("沙丘", "科幻"),
//   ]) ->
//     {
//         "科幻": ["三体", "沙丘"],
//         "文学": ["活着"],
//     }
//
// 请完成 archive 函数，返回按分类分组的 HashMap：
//   key 是 &str 分类名，value 是 Vec<String>（该分类下的所有书名，按原顺序）。

use std::collections::HashMap;

/// 将 (分类, 书名) 的集合按分类归档。
/// 输入：books，元素为 (分类名 &str, 书名 &str)。
/// 返回：HashMap<&str, Vec<String>>，key 为分类名，value 为该分类的书名列表。
///
/// 注意：返回值里的 key 直接借用输入的 &str（不 clone），
///       而 value 里的书名需要转成拥有的 String 存入。
pub fn archive<'a>(books: &[(&'a str, &'a str)]) -> HashMap<&'a str, Vec<String>> {
    // TODO: 遍历 books，用 entry() 按分类取出书单 Vec，
    // 再用 push 把书名（转成 String）追加进去
    let mut list: HashMap<&str, Vec<String>> = HashMap::new();
    for book in books {
        list.entry(book.1)
            .or_insert(vec![])
            .push(book.0.to_string());
    }
    list
}

/// 入口函数：返回每个分类下的书本数量（HashMap<&str, usize>）。
/// 可以直接调用 archive 之后 map 每个分类的 Vec 长度。
pub fn exercise_fn<'a>(books: &[(&'a str, &'a str)]) -> HashMap<&'a str, usize> {
    // TODO: 调用 archive 得到分组表，再统计每个分类的书本数量
    let arch = archive(books);
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for item in arch {
        *counts.entry(item.0).or_insert(0) = item.1.len();
    }
    counts
}
