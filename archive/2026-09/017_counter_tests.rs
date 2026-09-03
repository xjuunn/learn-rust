use learn_rust::exercises::current::compute;
use learn_rust::exercises::current::Counter;

#[test]
fn test_compute_result() {
    // 测试说明：验证累计和
    assert_eq!(compute(), 12, "0 + 5 + 10 - 3 = 12");
}

#[test]
fn test_counter_new() {
    // 测试说明：验证 Counter 可创建且初始总数为 0
    let c = Counter::new(0);
    assert_eq!(c.total(), 0, "新建计数器的 total 应为 0");
}

#[test]
fn test_counter_add_and_total() {
    // 测试说明：验证 add 与 total 配合工作
    let mut c = Counter::new(0);
    c.add(7);
    c.add(3);
    c.add(-2);
    assert_eq!(c.total(), 8, "7 + 3 - 2 = 8");
}
