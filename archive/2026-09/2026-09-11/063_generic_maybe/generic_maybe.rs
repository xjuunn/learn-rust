// ============================================
// 题目编号: ex063
// 知识点: 泛型枚举（基础）
// 难度: 基础
// 所属章节: 07_泛型
// ============================================
//
// 你已经见过标准库的泛型枚举：
//   Option<T> = Some(T) | None        （可能缺失）
//   Result<T, E> = Ok(T) | Err(E)     （可能出错）
//
// 本题自己动手实现一个迷你版 Option——泛型枚举 Maybe<T>，
// 从而透彻理解「枚举可以带类型参数」。
//   enum Maybe<T> { Just(T), Nothing }
//
// 任务：
//   1. 实现三个普通泛型方法（impl<T>）
//   2. 实现一个带 Clone 约束的泛型方法（impl<T: Clone>）
//   3. 写入口验证多类型使用

use crate::exercises::generic_maybe::Maybe::{Just, Nothing};

/// 迷你版 Option：Maybe<T> 要么有值（Just(T)），要么没有（Nothing）。
pub enum Maybe<T> {
    /// 有值
    Just(T),
    /// 无值
    Nothing,
}

impl<T> Maybe<T> {
    /// 是否为有值状态。
    ///
    /// 例子：Just(5).is_just() -> true，Nothing.is_just() -> false
    pub fn is_just(&self) -> bool {
        // TODO 1：用 match 判断是否为 Just(_)
        match &self {
            Self::Just(_) => true,
            Self::Nothing => false,
        }
    }

    /// 是否为无值状态。
    ///
    /// 例子：Nothing.is_nothing() -> true
    pub fn is_nothing(&self) -> bool {
        // TODO 2：用 match 判断是否为 Nothing
        match &self {
            Self::Just(_) => false,
            Self::Nothing => true,
        }
    }

    /// 有值时取出内部值，无值时返回 default。
    ///
    /// 注意：这里消费 self（按值接收），两者都返回 T。
    /// 例子：Just(5).unwrap_or(0) -> 5，Nothing.unwrap_or(0) -> 0
    pub fn unwrap_or(self, default: T) -> T {
        // TODO 3：match self，Just(v) 返回 v，Nothing 返回 default
        match self {
            Self::Just(v) => v,
            Self::Nothing => default,
        }
    }
}

impl<T: Clone> Maybe<T> {
    /// 复制一份内部值：有值时返回副本，无值时返回 default。
    /// 与 unwrap_or 的 Try…… 的区别：unwrap_or 会移动原值导致无法再用，
    /// 而 clone_or 保留原 Maybe 仍可再次使用。
    ///
    /// 例子：
    ///   let m = Just(5);
    ///   let v = m.clone_or(0);   // v = 5
    ///   m.is_just();             // m 仍可用（Copy 的 i32 场景下）
    pub fn clone_or(&self, default: T) -> T {
        // TODO 4：match &self，Just(v) 返回 v.clone()，Nothing 返回 default
        match &self {
            Self::Just(v) => v.clone(),
            Self::Nothing => default,
        }
    }
}

/// 入口：用多种类型验证 Maybe<T> 泛型枚举。
/// 返回 (整型is_just, 整型is_nothing, 浮点unwrap_or结果, 字符串clone_or结果)：
///   - Just(5).is_just()
///   - Just(5).is_nothing()
///   - Nothing.unwrap_or(3.14)            （f64）
///   - Just("hi").clone_or("empty")        （&str，注意 Clone）
pub fn exercise_fn() -> (bool, bool, f64, &'static str) {
    // TODO 5：用上面四行构造并返回
    (
        Just(5).is_just(),
        Just(5).is_nothing(),
        Nothing.unwrap_or(3.14),
        Just("hi").clone_or("empty"),
        )
}
