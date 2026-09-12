use learn_rust::exercises::current::{exercise_fn, Vault};

#[test]
fn test_new_and_peek() {
    // 测试说明：new 放入宝物后 peek 应返回该宝物引用，开箱次数为 0
    let v = Vault::new(42);
    assert_eq!(*v.peek(), 42);
    assert_eq!(v.open_count(), 0);
}

#[test]
fn test_deposit_overwrites_and_counts() {
    // 测试说明：deposit 覆盖旧宝物，并逐次累计开箱次数
    let mut v = Vault::new(String::from("金币"));
    v.deposit(String::from("宝石"));
    v.deposit(String::from("圣剑"));
    assert_eq!(v.peek(), "圣剑");
    assert_eq!(v.open_count(), 2);
}

#[test]
fn test_transform_changes_type() {
    // 测试说明：泛型方法 transform 能把 Vault<String> 变换为 Vault<usize>
    let v = Vault::new(String::from("宝石"));
    let len_vault = v.transform(|s| s.chars().count());
    assert_eq!(*len_vault.peek(), 2);
}

#[test]
fn test_transform_transfers_count() {
    // 测试说明：deposit +1、transform 调用 +1，次数随箱转移到新箱子
    let mut v = Vault::new(10);
    v.deposit(20);
    let w = v.transform(|x| x * 2);
    assert_eq!(*w.peek(), 40);
    assert_eq!(w.open_count(), 2);
}

#[test]
fn test_exercise_fn_demo() {
    // 测试说明：综合演示入口返回（最终宝物, 开箱次数, 变换后评级）
    let (name, count, rating) = exercise_fn();
    assert_eq!(name, "宝石");
    assert_eq!(count, 2);
    assert_eq!(rating, 2);
}