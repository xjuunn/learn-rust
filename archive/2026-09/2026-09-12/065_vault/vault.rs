// ============================================
// 题目编号: ex065
// 知识点: 泛型与方法定义
// 难度: 基础
// 所属章节: 07_泛型
// ============================================
//
// 本章回顾：
//   ex061 泛型函数   fn max_of<T>(...)
//   ex062 泛型结构体 struct Pair<T> + impl<T> Pair<T>
//   ex064 const 泛型 FixedArray<T, N>
//
// 本题聚焦「泛型方法」——方法自身携带额外的类型参数，
// 它能实现一种「类型变换」能力：Vault<Gems> 可以变成 Vault<String>。
//
// 场景：精灵王国的保险箱 Vault<T>，一个箱子只能存放一件宝物。
//   请补全下面所有 TODO。
//
// 任务：
//   1. 关联函数 new：创建保险箱，开箱次数初始为 0
//   2. 方法 peek：返回宝物引用（不打开箱子，次数不变）
//   3. 方法 open_count：返回开箱次数
//   4. 方法 deposit：放入新宝物（覆盖旧宝物），开箱次数 +1
//   5. 泛型方法 transform<U, F>：用闭包 f 把宝物 T 加工成 U，
//      返回装新宝物 U 的 Vault<U>（f 调用算一次开箱，次数随箱转移）
//
// 提示：
//   - impl 块写作 impl<T> Vault<T>，方法内直接使用类型参数 T / U
//   - 泛型方法写法：
//       pub fn transform<U, F>(self, f: F) -> Vault<U>
//       where F: FnOnce(T) -> U
//   - FnOnce 表示「调用一次就耗尽」的闭包，最适合传给加工函数
//   - 只要用到的项都是 pub，集成测试才能访问（E0603 检查）


/// 保险箱：存放一件宝物，并记录开箱次数。
pub struct Vault<T> {
    /// 箱内宝物（类型为泛型参数 T）
    pub item: T,
    /// 累计开箱次数
    pub access: u32,
}

impl<T> Vault<T> {
    /// 创建保险箱：放入宝物，开箱次数为 0。
    /// 例子：Vault::new(5) -> 存放 i32 的保险箱
    pub fn new(item: T) -> Vault<T> {
        // TODO 1：返回 Vault { item, access: 0 }
        Self { item, access: 0 }
    }

    /// 查看宝物引用（不改变开箱次数）。
    /// 例子：Vault::new(5).peek() -> &5
    pub fn peek(&self) -> &T {
        // TODO 2：返回 &self.item
        &self.item
    }

    /// 返回开箱次数。
    pub fn open_count(&self) -> u32 {
        // TODO 3：返回 self.access
        self.access
    }

    /// 放入新宝物（覆盖旧宝物），开箱次数 +1。
    pub fn deposit(&mut self, item: T) {
        // TODO 4：把 self.item 替换为 item，并把 self.access 加 1
        self.item = item;
        self.access += 1;
    }

    /// 泛型方法：把宝物 T 交给闭包 f 加工成 U，返回装新宝物的 Vault<U>。
    /// f 被调用的这一次也算开箱（次数 +1），新箱子的开箱次数与原箱一致。
    /// 例子：
    ///   let v = Vault::new(String::from("宝石"));
    ///   let w = v.transform(|s| s.chars().count());  // Vault<usize>
    ///   *w.peek() == 2
    pub fn transform<U, F>(mut self, f: F) -> Vault<U>
    where
        F: FnOnce(T) -> U,
    {
        // TODO 5：
        //   1) self.access += 1（f 的调用算一次开箱）
        //   2) 取出 self.item 交给 f 加工，得到新宝物
        //   3) 返回 Vault { item: f(self.item), access: self.access }
        self.access += 1;
        Vault {
            item: f(self.item),
            access: self.access,
        }
    }
}

/// 入口：演示「同箱不同宝、泛型方法变换类型」。
/// 返回 (最终宝物, 开箱次数, 变换后箱子的宝物)：
///   - 创建存放 "金币" 的 Vault<String>
///   - deposit 存入 "宝石"（开箱 1 次）
///   - peek 得到当前宝物 "宝石"
///   - 用 transform 把字符串加工成字符数（Vault<String> -> Vault<usize>）
///     示例 f: |s| s.chars().count()，结果应为 2
pub fn exercise_fn() -> (String, u32, usize) {
    // TODO 6：按上面四步构造并返回 (宝物, 开箱次数, 变换结果)
    // 提示：peek 返回 &String，需 .clone() 才能得到 String
    let mut jb = Vault::new("金币");
    jb.deposit("宝石");
    let name = jb.peek().to_string();
    let len = jb.transform(|f| f.chars().count());

    (name, len.access, *len.peek())
}
