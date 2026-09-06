// ============================================
// 题目编号: ex038
// 知识点: while let 与 Vec 边界
// 难度: 基础
// 所属章节: 04_枚举与模式匹配
// ============================================

// 题目要求：
// 快递驿站有一排货架（Vec<String>），包裹从货架**顶端**依次取出（后进先出，
// 类似一摞书）。取出时用 while let 循环，直到货架取空自动停止。
//
// 步骤：
// 1. 实现卸载函数（pub）：
//      pub fn unload(shelf: &mut Vec<String>) -> Vec<String>
//    - 每轮用 while let Some(item) = shelf.pop() 取出栈顶包裹
//    - 把 item 依次收集进结果 Vec 并返回
//    - 货架空后循环应自动结束
pub fn unload(shelf: &mut Vec<String>) -> Vec<String> {
    let mut list = Vec::new();
    while let Some(item) = shelf.pop() {
        list.push(item);
    }
    list
}

// 2. 实现入口函数（pub），供 main 与测试调用：
//      pub fn exercise_fn() -> Vec<String>
//    - 货架 ["A1", "B2", "C3"]，卸载后返回 ["C3", "B2", "A1"]
//
// 示例：
//   shelf = ["A1", "B2", "C3"]
//   unload 后返回 ["C3", "B2", "A1"]（取走顺序）
//
// 提示：
// - while let Some(item) = shelf.pop() { ... } 是核心写法：
//   pop 返回 Option<String>，Some 就处理，None（空货架）就退出循环
// - 结果用 Vec::push 收集，或 vec![] + push
// - 入参 shelf 是 &mut，这样才能 pop 修改它，用后可继续留空
// - 函数要 pub，否则集成测试访问报 E0603

pub fn exercise_fn() -> Vec<String> {
    // TODO: 货架 ["A1", "B2", "C3"]，调用 unload 并返回结果
    let mut shelf = vec!["A1".to_string(), "B2".to_string(), "C3".to_string()];
    unload(&mut shelf)
}
