// ============================================
// 题目编号: ex045
// 知识点: 集合与所有权（借用遍历、克隆、移动）
// 难度: 基础
// 所属章节: 05_集合类型
// ============================================

// 题目要求：
// 维护一份邮件订阅者昵称列表，练习对集合的所有权处理。请实现四个 pub 函数：
//
// 1. count_len(list: &[String]) -> usize
//    只读借用遍历整个列表，返回所有昵称的字符总数。
//    请用引用（&list 或 list.iter()），不得转移所有权。
//
pub fn count_len(list: &[String]) -> usize {
    let mut count = 0;
    for item in list.iter() {
        count += item.len();
    }
    count
}

// 2. to_upper(list: &[String]) -> Vec<String>
//    只读借用遍历，把所有昵称转为大写，返回一份全新的大写列表。
//    原列表不得被修改或移动。可用 .to_uppercase() 得到大写形式。
//
pub fn to_upper(list: &[String]) -> Vec<String> {
    let mut l: Vec<String> = Vec::new();
    for item in list {
        l.push(item.to_uppercase());
    }
    l
}

// 3. take_ownership(list: Vec<String>) -> usize
//    直接按值接收整个列表（所有权转移），返回列表里昵称的个数。
//
pub fn take_ownership(list: Vec<String>) -> usize {
    list.len()
}

// 4. exercise_fn() -> (usize, usize, Vec<String>)
//    对列表 ["alice", "bob", "carol"] 依次：
//    - 先对 count_len 求总字符数（借用，不移动）
//    - 再对 to_upper 求大写列表（借用，不移动）
//    - 最后把原列表按值传给 take_ownership（所有权发生转移）
//    返回 (总字符数, 昵称个数, 大写列表)
//
// 请完成以上四个函数。函数签名与返回值已由题目测试约定，勿改动。

pub fn exercise_fn() -> (usize, usize, Vec<String>) {
    let list: Vec<String> = vec!["alice".to_string(), "bob".to_string(), "carol".to_string()];
    (
        count_len(&list),
        take_ownership(list.clone()),
        to_upper(&list),
    )
}

// ============================================
// 示例与提示（仅供参考，先独立思考，卡住再看）
// ============================================

// 示例：
//   count_len(&["a".into(), "bb".into()]) -> 3  （"a" 1 字 + "bb" 2 字）
//   to_upper(&["ab".into()]) -> vec!["AB".to_string()]
//   take_ownership(vec!["x".into()]) -> 1
//   exercise_fn() -> (14, 3, ["ALICE", "BOB", "CAROL"])

// 提示：
// - 只读遍历三选一：for e in list、for e in list.iter()、list.iter().map/for_each
//   它们都不会移动元素，传 &[String] 的引用即可
// - 普通 for e in list 拿到的是 &String，可直接用 e.len()、e.to_uppercase()
// - to_upper 收集结果：let mut out = Vec::new(); out.push(...);  或
//   list.iter().map(|s| s.to_uppercase()).collect()
// - take_ownership 直接接收 Vec<String>，函数体内用 list.len() 即可
// - exercise_fn 注意顺序：先借用（count_len、to_upper），最后才把列表传给
//   take_ownership（一旦按值传入，原变量就失效了，因此必须放最后）
// - 四个函数都要 pub，否则集成测试访问报 E0603
