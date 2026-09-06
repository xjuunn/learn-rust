// ============================================
// 题目编号: ex027
// 知识点: 结构体克隆（Clone 与移动）
// 难度: 基础
// 所属章节: 03_结构体
// ============================================

// 题目要求：
// 军队出征前要给每名骑士制作"任务卡"副本，副本与原件相互独立，
// 修改副本不能影响原件。本题需要你**新建一个文件**，再接线使用。
//
// 步骤：
// 1. 新建文件 `src/exercises/knight.rs`，在其中定义 `Knight` 结构体（全部 pub）：
//    - 顶部加 `#[derive(Clone, Debug)]`（克隆 + 调试打印）
//    - 字段 name: String（姓名）、hp: u32（生命）、atk: u32（攻击）
//    - 关联函数 `pub fn new(name: &str, hp: u32, atk: u32) -> Self`
//    - 方法 `pub fn take_damage(&mut self, dmg: u32)`：生命减少 dmg，最低为 0
// 2. 回到本文件顶部（第一个注释块之前），添加两行接线代码：
//      mod knight;              // 引入同目录下的 knight.rs
//      pub use knight::Knight;  // 把 Knight 转发给外部（测试经 current 访问）
// 3. 实现下面的入口函数 exercise_fn -> (String, u32, u32)：
//    - 创建 Knight::new("亚瑟", 100, 30)
//    - 调用 .clone() 得到一份独立副本
//    - 让副本 take_damage(25)
//    - 返回 (原本姓名, 原本生命, 副本生命)
//
// 示例：
// 原本 亚瑟 生命 100；副本 100 受伤 25 → 75
// 期望返回: ("亚瑟", 100, 75)
//
// 提示：
// - 修改副本不影响原本，这正是 Clone 与移动（move）的关键区别：
//   移动后原值失效，克隆后两者并存且独立
// - take_damage 建议：if dmg >= hp { hp = 0 } else { hp -= dmg }，保证不出现负数
// - 文件放好后，cargo check 不应再报 "file not found for module `knight`"
// - 需要被测试访问的字段与方法都要 pub，否则测试报 E0603 私有项错误

pub use crate::exercises::Knight;

pub fn exercise_fn() -> (String, u32, u32) {
    // TODO: 创建"亚瑟"，克隆副本，副本受伤 25，返回 (姓名, 原本hp, 副本hp)
    let knight = Knight::new("亚瑟", 100, 30);
    let mut knight_copy = knight.clone();
    knight_copy.take_damage(25);
    (
        knight.name.clone(),
        knight.hp.clone(),
        knight_copy.hp.clone(),
    )
}
