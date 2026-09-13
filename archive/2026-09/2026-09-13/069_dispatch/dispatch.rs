// ============================================
// 题目编号: ex069
// 知识点: 泛型函数综合（从 0 实现函数体）
// 难度: 进阶
// 所属章节: 07_泛型
// ============================================
//
// 前几题多是结构体/方法中的泛型，本题反向训练：**独立泛型函数**。
// 场景：值班调度中心。你需要实现三个通用的工具函数，
// 它们不关心具体类型，只要求类型满足必要的约束。
//
// 任务（函数签名已给出，函数体完全由你实现）：
//
//   1. highest<T: PartialOrd>(items: &[T]) -> Option<&T>
//      返回列表中"最大"的元素的**引用**，空列表返回 None。
//
//   2. count_of<T: PartialEq>(items: &[T], target: &T) -> usize
//      统计 target 在列表中出现的次数（不存在返回 0）。
//
//   3. dedup<T: PartialOrd + Clone>(items: &[T]) -> Vec<T>
//      返回一个新 Vec：元素升序排列，且不含重复值。
//
// exercise_fn 已为你准备好调用逻辑，无需修改：
//   - tasks 为 ["battle", "patrol", "rescue", "battle"]，
//     highest 取字典序最大任务转成 String（空则"无任务"）
//   - count_of 统计 "battle" 出现次数
//   - dedup 对 [3, 1, 2, 3, 1, 2] 去重排序
//   返回 (最高优先级任务, battle 次数, 去重排序后的数值列表)
//
// 提示：
//   - 返回 &T 时无需手写生命周期标注，编译器会按省略规则自动补全
//   - dedup 的 T 同时需要"能比较"(PartialOrd) 和 "能复制"(Clone)
//   - highest 注意单元素/全相等的情况；dedup 注意空列表
//   - 三个函数保持 pub，集成测试才能访问
pub fn exercise_fn() -> (String, usize, Vec<i32>) {
    let tasks = vec!["battle", "patrol", "rescue", "battle"];
    let blank = String::from("无任务");
    (
        highest(&tasks).map(|s| s.to_string()).unwrap_or(blank),
        count_of(&tasks, &"battle"),
        dedup(&[3, 1, 2, 3, 1, 2]),
    )
}

/// 返回列表中最大元素的引用；空列表返回 None。
pub fn highest<T: PartialOrd>(items: &[T]) -> Option<&T> {
    // TODO: 实现遍历取值逻辑（注意用引用比较，元素不可移动）
    if items.is_empty() {
        return None;
    }
    let mut max = &items[0];
    for item in items {
        if max.lt(item) {
            max = item;
        }
    }
    Some(max)
}

/// 统计 target 在列表中出现的次数；不存在返回 0。
pub fn count_of<T: PartialEq>(items: &[T], target: &T) -> usize {
    // TODO: 实现计数逻辑
    let mut num = 0;
    for item in items {
        if item.eq(target) {
            num += 1;
        }
    }
    num
}

/// 返回新 Vec：升序排列且不含重复元素。
pub fn dedup<T: PartialOrd + Clone>(items: &[T]) -> Vec<T> {
    // TODO: 实现去重 + 排序（提示：可先排序再相邻去重，或借助向量查找）
    let mut list: Vec<T> = Vec::from(items);
    list.sort_by(|a, b| a.partial_cmp(b).unwrap());
    list.dedup_by(|a: &mut T, b: &mut T| a.eq(&b));
    list
}
