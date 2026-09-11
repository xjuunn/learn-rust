// ============================================
// 题目编号: ex062
// 知识点: 泛型结构体（基础）
// 难度: 基础
// 所属章节: 07_泛型
// ============================================
//
// 上一题学会了泛型函数 <T>，本题学习「泛型结构体」。
//   - 结构体可以带类型参数：struct Pair<T>
//   - 方法也能带类型参数：impl<T> Pair<T>
//   - 同一个结构体可用于多种类型（Pair<i32>、Pair<String> ...）
//
// 还要注意一个细节：「泛型方法 + 方法级约束」
//   impl<T: PartialOrd> Pair<T> 只对可比较类型的 Pair 生效。
//
// 题目：实现一个通用「对子」结构体 Pair<T>，包含四个方法。

/// 一个容纳两个同类型值的「对子」。
pub struct Pair<T> {
    pub first: T,
    pub second: T,
}

impl<T> Pair<T> {
    /// 构造一个新的 Pair（关联函数）。
    ///
    /// 例子：Pair::new(1, 2) -> Pair { first: 1, second: 2 }
    pub fn new(first: T, second: T) -> Pair<T> {
        // TODO 1：返回一个 Pair { first, second }
        Self {
            first,
            second
        }
    }

    /// 交换两个值，返回新的 Pair（first/second 对调）。
    ///
    /// 例子：Pair { first: 1, second: 2 }.swap() -> Pair { first: 2, second: 1 }
    pub fn swap(self) -> Pair<T> {
        // TODO 2：返回 first/second 对调的新 Pair
        Self {
            first: self.second,
            second: self.first
        }
    }

    /// 返回 first 的引用（不移动值）。
    pub fn first_ref(&self) -> &T {
        // TODO 3：返回 &self.first
        return &self.first
    }
}

/// 仅当 T 支持比较（PartialOrd）时，Pair<T> 才有 larger 方法。
impl<T: PartialOrd> Pair<T> {
    /// 返回较大值的引用。
    ///
    /// 例子：Pair { first: 3, second: 5 }.larger() -> &5
    pub fn larger(&self) -> &T {
        // TODO 4：比较 first 和 second，返回较大者的引用
        if self.first > self.second {
            &self.first
        } else {
            &self.second
        }
    }
}

/// 入口：构造多种类型的 Pair 验证泛型结构体。
/// 返回 (整型first, 浮点second, 字符串较大者, 整型交换后的first)：
///   - Pair::new(10, 20)，取 first_ref()
///   - Pair::new(1.5, 2.5)，取 second 字段（直接访问）
///   - Pair::new("rust", "python")，调用 larger()
///   - Pair::new(7, 3)，调用 swap() 后取 first
pub fn exercise_fn() -> (i32, f64, &'static str, i32) {
    // TODO 5：用上面四个点构造并返回
    (
        *Pair::new(10,20).first_ref(),
        Pair::new(1.5,2.5).second,
        Pair::new("rust", "python").larger(),
        Pair::new(7,3).swap().first,
    )
}