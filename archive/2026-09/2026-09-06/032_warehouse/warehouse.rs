// ============================================
// 题目编号: ex032
// 知识点: Option 枚举
// 难度: 基础
// 所属章节: 04_枚举与模式匹配
// ============================================

// 题目要求：
// 仓库登记员按编号查找货架号：找到返回 Some(货架号)，找不到返回 None。
// 货品表用切片表示，元素是 (编号, 货架号) 元组。
//
// 步骤：
// 1. 实现查找函数（pub）：
//      pub fn locate(products: &[(u32, &str)], id: u32) -> Option<String>
//    - 遍历 products，找到编号等于 id 的条目时返回 Some(货架号字符串)
//    - 遍历结束仍未找到时返回 None
pub fn locate(products: &[(u32, &str)], id: u32) -> Option<String> {
    for product in products {
        if product.0 == id {
            return Some(product.1.to_string());
        }
    }
    None
}

// 2. 实现入口函数（pub），供 main 与测试调用：
//      pub fn exercise_fn() -> (Option<String>, Option<String>)
//    - 货架表为 [(101, "A3"), (102, "B1")]
//    - 返回 (locate(货架表, 101), locate(货架表, 999))
//
// 示例：
//   locate([(101,"A3"),(102,"B1")], 101) -> Some("A3")
//   locate([(101,"A3"),(102,"B1")], 999) -> None
//   期望返回: (Some("A3"), None)
//   （函数实际返回 Option<String>，即 Some("A3".to_string())）
//
// 提示：
// - Option 是标准库预置枚举：Some(值)"表示存在"，None"表示缺失"
// - 遍历可用 for (code, shelf) in products，元组解构同时拿到编号与货架号
// - 命中时返回 Some(shelf.to_string())，把 &str 转成 String；循环后 return None
// - 数组借给函数请写 &[(101,"A3"),(102,"B1")]
// - 两个函数都要 pub，否则集成测试访问报 E0603

pub fn exercise_fn() -> (Option<String>, Option<String>) {
    // TODO: 构建货架表并调用 locate，返回 (Some("A3"), None)
    let b = [(101, "A3"), (102, "B1")];
    (locate(&b, 101), locate(&b, 999))
}
