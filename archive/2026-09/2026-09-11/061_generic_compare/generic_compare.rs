// ============================================
// 题目编号: ex061
// 知识点: 泛型函数（基础）
// 难度: 基础
// 所属章节: 07_泛型
// ============================================
//
// 新章节「泛型」第 1 题。泛型让你写一次代码就能适用于多种类型。
// 本章只用一个尖括号 <T> 语法，函数就能对任意类型工作。
//
// 本题练习两个泛型函数，理解 <T: PartialOrd> 约束：
//   - PartialOrd 表示 T 支持 < / <= / > / >= 比较（i32、f64、String 等都有）
//
// 用 Todo! 补全函数体。max_of 和 clamp 都要用 <T: PartialOrd>。

/// 返回 a 和 b 中较大的一个。
///
/// 例子：
///   max_of(3, 5) -> 5
///   max_of(2.5, 1.5) -> 2.5
///   max_of("app", "banana") -> "banana"
pub fn max_of<T: PartialOrd>(a: T, b: T) -> T {
    // TODO 1：a >= b 时返回 a，否则返回 b
    if a >= b {
        return a;
    }
    return b;
}

/// 把 value 限制在 [low, high] 范围内：
///   - value < low 时返回 low
///   - value > high 时返回 high
///   - 否则返回 value 本身
///
/// 例子：
///   clamp(5, 0, 10) -> 5
///   clamp(-3, 0, 10) -> 0
///   clamp(15, 0, 10) -> 10
///   clamp(7.5, 0.0, 5.0) -> 5.0
pub fn clamp<T: PartialOrd>(value: T, low: T, high: T) -> T {
    // TODO 2：按上面三条规则返回（用 if / else if / else）
    if value < low {
        return low;
    } else if value > high {
        return high;
    }
    return value;
}

/// 入口：用三种不同类型调用上面的泛型函数，验证类型推断。
/// 返回 (整型结果, 浮点结果, 字符串结果)：
///   - 整型：max_of(3, 9)
///   - 浮点：clamp(7.5, 0.0, 5.0)
///   - 字符串：max_of("rust", "python")
pub fn exercise_fn() -> (i32, f64, &'static str) {
    // TODO 3：分别调用 max_of / clamp 并返回三元组

    (max_of(3, 9), clamp(7.5, 0.0, 5.0), max_of("rust", "python"))
}
