// ============================================
// 题目编号: ex067
// 知识点: 泛型与 trait 边界（深化）
// 难度: 进阶
// 所属章节: 07_泛型
// ============================================
//
// 泛型类型在使用时往往需要给类型参数加「边界」（bounds），
// 限定 T 必须具备某些能力才能参与运算：
//   - T: PartialOrd → T 可比较大小（>、<），是 best/worst 的前提
//   - T: Clone      → T 可复制，才能「返回副本」而不移动榜单里的原成绩
//
// 场景：公会演武场的战绩榜 Ranker<T> —— 登记任意「可比较、可复制」的成绩。
// 请补全下面所有 TODO。
//
// 任务（结构体与字段已给出）：
//   1. new()                创建空榜单
//   2. add(item)            登记一条成绩（追加到末尾）
//   3. count() -> usize     当前登记数量
//   4. best() -> Option<T>  返回最高成绩的副本（榜单为空返回 None）
//   5. worst() -> Option<T> 返回最低成绩的副本（榜单为空返回 None）
//
// 要求：
//   - 约束写在 impl 块上，而不是结构体上
//   - 只需 Clone+PartialOrd 的方法（best/worst）所在 impl 块才加约束
//
// 提示：
//   - 先写 impl<T> Ranker<T>：new / add / count（无需任何约束）
//   - 再写 impl<T: Clone + PartialOrd> Ranker<T>：best / worst
//   - best/worst 思路：先取出第一项 clone 为「当前最优/最差」，逐个与后项比较更新
//   - 所有项都保持 pub（E0603 检查）

/// 公会演武场战绩榜：登记任意可比较、可复制的成绩。
pub struct Ranker<T> {
    /// 已登记的成绩，按登记顺序存放
    pub records: Vec<T>,
}

impl<T> Ranker<T> {
    /// 创建空榜单。
    /// 例子：Ranker::<i32>::new()——空榜单，成绩类型为 i32
    pub fn new() -> Ranker<T> {
        // TODO 1：返回 Ranker { records: Vec::new() }
        Self {
            records: Vec::new(),
        }
    }

    /// 登记一条成绩（追加到末尾）。
    pub fn add(&mut self, item: T) {
        // TODO 2：把 item 追加进 self.records
        self.records.push(item);
    }

    /// 返回当前登记的成绩数量。
    pub fn count(&self) -> usize {
        // TODO 3：返回 self.records.len()
        self.records.len()
    }
}

impl<T: Clone + Ord> Ranker<T> {
    /// 返回最高成绩的副本。榜单为空时返回 None。
    pub fn best(&self) -> Option<T> {
        // TODO 4：
        //   1) 空榜单返回 None
        //   2) 否则把第一条记录 clone 为当前最高，逐条比较后返回最高的副本
        // if self.count() == 0 {
        //     None
        // } else {

        // }
        self.records.iter().cloned().max()
    }

    /// 返回最低成绩的副本。榜单为空时返回 None。
    pub fn worst(&self) -> Option<T> {
        // TODO 5：逻辑与 best 相反，找最低成绩
        self.records.iter().cloned().min()
    }
}

/// 入口：演示战绩榜。
/// 返回 (最高, 最低, 数量)：
///   - 依次登记 30 / 10 / 20
///   - best() 与 worst() 应分别为 30、10；count() 为 3
pub fn exercise_fn() -> (i32, i32, usize) {
    // TODO 6：构造榜单登记 30、10、20，返回 (best, worst, count)
    //   提示：Option<i32> 需要 unwrap() 取值
    let mut ranker = Ranker::new();
    ranker.add(30);
    ranker.add(20);
    ranker.add(10);
    (
        ranker.best().unwrap(),
        ranker.worst().unwrap(),
        ranker.count(),
    )
}
