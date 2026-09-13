// ============================================
// 题目编号: ex071
// 知识点: Trait 定义与实现
// 难度: 基础
// 所属章节: 08_Trait
// ============================================
//
// 告别泛型板块，进入 **Trait（特质）** 板块。Trait 给任意类型定义
// 一组公共能力：不同类型实现同一个 Trait，就能用同一套方法名调用，
// 各自的行为却不相同。
//
// 场景：战棋游戏的角色情报面板。骑士看生命值、法师看魔力值，
// 但它们都能执行同一个动作——describe（自我描述）。
//
// 任务（Trait 定义与两个结构体已给出，impl 完全由你实现）：
//
//   1. 为 `Knight`（骑士）实现 `Describe`：
//      返回格式 `"骑士{name} HP{hp}"`
//      例：name="阿德", hp=120 -> "骑士阿德 HP120"
//   2. 为 `Mage`（法师）实现 `Describe`：
//      返回格式 `"法师{name} MP{mp}"`
//      例：name="露娜", mp=80 -> "法师露娜 MP80"
//   3. exercise_fn 已写好：构造阿德与露娜，分别 describe，
//      返回 (骑士情报, 法师情报)
//
// 提示：
//   - describe(&self) 借用自身，不消耗角色
//   - 用 format! 拼接字符串即可
//   - 体会：Knight 与 Mage 类型不同，但都能调用 describe()
//   - trait 定义、impl 后的类型与 impl 块保持 pub（题目已满足）
pub trait Describe {
    /// 返回角色的自我介绍文本。
    fn describe(&self) -> String;
}

/// 骑士角色。
/// name：名字，hp：生命值。
pub struct Knight {
    pub name: String,
    pub hp: i32,
}

/// 法师角色。
/// name：名字，mp：魔力值。
pub struct Mage {
    pub name: String,
    pub mp: i32,
}

impl Describe for Knight {
    fn describe(&self) -> String {
        // TODO: 返回 "骑士{name} HP{hp}"
        format!("骑士{} HP{}", self.name, self.hp)
    }
}

impl Describe for Mage {
    fn describe(&self) -> String {
        // TODO: 返回 "法师{name} MP{mp}"
        format!("法师{} MP{}", self.name, self.mp)
    }
}

/// 入口：构造角色并输出情报面板。
/// 返回 (骑士情报, 法师情报)，无需修改。
pub fn exercise_fn() -> (String, String) {
    let knight = Knight {
        name: String::from("阿德"),
        hp: 120,
    };
    let mage = Mage {
        name: String::from("露娜"),
        mp: 80,
    };
    (knight.describe(), mage.describe())
}
