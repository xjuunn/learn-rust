// ============================================
// 题目编号: ex024
// 知识点: 结构体方法综合（从 0 定义）
// 难度: 基础
// 所属章节: 03_结构体
// ============================================

// 题目要求：
// 请你从 0 定义一本书结构体 `Book`，并实现多个方法，最后在入口函数 exercise_fn 中调用。
//
// 1. 定义结构体 `Book`（声明为 pub）：
//    - 字段 title: String（书名）
//    - 字段 price: f64（原价）
//    - 字段 stock: u32（库存数量）
//    - 所有字段声明为 pub
//

use std::panic::PanicHookInfo;

pub struct Book {
    pub title: String,
    pub price: f64,
    pub stock: u32,
}

// 2. 为 Book 实现如下方法（均为 pub）：
//    - `fn new(title: &str, price: f64, stock: u32) -> Self`：关联函数，构造 Book
//    - `fn discounted_price(&self, discount: f64) -> f64`：返回打折后价格
//      （原价 * (1 - discount)，例如原价100打8折 discount=0.2 → 80.0）
//    - `fn buy(&mut self, qty: u32) -> bool`：购买 qty 本，若库存足够则减少库存并返回 true，否则返回 false 且库存不变
//    - `fn is_low_stock(&self) -> bool`：库存小于 10 视为低库存，返回 true
//

impl Book {
    pub fn new(title: &str, price: f64, stock: u32) -> Self {
        Self {
            title: title.to_string(),
            price,
            stock,
        }
    }

    pub fn discounted_price(&self, discount: f64) -> f64 {
        self.price * (1.0 - discount)
    }

    pub fn buy(&mut self, qty: u32) -> bool {
        if self.stock >= qty {
            self.stock -= qty;
            true
        } else {
            false
        }
    }

    pub fn is_low_stock(&self) -> bool {
        if self.stock < 10 { true } else { false }
    }
}

// 3. 实现入口函数 exercise_fn：
//    - 创建 Book { title: "Rust编程", price: 100.0, stock: 5 }
//    - 返回一个元组 (打折后价格, 是否低库存)
//    - 打折用 0.2（8折）
//
// 示例：
// discounted_price(0.2) → 100 * 0.8 = 80.0
// 库存 5 < 10 → is_low_stock() = true
//
// 提示：
// - discounted_price 返回 self.price * (1.0 - discount)
// - buy 里先判断 stock >= qty，够则 stock -= qty 返回 true，否则返回 false
// - is_low_stock 返回 self.stock < 10
// - 方法都需 pub，字段需 pub

pub fn exercise_fn() -> (f64, bool) {
    // TODO: 创建 Book 并返回 (打折后价格, 是否低库存)
    let book = Book::new("Rust编程", 100.0, 5);
    (book.discounted_price(0.2), book.is_low_stock())
}
