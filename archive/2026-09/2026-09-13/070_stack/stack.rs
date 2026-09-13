// ============================================
// 题目编号: ex070
// 知识点: 泛型结构体方法与 impl 分裂（栈 Stack<T>）
// 难度: 进阶
// 所属章节: 07_泛型
// ============================================
//
// 本题是「泛型」板块收官：体验泛型结构体方法的一个关键写法——
// **impl 分裂**。同一个 Stack<T> 可以有多个 impl 块：
//   - impl<T> Stack<T>         适用所有类型 T 的方法
//   - impl<T: Clone> Stack<T>  只有 T 满足 Clone 时才有这些方法
// 这就是为什么"能 dup 复制"的方法要放在带约束的 impl 里。
//
// 场景：火车站的车厢调度区（后进先出：最后挂上的车厢最先脱开）。
//
// 任务（结构体与多个 impl 块已给出，函数体完全由你实现）：
//
//   1. impl<T> Stack<T>：
//      - new() 新建空栈
//      - push(&mut self, item: T) 入栈
//      - pop(&mut self) -> Option<T> 出栈（空栈返回 None）
//      - peek(&self) -> Option<&T> 只看栈顶不取出
//      - is_empty(&self) -> bool
//      - len(&self) -> usize
//   2. impl<T: Clone> Stack<T>：
//      - dup(&self) -> Stack<T> 返回一份内容相同的深拷贝栈
//      （此刻体会有无 Clone 约束的差别）
//
// exercise_fn 演示车厢调度，无需修改：
//   依次挂接 3 → 7 → 9 号车厢：
//     - peek 得栈顶 9（不移除）
//     - pop 弹出 9
//     - dup 复制剩余 [7, 3]
//  返回 (peek 值, pop 值, 副本栈长度, 原文栈是否已空)
//
// 提示：
//   - pop / push 就是 Vec 的 pop / push，注意栈顶约定
//   - peek 返回的是借用 &T，不是 T 本身
//   - 过程遇到"不能移动 Vec 内值"的报错时，想想借用 vs 所有权
//   - 所有方法和结构体保持 pub
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    /// 新建空栈。
    pub fn new() -> Self {
        // TODO: 返回内容为空的 Stack
        Self { items: Vec::new() }
    }

    /// 入栈：把 item 追加到栈顶。
    pub fn push(&mut self, item: T) {
        // TODO: 推入 items
        self.items.push(item);
    }

    /// 出栈：返回并移除栈顶元素；空栈返回 None。
    pub fn pop(&mut self) -> Option<T> {
        // TODO: 取出栈顶
        self.items.pop()
    }

    /// 查看栈顶（借用），不移除。
    pub fn peek(&self) -> Option<&T> {
        // TODO: 返回栈顶引用
        if self.items.is_empty() {
            return None;
        }
        Some(&self.items[self.items.len() - 1])
    }

    /// 是否为空栈。
    pub fn is_empty(&self) -> bool {
        // TODO: 判断 items 是否为空
        self.items.is_empty()
    }

    /// 栈内元素个数。
    pub fn len(&self) -> usize {
        // TODO: 返回 items 长度
        self.items.len()
    }
}

impl<T: Clone> Stack<T> {
    /// 返回一份内容相同的深拷贝栈（原栈保持不变）。
    /// 注意：此方法只在 T 满足 Clone 时可用。
    pub fn dup(&self) -> Stack<T> {
        // TODO: 克隆 items 构造新栈
        Stack {
            items: self.items.clone(),
        }
    }
}

/// 入口：演示车厢调度。
/// 返回 (peek 值, pop 值, 副本长度, 原栈是否已空)。
pub fn exercise_fn() -> (Option<i32>, Option<i32>, usize, bool) {
    let mut s = Stack::new();
    s.push(3);
    s.push(7);
    s.push(9);
    let top = s.peek().copied();
    let popped = s.pop();
    let copy = s.dup();
    let copy_len = copy.len();
    (top, popped, copy_len, s.is_empty())
}
