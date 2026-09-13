use learn_rust::exercises::current::{exercise_fn, Announce, CourtWizard, TownGuard};

#[test]
fn test_guard_message() {
    // 测试说明：守卫 message 输出格式
    let g = TownGuard {
        name: String::from("Bravo"),
    };
    assert_eq!(g.message(), "Guard Bravo on duty");
}

#[test]
fn test_wizard_message() {
    // 测试说明：法师 message 输出格式
    let w = CourtWizard {
        name: String::from("Elena"),
    };
    assert_eq!(w.message(), "Wizard Elena casting blessings");
}

#[test]
fn test_guard_uses_default_title() {
    // 测试说明：守卫未覆盖 title，返回默认 "visitor"
    let g = TownGuard {
        name: String::from("Bravo"),
    };
    assert_eq!(g.title(), "visitor");
}

#[test]
fn test_wizard_overrides_title() {
    // 测试说明：法师覆盖 title，返回 "Sage"
    let w = CourtWizard {
        name: String::from("Elena"),
    };
    assert_eq!(w.title(), "Sage");
}

#[test]
fn test_exercise_fn_demo() {
    // 测试说明：综合演示返回
    let (msg1, msg2, title1) = exercise_fn();
    assert_eq!(msg1, "Guard Bravo on duty");
    assert_eq!(msg2, "Wizard Elena casting blessings");
    assert_eq!(title1, "visitor");
}