// ============================================
// 题目编号: ex035
// 知识点: 模式匹配绑定
// 难度: 基础
// 所属章节: 04_枚举与模式匹配
// ============================================

// 题目要求：
// 天气预报员要按天气类型播报。带数据的枚举在 match 时可以把数据"绑定"
// 到变量再使用。本题练习模式匹配绑定。
//
// 步骤：
// 1. 枚举 Weather 已给出（无需修改）：
//      pub enum Weather { Sunny, Windy(u32), Rainy(u32) }
//    Windy 里的 u32 是风力等级，Rainy 里的 u32 是降雨量（毫米）。
// 2. 实现函数（pub）：
//      pub fn report(weather: Weather) -> String
//    - Sunny    -> "晴天"
//    - Windy(level) -> format!("刮风，风力 {} 级", level)
//    - Rainy(mm)    -> format!("下雨，降雨 {} 毫米", mm)
// 3. 实现入口函数（pub），供 main 与测试调用：
//      pub fn exercise_fn() -> Vec<String>
//    - 依次对四种天气调用 report，返回播报列表
//      顺序：[Sunny, Windy(6), Rainy(30)]
//
// 示例：
//   report(Weather::Windy(6)) -> "刮风，风力 6 级"
//   report(Weather::Rainy(30)) -> "下雨，降雨 30 毫米"
//   期望返回: vec!["晴天", "刮风，风力 6 级", "下雨，降雨 30 毫米"]
//
// 提示：
// - match 里绑定数据：match weather { Weather::Windy(level) => { 用 level }, ... }
// - format! 拼接字符串，level/mm 会作为展示值
// - 入口返回 Vec<String>，直接 vec![report(Weather::Sunny), ...]
// - 枚举与函数都要 pub，否则集成测试访问报 E0603

use crate::exercises::weather::Weather::Rainy;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Weather {
    Sunny,
    Windy(u32),
    Rainy(u32),
}

// 2. 实现函数（pub）：
//      pub fn report(weather: Weather) -> String
//    - Sunny    -> "晴天"
//    - Windy(level) -> format!("刮风，风力 {} 级", level)
//    - Rainy(mm)    -> format!("下雨，降雨 {} 毫米", mm)
pub fn report(weather: Weather) -> String {
    match weather {
        Weather::Sunny => "晴天".to_string(),
        Weather::Rainy(v) => format!("下雨，降雨 {} 毫米", v),
        Weather::Windy(v) => format!("刮风，风力 {} 级", v),
    }
}
// 3. 实现入口函数（pub），供 main 与测试调用：
//      pub fn exercise_fn() -> Vec<String>
//    - 依次对四种天气调用 report，返回播报列表
//      顺序：[Sunny, Windy(6), Rainy(30)]
//
pub fn exercise_fn() -> Vec<String> {
    // TODO: 依次 report 三种天气并返回播报列表

    vec![
        report(Weather::Sunny),
        report(Weather::Windy(6)),
        report(Rainy(30)),
    ]
}
