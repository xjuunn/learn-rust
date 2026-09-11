// ============================================
// 题目编号: ex059
// 知识点: 错误处理综合收官（电子钱包转账）
// 难度: 基础
// 所属章节: 06_错误处理
// ============================================
//
// 场景：一个「电子钱包系统」支持账户间转账。每笔转账可能失败：
//   1. 收款方账户不存在        -> AccountNotFound(账户名)
//   2. 转账金额必须大于 0      -> InvalidAmount(金额)
//   3. 付款方余额不足          -> InsufficientBalance
//
// 这是 06 错误处理章节的收官综合题，综合运用本章主要能力：
//   - 自定义错误类型 TransferError（带数据变体 + Display）
//   - 用 Result 表达每一步可能失败的业务逻辑
//   - 边界校验（金额 <= 0）
//   - 失败时不破坏账户状态
//
// 本题采用「定义函数 + 部分提供」形式：
//   - Account、TransferError 及其 Display/Error 已给出（正确）
//   - 你需要实现 find_account、transfer、exercise_fn 三处
//
// 注意：transfer 需要修改账户余额，所以入参是 &mut [Account]。

use std::fmt;

/// 一个电子钱包账户
pub struct Account {
    pub name: String,
    pub balance: f64,
}

/// 转账时的自定义错误类型
#[derive(Debug, PartialEq)]
pub enum TransferError {
    /// 收款方账户不存在，携带账户名
    AccountNotFound(String),
    /// 转账金额必须大于 0，携带金额
    InvalidAmount(f64),
    /// 付款方余额不足
    InsufficientBalance,
}

impl fmt::Display for TransferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransferError::AccountNotFound(name) => write!(f, "收款方账户 '{}' 不存在", name),
            TransferError::InvalidAmount(amount) => {
                write!(f, "转账金额必须大于 0，当前为 {}", amount)
            }
            TransferError::InsufficientBalance => write!(f, "余额不足"),
        }
    }
}

impl std::error::Error for TransferError {}

/// 在钱包列表中查找账户，返回其下标。找不到返回 None。
///
/// 例子：
///   find_account(&[Account{"A",100}, Account{"B",50}], "B") -> Some(1)
///   find_account(&[], "C") -> None
pub fn find_account(wallets: &[Account], name: &str) -> Option<usize> {
    // TODO 1：遍历 wallets，返回 name 匹配的账户下标
    // 提示：用迭代器 .position(|a| a.name == name)，它正好返回 Option<usize>
    for (index, wallet) in wallets.iter().enumerate() {
        if wallet.name == name {
            return Some(index);
        }
    }
    None
}

/// 从付款方向收款方转账 amount 金额。
///
/// 失败条件（按顺序检查）：
///   1. 收款方不存在  -> Err(AccountNotFound(收款方名))
///   2. amount <= 0   -> Err(InvalidAmount(amount))
///   3. 付款方余额不足 -> Err(InsufficientBalance)
/// 以上任一失败时，两个账户余额都不能改变。
///
/// 成功时更新付款方（扣款）与收款方（入账），返回 Ok(())。
///
/// 例子：
///   transfer(&mut [A:100, B:50], "A", "B", 30) -> Ok(())
///     之后 A 余额 70，B 余额 80
///   transfer(&mut [A:100, B:50], "A", "C", 30) -> Err(AccountNotFound("C"))
///   transfer(&mut [A:100, B:50], "A", "B", 0)  -> Err(InvalidAmount(0))
///   transfer(&mut [A:10,  B:50], "A", "B", 20) -> Err(InsufficientBalance)
pub fn transfer(
    wallets: &mut [Account],
    from: &str,
    to: &str,
    amount: f64,
) -> Result<(), TransferError> {
    // TODO 2：实现转账逻辑
    // 提示：
    //   1. 用 find_account 找收款方下标，None 则返回 Err(AccountNotFound(to.to_string()))
    //   2. amount <= 0 返回 Err(InvalidAmount(amount))
    //   3. 找到付款方下标，检查其 balance >= amount，否则 Err(InsufficientBalance)
    //   4. 三个下标都拿到后，用两个可变借出同时修改余额
    //      （注意：避免同一数组上的两个 &mut 冲突——
    //       可以先扣款再入账，或用 split_at_mut）
    let to_index = match find_account(wallets, to) {
        Some(index) => index,
        None => return Err(TransferError::AccountNotFound(to.to_string())),
    };
    if amount <= 0.0 {
        return Err(TransferError::InvalidAmount(amount));
    };
    let from_index = match find_account(wallets, from) {
        Some(index) => index,
        None => return Err(TransferError::AccountNotFound(from.to_string())),
    };
    let from_account = &mut wallets[from_index];
    if from_account.balance < amount {
        return Err(TransferError::InsufficientBalance);
    }
    from_account.balance -= amount;
    let to_account = &mut wallets[to_index];
    to_account.balance += amount;
    Ok(())
}

/// 入口函数：依次执行多笔转账，返回每笔的结果文本。
/// 成功 -> "{} -> {} 转账 {:.2} 元成功"，失败 -> "{} -> {} 失败：{错误}"。
///
/// 例子：
///   exercise_fn(&mut [A:100, B:50], &[("A","B",30.0),("A","C",10.0),("A","B",0.0)]) ->
///     ["A -> B 转账 30.00 元成功",
///      "A -> C 失败：收款方账户 'C' 不存在",
///      "A -> B 失败：转账金额必须大于 0，当前为 0"]
pub fn exercise_fn(wallets: &mut [Account], transfers: &[(&str, &str, f64)]) -> Vec<String> {
    // TODO 3：遍历 transfers，调用 transfer 并把结果转成文本
    let mut list: Vec<String> = Vec::new();
    for &(from, to, amount) in transfers {
        match transfer(wallets, from, to, amount) {
            Ok(()) => list.push(format!("{} -> {} 转账 {:.2} 元成功", from, to, amount)),
            Err(err) => list.push(format!("{} -> {} 失败：{}", from, to, err)),
        }
    }
    list
}
