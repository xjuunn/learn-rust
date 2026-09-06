// ============================================
// 题目编号: ex028
// 知识点: 元组结构体
// 难度: 基础
// 所属章节: 03_结构体
// ============================================

// 题目要求：
// 调色师用 RGB 三原色调配新颜色。本题需要你**从 0 定义**一个元组结构体，
// 并为其实现配色与亮度判断。
//
// 步骤：
// 1. 定义元组结构体（三个字段依次为红、绿、蓝分量，取值 0~255）：
//      pub struct Color(pub u8, pub u8, pub u8);
pub struct Color(pub u8, pub u8, pub u8);

// 2. 为 Color 实现以下方法（均需 pub，否则集成测试无法访问）：
//      - pub fn mix(&self, other: &Self) -> Self：
//        对应分量取平均并向下取整，返回混合后的新颜色（不得修改 self/other）
//      - pub fn is_bright(&self) -> bool：
//        三分量平均 > 128 时返回 true，否则返回 false
impl Color {
    pub fn mix(&self, other: &Self) -> Self {
        let r = self.0 as u32 + other.0 as u32;
        let g = self.1 as u32 + other.1 as u32;
        let b = self.2 as u32 + other.2 as u32;
        Self((r / 2) as u8, (g / 2) as u8, (b / 2) as u8)
    }

    pub fn is_bright(&self) -> bool {
        let sum = self.0 as u32 + self.1 as u32 + self.2 as u32;
        if (sum / 3) > 128 { true } else { false }
    }
}

// 3. 实现入口函数（需 pub），供 main 与测试调用：
//      pub fn exercise_fn() -> (u8, u8, u8)
//    - 混合 Color(60, 200, 120) 与 Color(140, 30, 220)
//    - 返回混合结果的三个分量 (r, g, b)
//
// 示例：
//   mix: ((60+140)/2, (200+30)/2, (120+220)/2) = (100, 115, 170)
//   is_bright: (100+115+170)/3 = 128，128 > 128 为 false
//   期望返回: (100, 115, 170)
//
// 提示：
// - 元组结构体定义形如 struct Color(a, b, c); 字段用 .0 / .1 / .2 访问
// - mix 返回新 Color 而不修改原值，思想与 ex027 的克隆一致
// - 整数除法 (a + b) / 2 天然向下取整
// - 解构写法：let Color(r, g, b) = *self;（也可直接 self.0 / self.1 避免解构）
// - 元组结构体的字段须 pub，否则测试读取字段时报 E0603

pub fn exercise_fn() -> (u8, u8, u8) {
    // TODO: 在此调用你定义的 Color 完成配色，返回混合后的 (r, g, b)
    let color1 = Color(60, 200, 120);
    let color2 = Color(140, 30, 220);
    let Color(r, g, b) = color1.mix(&color2);
    (r, g, b)
}
