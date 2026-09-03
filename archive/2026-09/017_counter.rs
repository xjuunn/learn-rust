// ============================================
// 题目编号: ex017
// 知识点: 所有权综合（定义结构体封装）
// 难度: 基础
// 所属章节: 02_所有权
// ============================================

// 题目要求：
// 请你从头定义一个"计数器"结构体 Counter，并实现其相关方法，
// 然后在入口函数 compute 中调用，完成一个累计总和的场景。
//
// 具体要做的事：
// 1. 定义一个结构体 Counter，它拥有一个字段 count: i32
// 2. 为 Counter 实现两个方法：
//    - fn add(&mut self, n: i32)，把 n 累加到 count
//    - fn total(&self) -> i32，返回当前 count
// 3. 在入口函数 compute 中：创建一个初始 count 为 0 的 Counter，
//    依次 add(5)、add(10)、add(-3)，最后返回 total()
//
// 示例：
// 输入：无（固定累加 5 + 10 - 3）
// 输出：12
//
// 提示：
// - 结构体定义：`struct Counter { count: i32 }`
// - 方法写在 `impl Counter { ... }` 中
// - 修改 count 的方法需要 `&mut self`；只读取的方法用 `&self`
// - 初始化结构体：`Counter { count: 0 }`（或为它写一个关联函数 new）


pub struct Counter {
    pub count: i32
}

impl Counter {
    pub fn new(count: i32) -> Self {
        Self {
            count
        }
    }

    pub fn add(&mut self, n:i32) {
        self.count += n;
    }

    pub fn total(&self)->i32 {
        self.count
    }
}



pub fn compute() -> i32 {
    // TODO: 在这里创建 Counter 并调用其方法，返回累计值
    let mut counter = Counter::new(0);
    counter.add(5);
    counter.add(10);
    counter.add(-3);
    counter.total()
}
