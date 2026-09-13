// ============================================
// 题目编号: ex072
// 知识点: Trait 默认方法与覆盖
// 难度: 基础
// 所属章节: 08_Trait
// ============================================
//
// 上一题你为两个类型实现了同一个 Trait。本题进一步体验 **默认方法**：
// Trait 里的方法可以带有默认实现，实现它的类型可以选择"用默认"，
// 也可以自己覆盖——就像每种公告都有固定台词，但有人要换词也没问题。
//
// 场景：小镇公告广场——
//   - 守卫发出巡逻通报
//   - 法官法师发出祝福公告
//   两者都有 message() 内容，也都有 title() 用来标注身份。
//   title() 有默认值 "visitor"，法师选择覆盖为 "Sage"，守卫用默认。
//
// 任务（结构体与 trait 定义已给出）：
//
//   1. 为 TownGuard 实现 Announce：
//      - message() 返回 "Guard {name} on duty"
//      - 不覆盖 title()，沿用默认 "visitor"
//
//   2. 为 CourtWizard 实现 Announce：
//      - message() 返回 "Wizard {name} casting blessings"
//      - 覆盖 title() 返回 "Sage"
//
//   3. exercise_fn 已写好：构造 Bravo(Elena)，
//      返回 (message1, message2, title1.to_string())
//
// 例子：
//   g.title() == "visitor"（默认） ；w.title() == "Sage"（覆盖）
//
// 提示：
//   - 不覆盖默认方法时，直接不写即可；写出来也不算错，但本题要求不写
//   - 覆盖时在 impl 块里重新定义对应方法即可
//   - 所有 pub 字段/结构体/trait 保持 pub，集成测试才能访问
pub struct TownGuard {
    pub name: String,
}

pub struct CourtWizard {
    pub name: String,
}

pub trait Announce {
    /// 公告内容。
    fn message(&self) -> String;

    /// 发言人身份标签，拥有默认实现。
    /// 默认返回 "visitor"，CourtWizard 覆盖为 "Sage"。
    fn title(&self) -> &str {
        "visitor"
    }
}

impl Announce for TownGuard {
    fn message(&self) -> String {
        // TODO: 返回 "Guard {name} on duty"
        format!("Guard {} on duty", self.name)
    }
}

impl Announce for CourtWizard {
    fn message(&self) -> String {
        // TODO: 返回 "Wizard {name} casting blessings"
        format!("Wizard {} casting blessings", self.name)
    }

    fn title(&self) -> &str {
        // TODO: 覆盖为 "Sage"
        "Sage"
    }
}

/// 入口：构造公告角色并输出。
/// 返回 (message1, message2, title1.to_string())。
pub fn exercise_fn() -> (String, String, String) {
    let guard = TownGuard {
        name: String::from("Bravo"),
    };
    let wizard = CourtWizard {
        name: String::from("Elena"),
    };
    (guard.message(), wizard.message(), guard.title().to_string())
}
