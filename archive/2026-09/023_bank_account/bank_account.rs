// ============================================
// 题目编号: ex023
// 知识点: 结构体方法（修复 bug）
// 难度: 基础
// 所属章节: 03_结构体
// ============================================

// 题目要求：
// 下面定义了一个银行账户结构体 BankAccount，但代码**包含几个 bug**，
// 导致测试无法通过甚至无法编译。请你找出并修复所有问题。
//
// BankAccount 的功能：
// - 字段 balance: f64（账户余额）
// - deposit(&mut self, amount: f64)：存款，余额增加 amount
// - withdraw(&mut self, amount: f64)：取款，余额减少 amount
// - balance(&self) -> f64：返回当前余额（供外部读取）
//
// 请修复以下问题（可能有多个）：
// 1. 可见性 bug：方法或字段缺少 pub，导致集成测试无法访问（报 E0624/E0603）
// 2. 逻辑 bug：某个方法的计算有误
// 3. 签名 bug：方法接收的参数类型或返回值类型有误
//
// 示例（修复后应满足）：
// 新账户 balance = 100.0，deposit(50.0) → 余额 150.0
// withdraw(30.0) → 余额 120.0
//
// 提示：
// - 检查每个方法是否是 pub fn
// - 检查字段 balance 是否 pub
// - 检查 deposit 和 withdraw 是否用 += 和 -= 正确更新余额
// - 检查 balance() 方法是否返回余额

pub struct BankAccount {
    pub balance: f64,
}

impl BankAccount {
    pub fn new(initial: f64) -> Self {
        Self { balance: initial }
    }

    // 存款：余额应增加 amount
    pub fn deposit(&mut self, amount: f64) {
        self.balance = self.balance + amount;
    }

    // 取款：余额应减少 amount
    pub fn withdraw(&mut self, amount: f64) {
        self.balance = self.balance - amount;
    }

    // 返回当前余额
    pub fn balance(&self) -> f64 {
        self.balance
    }
}

pub fn exercise_fn() -> f64 {
    let mut acc = BankAccount::new(100.0);
    acc.deposit(50.0);
    acc.withdraw(30.0);
    acc.balance()
}
