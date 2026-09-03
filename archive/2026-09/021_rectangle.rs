// ============================================
// 题目编号: ex021
// 知识点: 结构体定义与实例化
// 难度: 基础
// 所属章节: 03_结构体
// ============================================

// 题目要求：
// 请你定义两个结构体和一个函数，并在入口函数 exercise_fn 中调用。
//
// 1. 定义结构体 `Rectangle`：
//    - 字段 width: u32
//    - 字段 height: u32
//    - 声明为 pub，字段也声明为 pub（供测试访问）
//
// 2. 为 Rectangle 实现方法：
//    - `fn area(&self) -> u32`：返回面积（width * height）
//    - `fn perimeter(&self) -> u32`：返回周长（(width + height) * 2）
//
// 3. 实现入口函数 exercise_fn：
//    - 接收 (width: u32, height: u32)
//    - 创建一个 Rectangle 实例
//    - 返回一个元组 (面积, 周长)
//
// 示例：
// 输入：(3, 4) → 输出：(12, 14)【面积 3*4=12，周长 (3+4)*2=14】
// 输入：(5, 5) → 输出：(25, 20)
//
// 提示：
// - 结构体：`pub struct Rectangle { pub width: u32, pub height: u32 }`
// - 方法写在 `impl Rectangle` 中，方法需 pub
// - 实例化：`Rectangle { width, height }`
// - 入口返回值类型为 `(u32, u32)`

pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    // TODO: 实现 area 和 perimeter 方法

    pub fn new(width:u32, height: u32) -> Self{
        Self {
            width,
            height
        }
    }

    pub fn area(&self) -> u32 {
        self.height * self.width
    }

    pub fn perimeter(&self) -> u32 {
        (self.height + self.width) * 2
    }
}

pub fn exercise_fn(width: u32, height: u32) -> (u32, u32) {
    // TODO: 创建 Rectangle 并返回 (面积, 周长)
    let rect = Rectangle::new(width, height);
    (rect.area(),rect.perimeter())
}
