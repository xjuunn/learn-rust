// ============================================
// 题目编号: ex022
// 知识点: 元组结构体与关联函数
// 难度: 基础
// 所属章节: 03_结构体
// ============================================

// 题目要求：
// 请你定义元组结构体 Point，并实现关联函数（constructor），
// 然后在入口函数 exercise_fn 中调用。
//
// 1. 定义元组结构体 `Point`：
//    - 只有一个字段：`(f64, f64)`，表示 (x, y) 坐标
//    - 声明为 pub（测试需访问）
//    - 例：`Point(3.0, 4.0)` 表示 x=3.0, y=4.0
//
// 2. 为 Point 实现关联函数：
//    - `fn new(x: f64, y: f64) -> Self`：构造一个 Point
//    - `fn distance(&self, other: &Point) -> f64`：计算两点间的欧氏距离
//      （公式：sqrt((x1-x2)² + (y1-y2)²)）
//    - 用 `f64::sqrt` 计算平方根
//
// 3. 实现入口函数 exercise_fn：
//    - 接收六个参数 p1_x, p1_y, p2_x, p2_y（即两点的坐标）
//    - 用 Point::new 创建两个点
//    - 返回 p1.distance(&p2)
//
// 示例：
// 输入：(0.0, 0.0, 3.0, 4.0) → 输出：5.0（勾股数 3-4-5）
// 输入：(1.0, 1.0, 1.0, 1.0) → 输出：0.0（同一点）
//
// 提示：
// - 元组结构体：`pub struct Point(pub f64, pub f64);`
// - 关联函数用 `Self` 或 `Point` 作为返回类型
// - 距离公式用 `let dx = self.0 - other.0;`（.0 访问元组第一项）
// - `f64::sqrt(值)` 计算平方根
// - 方法需 pub，entry 函数也需 pub

pub struct Point(pub f64, pub f64);

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self(x, y)
    }

    pub fn distance(&self, other: &Point) -> f64 {
        f64::sqrt((self.0 - other.0).powi(2) + (self.1 - other.1).powi(2))
    }
}

pub fn exercise_fn(p1_x: f64, p1_y: f64, p2_x: f64, p2_y: f64) -> f64 {
    // TODO: 创建两个 Point，返回 p1.distance(&p2)
    let p1 = Point::new(p1_x, p1_y);
    let p2 = Point::new(p2_x, p2_y);
    p1.distance(&p2)
}
