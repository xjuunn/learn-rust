// ============================================
// 题目编号: ex040
// 知识点: 枚举变体携带数据 + 模式匹配绑定
// 难度: 基础
// 所属章节: 04_枚举与模式匹配
// ============================================

// 题目要求：
// 为 RPG 冒险游戏实现一个"掉落物结算"功能。本题练习让枚举变体携带数据，
// 再用 match 把变体中的数据绑定出来处理。
//
// 请自行定义掉落物枚举（pub，顶部加 derive Debug、PartialEq、Clone）：
//   pub enum Loot {
//       Gold(u32),          // 携带获得的金币数量
//       Item(String),       // 携带获得的物品名称
//       Nothing,            // 空手而归
//   }
//
pub enum Loot {
    Gold(u32),
    Item(String),
    Nothing,
}

// 实现结算函数（pub）：
//   pub fn settle(loot: Loot) -> String
//   - Loot::Gold(n)  => format!("获得 {} 金币", n)
//   - Loot::Item(s)  => format!("获得物品：{}", s)
//   - Loot::Nothing  => "一无所获".to_string()
//
pub fn settle(loot: Loot) -> String {
    match loot {
        Loot::Gold(n) => format!("获得 {} 金币", n),
        Loot::Item(name) => format!("获得物品：{}", name),
        Loot::Nothing => "一无所获".to_string(),
    }
}

// 实现入口函数（pub）：
//   pub fn exercise_fn() -> Vec<String>
//   - 依次结算下面这组掉落物，返回描述列表：
//       Gold(50), Item("铁剑".to_string()), Nothing, Gold(120), Item("回复药水".to_string())
//
// 示例：
//   settle(Loot::Gold(50)) -> "获得 50 金币"
//   settle(Loot::Nothing)  -> "一无所获"
//   exercise_fn() -> ["获得 50 金币", "获得物品：铁剑", "一无所获",
//                     "获得 120 金币", "获得物品：回复药水"]
//
// 提示：
// - 枚举变体携带的数据要用小括号，如 Gold(u32)；匹配时用 Loot::Gold(n) 绑定
// - 枚举与函数都要 pub，否则集成测试访问报 E0603
// - String 字面量转 String 用 .to_string() 或 String::from()
// - 需要为 Loot 加 Clone（测试对比与多次使用），无需 Copy（String 不实现 Copy）

// TODO: 在此处定义 Loot 枚举、settle 函数与 exercise_fn 入口函数

pub fn exercise_fn() -> Vec<String> {
    vec![
        settle(Loot::Gold(50)),
        settle(Loot::Item("铁剑".to_string())),
        settle(Loot::Nothing),
        settle(Loot::Gold(120)),
        settle(Loot::Item("回复药水".to_string())),
    ]
}
