// ============================================
// 题目编号: ex029
// 知识点: 结构体中的生命周期
// 难度: 基础
// 所属章节: 03_结构体
// ============================================

// 题目要求：
// 收藏家为珍藏的铭牌制作登记卡。登记卡**借用**外部的字符串（不做复制），
// 因此结构体会持有引用，需要标注生命周期。本题需要你**补全生命周期标注**
// 并实现相关方法。
//
// 步骤：
// 1. 补全下面结构体的两处 ____，使结构体能持有字符串引用：
//      第一处（尖括号内）写生命周期参数，如 'a；
//      第二处（& 后面）写同一个生命周期标注。
//   补全后应为：pub struct Label<'a> { pub title: &'a str, pub year: u16 }
// 2. 为 Label 实现关联函数与方法（均需 pub）：
//      - pub fn new(title: &'a str, year: u16) -> Self
//      - pub fn describe(&self) -> String：
//        返回 "《标题》 - 年份" 格式，例如 "《静谧湖面》 - 1892"
// 3. 实现入口函数（需 pub），供 main 与测试调用：
//      pub fn exercise_fn() -> (String, u16)
//    - 创建 Label::new("静谧湖面", 1892)
//    - 返回 (describe() 的结果, year)
//
// 示例：
//   Label::new("静谧湖面", 1892)
//   describe() -> "《静谧湖面》 - 1892"
//   期望返回: ("《静谧湖面》 - 1892", 1892)
//
// 提示：
// - 结构体含引用字段时，必须在尖括号里声明生命周期参数 <'a>，字段写作 &'a str
// - new 的入参 title 与返回类型 Self 应使用同一个 'a，保证借用来源一致
// - 生命周期标注只描述"借用持续多久"，不改变运行时的行为
// - 实现方法的 impl 块也要携带同一个生命周期参数：
//     impl<'a> Label<'a> { ... }
// - describe 使用 format! 拼接字符串
// - 字段与方法都要 pub，否则集成测试访问时报 E0603

pub struct Label<'a> {
    pub title: &'a str,
    pub year: u16,
}

// 2. 为 Label 实现关联函数与方法（均需 pub）：
//      - pub fn new(title: &'a str, year: u16) -> Self
//      - pub fn describe(&self) -> String：
//        返回 "《标题》 - 年份" 格式，例如 "《静谧湖面》 - 1892"
impl<'a> Label<'a> {
    pub fn new(title: &'a str, year: u16) -> Self {
        Self { title, year }
    }

    pub fn describe(&self) -> String {
        format!("《{}》 - {}", self.title, self.year)
    }
}

pub fn exercise_fn() -> (String, u16) {
    // TODO: 创建 Label::new("静谧湖面", 1892)，返回 (describe 结果, year)
    let l = Label::new("静谧湖面", 1892);
    (l.describe(), l.year)
}
