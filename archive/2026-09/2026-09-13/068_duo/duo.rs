// ============================================
// 题目编号: ex068
// 知识点: 多类型参数的泛型（Duo 二元组合）
// 难度: 进阶
// 所属章节: 07_泛型
// ============================================
//
// 之前的结构体只带一个类型参数（Vault<T>、Bag<T>、Ranker<T>），
// 本题体验「多个类型参数」：struct Duo<A, B> 的 first、second 可以
// 是两种**互不相同**的类型。泛型方法还能在 A、B 之外引入第三个
// 类型参数 U，实现单向「换类型」。
//
// 场景：情报局的加密组合 Duo<A, B> —— 一份密文 + 一条密钥，
// 密文、密钥各管各的类型。
//
// 任务（结构体与字段已给出）：
//   1. new(first, second) 构造组合
//   2. swap() 把 first/second 互换，返回 Duo<B, A>
//      （注意：互换后连类型参数的位置都颠倒了）
//   3. map_first<U, F>(self, f)  只加工 first：A → U，second 保持 B
//   4. map_second<U, F>(self, f) 只加工 second：B → U，first 保持 A
//
// 提示：
//   - swap 的返回类型是 Duo<B, A>（类型参数位置颠倒），体会「类型参数随值交换」
//   - map_first / map_second 是泛型方法（方法自身带 U），约束 F: FnOnce(...) -> U
//   - 所有方法保持 pub

/// 情报加密组合：一份密文 + 一条密钥，各自可以是不同类型。
pub struct Duo<A, B> {
    /// 密文（类型 A）
    pub first: A,
    /// 密钥（类型 B）
    pub second: B,
}

impl<A, B> Duo<A, B> {
    /// 构造一个组合。
    /// 例子：Duo::new("密文", 12345u64) -> Duo<&str, u64>
    pub fn new(first: A, second: B) -> Duo<A, B> {
        // TODO 1：返回 Duo { first, second }
        Self { first, second }
    }

    /// 交换密文和密钥的位置，返回新组合。
    /// 注意返回类型：Duo<B, A>，两个类型参数的位置也随值对调。
    /// 例子：Duo::new("a", 1).swap() -> Duo { first: 1, second: "a" }
    pub fn swap(self) -> Duo<B, A> {
        // TODO 2：返回 Duo { first: self.second, second: self.first }
        Duo {
            first: self.second,
            second: self.first,
        }
    }

    /// 只加工密文：调用 f 把 A 变成 U，密钥原样保留（类型 B）。
    /// 例子：Duo::new(5, "key").map_first(|n| n * 2) -> Duo { first: 10, second: "key" }
    pub fn map_first<U, F>(self, f: F) -> Duo<U, B>
    where
        F: FnOnce(A) -> U,
    {
        // TODO 3：f(self.first) 得到新密文，second 原样搬运
        Duo {
            first: f(self.first),
            second: self.second,
        }
    }

    /// 只加工密钥：调用 f 把 B 变成 U，密文原样保留（类型 A）。
    /// 例子：Duo::new("msg", 7).map_second(|k| k * 2) -> Duo { first: "msg", second: 14 }
    pub fn map_second<U, F>(self, f: F) -> Duo<A, U>
    where
        F: FnOnce(B) -> U,
    {
        // TODO 4：f(self.second) 得到新密钥，first 原样搬运
        Duo {
            first: self.first,
            second: f(self.second),
        }
    }
}

/// 入口：演示多类型参数与换型。
/// 返回 (digits, 文案, 字符数)：
///   - Duo::new(5, "剑士")，map_second 加工成 "精锐剑士"（Duo<&str?, String>）
///   - 取 second 得 "精锐剑士"
///   - Duo::new("名字", 7).map_first(|s| s.chars().count()) 得 first = 2
///   - 返回 (5, "精锐剑士", 2)
pub fn exercise_fn() -> (i32, String, usize) {
    // TODO 5：按上面三步组装，返回三元组
    //   提示：duo.second 是 String，直接拿去用即可
    let duo1 = Duo::new(5, "剑士").map_second(|_| "精锐剑士");
    let duo2 = Duo::new("名字", 7).map_first(|s| s.chars().count());
    (duo1.first, duo1.second.to_string(), duo2.first)
}