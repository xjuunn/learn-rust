// ============================================
// 题目编号: ex036
// 知识点: 枚举综合应用（修复 bug）
// 难度: 基础
// 所属章节: 04_枚举与模式匹配
// ============================================

// 题目要求：
// 交通信号灯循环切换。下面这段代码有一个**逻辑错误**（编译能通过、跑起来却错），
// 请找出 bug 并修复；另外 duration 尚未实现，请一并完成。
//
// 步骤：
// 1. 修复 next_light 中的逻辑错误：
//    - 正确顺序是 Red -> Yellow -> Green -> Red（循环）
//    - 当前某条分支返回了错误的颜色
// 2. 实现 duration（pub）：
//      pub fn duration(light: Light) -> u32
//    - Red = 30，Yellow = 5，Green = 35
// 3. 实现入口（pub），供 main 与测试调用：
//      pub fn exercise_fn() -> (Light, u32)
//    - 返回 (next_light(Light::Red), duration(Light::Yellow))
//
// 示例：
//   next_light(Light::Red) -> Yellow
//   duration(Light::Yellow) -> 5
//   期望返回: (Yellow, 5)
//
// 提示：
// - 红灯后应短暂黄灯再转绿灯，找找哪个分支与这个顺序矛盾
// - duration 用 match 映射即可，与 ex033 硬币题同思路
// - 枚举与函数都要 pub，否则集成测试访问报 E0603

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Light {
    Red,
    Yellow,
    Green,
}

// 下面的切换逻辑有一处 bug，请修复
pub fn next_light(light: Light) -> Light {
    match light {
        Light::Red => Light::Yellow,
        Light::Yellow => Light::Green,
        Light::Green => Light::Red,
    }
}

// 2. 实现 duration（pub）：
//      pub fn duration(light: Light) -> u32
//    - Red = 30，Yellow = 5，Green = 35
pub fn duration(light: Light) -> u32 {
    match light {
        Light::Green => 35,
        Light::Red => 30,
        Light::Yellow => 5,
    }
}

// 3. 实现入口（pub），供 main 与测试调用：
//      pub fn exercise_fn() -> (Light, u32)
//    - 返回 (next_light(Light::Red), duration(Light::Yellow))
//
pub fn exercise_fn() -> (Light, u32) {
    // TODO: 返回 (next_light(Light::Red), duration(Light::Yellow))
    (next_light(Light::Red), duration(Light::Yellow))
}
