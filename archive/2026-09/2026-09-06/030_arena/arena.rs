// ============================================
// 题目编号: ex030
// 知识点: 借用
// 难度: 基础
// 所属章节: 03_结构体
// ============================================

// 题目要求：
// 竞技场主持人既要只读查看玩家总分，又要允许实时累加分数。
// 只读用 &self 借用，修改用 &mut self 借用；把数据传给函数时
// 应借引用而非转移所有权。本题需要你**从 0 定义**一个玩家计分板。
//
// 步骤：
// 1. 定义结构体（字段均 pub）：
//      pub struct PlayerScore { pub name: String, pub score: u32 }
pub struct PlayerScore {
    pub name: String,
    pub score: u32,
}
// 2. 为 PlayerScore 实现方法（均 pub）：
//      - pub fn new(name: &str) -> Self：新玩家，初始分数 0
//      - pub fn add_points(&mut self, points: u32)：
//        累加分数（可变借用 self）
//      - pub fn show(&self) -> String：
//        返回 "名字: 分数"，如 "汉克: 30"（只读借用 self）
impl PlayerScore {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            score: 0,
        }
    }

    pub fn add_points(&mut self, point: u32) {
        self.score += point;
    }

    pub fn show(&self) -> String {
        format!("{}: {}", self.name, self.score)
    }
}

// 3. 实现一个游离函数（不在 impl 内，需 pub）：
//      pub fn total_score(players: &[PlayerScore]) -> u32：
//      借入玩家切片，把所有分数相加（不得转移所有权）
pub fn total_score(players: &[PlayerScore]) -> u32 {
    let mut sum = 0;
    for player in players {
        sum += player.score;
    }
    sum
}

// 4. 实现入口函数（pub），供 main 与测试调用：
//      pub fn exercise_fn() -> (String, u32)
//    - 创建两名玩家：汉克、格罗特（初始 0 分）
//    - 汉克 add_points(30)，格罗特 add_points(25)
//    - 返回 (汉克.show() 的结果, total_score(全部玩家))
//
// 示例：
//   汉克 30 分、格罗特 25 分
//   汉克.show() -> "汉克: 30"
//   total_score -> 30 + 25 = 55
//   期望返回: ("汉克: 30", 55)
//
// 提示：
// - &self 只读借用后 self 仍可用；&mut self 可变借用期间不可再同时借用
// - total_score 里 for p in players 遍历即可（players 是 &[..]，迭代即借用元素）
// - 变量要声明为 mut 才能调用 add_points（接收 &mut self）
// - add_points 直接写 self.score += points 即可
// - 类型与函数都要 pub，否则集成测试访问报 E0603

pub fn exercise_fn() -> (String, u32) {
    // TODO: 创建 汉克、格罗特，加分，返回 (汉克.show(), 总分数)
    let mut player1 = PlayerScore::new("汉克");
    let mut player2 = PlayerScore::new("格罗特");
    player1.add_points(30);
    player2.add_points(25);
    (player1.show(), total_score(&[player1, player2]))
}
