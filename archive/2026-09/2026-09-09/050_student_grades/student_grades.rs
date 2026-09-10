// ============================================
// 题目编号: ex050
// 知识点: 集合综合应用（HashMap + Vec 统计）
// 难度: 基础
// 所属章节: 05_集合类型
// ============================================
//
// 实现一个「学生成绩统计器」：给定一组学生的姓名和成绩，
// 计算每个学生的平均分，并找出最高分和最低分的学生。
// 这是 05 集合章节的最后一道综合题，重点考察：
//   - HashMap 的插入与查询
//   - Vec 的收集与排序
//   - 遍历时的统计逻辑
//
// 请看下面的骨架：3 处逻辑需要你补全。

use std::collections::HashMap;

/// 给定一组 (学生姓名, 成绩) 的数据，返回每个学生的平均分。
/// 输入格式：Vec<(&str, f64)>，每个元素是 (姓名, 成绩)
/// 输出格式：HashMap<String, f64>，键是姓名，值是平均分
///
/// 例子：
///   calculate_average(vec![("Alice", 85.0), ("Bob", 92.0), ("Alice", 90.0)])
///   -> {"Alice": 87.5, "Bob": 92.0}
///
/// 注意：同一个学生可能出现多次，需要计算平均分
pub fn calculate_average(scores: Vec<(&str, f64)>) -> HashMap<String, f64> {
    // 填空 1：遍历 scores，用 HashMap 记录每个学生的总分和次数
    // 然后计算平均分并返回
    let mut list: HashMap<String, (f64, i32)> = HashMap::new();
    for (name, score) in scores {
        list.entry(name.to_string()).or_insert((0.0, 0)).0 += score;
        list.entry(name.to_string()).or_insert((0.0, 0)).1 += 1;
    }
    let mut list2: HashMap<String, f64> = HashMap::new();
    for (name, (score, n)) in list {
        *list2.entry(name.to_string()).or_insert(0.0) = score / n as f64;
    }
    list2
}

/// 入口函数：根据学生平均分数据，找出最高分和最低分的学生
/// 返回 (最高分学生姓名, 最低分学生姓名)
///
/// 例子：
///   exercise_fn(vec![("Alice", 85.0), ("Bob", 92.0), ("Charlie", 78.0)])
///   -> ("Bob", "Charlie")
///
/// 注意：如果有多个学生并列最高/最低，返回任意一个即可
pub fn exercise_fn(scores: Vec<(&str, f64)>) -> (String, String) {
    // 填空 2：调用 calculate_average 得到平均分 HashMap
    // 然后遍历找出最高分和最低分的学生姓名，返回元组
    let mut max: (&str, f64) = ("", -1.0);
    let mut min: (&str, f64) = ("", 9999.0);
    for item in scores {
        if max.1 < item.1 {
            max = item;
        }
        if min.1 > item.1 {
            min = item;
        }
    }
    (max.0.to_string(), min.0.to_string())
}
