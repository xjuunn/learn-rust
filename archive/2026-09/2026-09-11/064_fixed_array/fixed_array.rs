// ============================================
// 题目编号: ex064
// 知识点: const 泛型（定长容器）
// 难度: 基础
// 所属章节: 07_泛型
// ============================================
//
// 泛型不仅能带「类型参数」，还能带「数值参数」——这就是 const 泛型：
//   struct FixedArray<T, const N: usize>
// 这里 N 是一个编译期常量，它让「长度」变成类型的一部分：
//   FixedArray<i32, 3> 与 FixedArray<i32, 5> 是两种不同类型！
//
// 核心语法（本题的关键）：
//   - 类型参数后写 `const N: usize` 声明 const 泛型
//   - 在结构体里可以用 N 初始化定长数组：[value; N]
//   - 函数也可用 const 泛型：fn identity<const N: usize>()
//
// 题目：实现一个「定长容器」FixedArray<T, N> 及相关函数。

/// 定长容器：长度 N 在编译期确定。
pub struct FixedArray<T, const N: usize> {
    /// 内部定长数组
    pub items: [T; N],
}

impl<T: Copy, const N: usize> FixedArray<T, N> {
    /// 用同一个值填满整个容器。
    ///
    /// 例子：FixedArray::<i32, 3>::filled(7) -> { items: [7, 7, 7] }
    pub fn filled(value: T) -> FixedArray<T, N> {
        // TODO 1：用数组初始化语法 [value; N] 构造（N 是 const 泛型，可直接用于数组长度）
        Self { items: [value; N] }
    }
}

impl<T, const N: usize> FixedArray<T, N> {
    /// 返回容器长度（编译期已知的 N）。
    ///
    /// 例子：FixedArray::<i32, 5>::filled(1).len() -> 5
    pub fn len(&self) -> usize {
        // TODO 2：直接返回 N
        N
    }

    /// 按下标取元素引用；下标越界时返回 None（不 panic）。
    ///
    /// 例子：
    ///   let a = FixedArray::<i32, 3>::filled(7);
    ///   a.get(0) -> Some(&7)
    ///   a.get(3) -> None                 （越界，长度只有 3）
    pub fn get(&self, index: usize) -> Option<&T> {
        // TODO 3：index < N 时返回 Some(&items[index])，否则 None
        if index >= N {
            return None;
        }
        Some(&self.items[index])
    }
}

/// 生成下标数组 [0, 1, 2, ..., N-1]。用 const 泛型的另一个例子。
///
/// 例子：identity::<4>()  ->  [0, 1, 2, 3]
pub fn identity<const N: usize>() -> [usize; N] {
    // TODO 4：用 std::array::from_fn 生成 f(i) = i 的数组
    // 提示：std::array::from_fn(|i| i)
    std::array::from_fn(|f| f)
}

/// 入口：演示不同长度 N / 不同元素类型的 FixedArray。
/// 返回 (取到的第3号元素, 容器A长度, 容器B长度, identity::<4>()[2])：
///   - let a: FixedArray<i32, 3> = FixedArray::filled(7);   再 a.get(2) 解引用
///   - a.len()
///   - let b: FixedArray<f64, 5> = FixedArray::filled(1.5); 再 b.len()
///   - identity::<4>()[2]
pub fn exercise_fn() -> (i32, usize, usize, usize) {
    // TODO 5：按上面四行构造并返回
    let a: FixedArray<i32, 3> = FixedArray::filled(7);
    let b: FixedArray<f64,5> = FixedArray::filled(1.5);
    (
        *a.get(2).unwrap(),
        a.len(),
        b.len(),
        identity::<4>()[2],
    )
}
