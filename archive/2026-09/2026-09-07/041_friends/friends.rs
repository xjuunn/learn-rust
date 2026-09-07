// ============================================
// 题目编号: ex041
// 知识点: Vec 动态数组（创建、添加、遍历）
// 难度: 基础
// 所属章节: 05_集合类型
// ============================================

// 题目要求：
// 管理一份好友名单。请实现三个 pub 函数：
//
// 1. make_list() -> Vec<String>
//    创建并返回一份初始好友名单，包含三位好友：小红、小明、小刚
//
pub fn make_list() -> Vec<String> {
    vec!["小红".to_string(), "小明".to_string(), "小刚".to_string()]
}

// 2. add_friend(list: &mut Vec<String>, name: &str) -> bool
//    将名字追加到名单末尾。只有名字在名单中不存在、且名字不是"空"字时
//    才追加成功并返回 true；否则不添加并返回 false
//
pub fn add_friend(list: &mut Vec<String>, name: &str) -> bool {
    if name == "空" {
        return false;
    }

    let mut has_name = false;
    for item in list.clone() {
        if item == name {
            has_name = true;
        }
    }
    if !has_name {
        list.push(name.to_string());
        return true;
    } else {
        return false;
    }
}

// 3. exercise_fn() -> usize
//    用 make_list() 得到名单，然后依次尝试添加：小美、小明、空、大壮
//    返回最终名单的长度（长度应反映哪些添加成功）
//
// 请完成以上三个函数。函数签名与返回值已由题目测试约定，勿改动。

pub fn exercise_fn() -> usize {
    let mut list = make_list();
    add_friend(&mut list, "小美");
    add_friend(&mut list, "小明");
    add_friend(&mut list, "空");
    add_friend(&mut list, "大壮");
    list.len()
}

// ============================================
// 示例与提示（仅供参考，先独立思考，卡住再看）
// ============================================

// 示例：
//   make_list() -> ["小红", "小明", "小刚"]
//   依次添加后名单 -> ["小红", "小明", "小刚", "小美", "大壮"]（长度 5）
//   exercise_fn() -> 5

// 提示：
// - Vec<String> 的 push 接收 String，&str 转 String 用 .to_string()
// - 判断是否已存在：for existing in list.iter() { if existing == name { ... } }
// - add_friend 接收 &mut Vec，遍历时不要移动元素
// - 三个函数都要 pub，否则集成测试访问报 E0603
// - 初始名单可用 Vec::new() + 逐个 push，或直接用 vec![] 宏，两种都练一下
