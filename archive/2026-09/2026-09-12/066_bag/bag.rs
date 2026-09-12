// ============================================
// 题目编号: ex066
// 知识点: 泛型综合应用（内嵌集合 + 泛型方法）
// 难度: 进阶
// 所属章节: 07_泛型
// ============================================
//
// 冒险者的魔法背包 Bag<T>：能容纳任意件、任意类型的物品。
// 内部用标准库 Vec<T> 作为储物空间。
//
// 本题目已实现一半（new / push / len），请实现剩余部分：
//   TODO 1: pop        取出最后放入的物品（空包返回 None）
//   TODO 2: peek_last  偷看最后一件（借引用，不移走物品）
//   TODO 3: map<U, F>  泛型方法：把所有物品逐件加工成 U，返回新 Bag<U>（顺序不变）
//   TODO 4: exercise_fn 完成演示并返回
//
// 提示：
//   - pop 空包返回 None：Vec::pop() 恰到好处
//   - peek_last 返回 Option<&T>：Vec::last() 恰到好处
//   - map 的 f 接收 T 返回 U，用 items.into_iter() 逐件加工后 collect 成 Vec<U>
//   - key point：map 里的 f 会被调用多次，约束写 Fn(T) -> U（不是 FnOnce）
//   - 泛型方法写法（与上题相同）：
//       pub fn map<U, F>(self, f: F) -> Bag<U>
//       where F: Fn(T) -> U
//   - 所有被测试引用的项要保持 pub

/// 魔法背包：可装多件物品的泛型容器（内部为 Vec<T>）。
pub struct Bag<T> {
    /// 背包内的储物清单
    items: Vec<T>,
}

impl<T> Bag<T> {
    /// 创建空背包。
    pub fn new() -> Bag<T> {
        Bag { items: Vec::new() }
    }

    /// 放入一件物品（追加到背包）。
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    /// 返回已放入的物品件数。
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// 取出最后放入的物品（从背包移除）。
    /// 空背包时返回 None。
    /// 例子：依次 push(a) push(b) 后 pop() -> Some(b)
    pub fn pop(&mut self) -> Option<T> {
        // TODO 1：返回 self.items.pop()
        if self.len() == 0 {
            None
        } else {
            Some(self.items.pop()?)
        }
    }

    /// 偷看最后放入的物品（不移除、不转移所有权）。
    /// 空背包时返回 None。
    pub fn peek_last(&self) -> Option<&T> {
        // TODO 2：返回 self.items.last()
        self.items.last()
    }

    /// 泛型方法：把背包里所有物品逐件交给 f 加工，返回装新物品的 Bag<U>。
    /// 物品先后顺序保持不变。
    /// 例子：Bag[1, 2, 3].map(|x| x * 10) -> Bag[10, 20, 30]
    pub fn map<U, F>(self, f: F) -> Bag<U>
    where
        F: Fn(T) -> U,
    {
        // TODO 3：消费 self 后逐件加工，再组成新背包
        //   提示：items: Vec<U> = self.items.into_iter().map(f).collect()
        let items: Vec<U> = self.items.into_iter().map(f).collect();
        Bag { items }
    }
}

/// 入口：演示背包的各种操作。
/// 返回 (pop取出的值, 包内剩余件数, map后最后一件)：
///   - 依次放入 10 / 20 / 30
///   - 调用一次 pop() 得到 30（包内剩 10、20，共 2 件）
///   - 调用 map(|x| x * 10) 得到新包 [100, 200]
///   - 取新包 peek_last() 得 200
pub fn exercise_fn() -> (i32, usize, i32) {
    // TODO 4：按上面三步组装，返回 (取出的值, 剩余件数, 新包最后一件)
    //   提示：pop 返回 Option<i32>，用 unwrap() 取值；peek_last 返回 Option<&i32>，解引用得到值
    let mut bag: Bag<i32> = Bag::new();
    bag.push(10);
    bag.push(20);
    bag.push(30);
    let popd = bag.pop().unwrap();
    let len = bag.len();
    let mapped = bag.map(|x| x * 10);
    (popd, len, *mapped.peek_last().unwrap())
}
