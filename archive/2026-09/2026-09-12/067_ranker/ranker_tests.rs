use learn_rust::exercises::current::{exercise_fn, Ranker};

#[test]
fn test_add_and_count() {
    // 测试说明：add 登记成绩后 count 返回数量
    let mut r = Ranker::new();
    r.add(10);
    r.add(20);
    r.add(30);
    assert_eq!(r.count(), 3);
}

#[test]
fn test_empty_returns_none() {
    // 测试说明：空榜单 best/worst 返回 None，不 panic
    let r: Ranker<i32> = Ranker::new();
    assert_eq!(r.best(), None);
    assert_eq!(r.worst(), None);
}

#[test]
fn test_best_worst_of_i32() {
    // 测试说明：best/worst 找出最高与最低成绩
    let mut r = Ranker::new();
    r.add(30);
    r.add(10);
    r.add(20);
    assert_eq!(r.best(), Some(30));
    assert_eq!(r.worst(), Some(10));
    assert_eq!(r.count(), 3);
}

#[test]
fn test_best_does_not_remove() {
    // 测试说明：best 返回副本，不影响榜单内的成绩
    let mut r = Ranker::new();
    r.add(7);
    r.add(9);
    assert_eq!(r.best(), Some(9));
    assert_eq!(r.count(), 2);
}

#[test]
fn test_works_with_string() {
    // 测试说明：Ranker<String> 按字典序比较，验证泛型边界适用于多种类型
    let mut r = Ranker::new();
    r.add(String::from("banana"));
    r.add(String::from("apple"));
    r.add(String::from("cherry"));
    assert_eq!(r.best(), Some(String::from("cherry")));
    assert_eq!(r.worst(), Some(String::from("apple")));
}

#[test]
fn test_exercise_fn_demo() {
    // 测试说明：综合演示入口返回（最高, 最低, 数量）
    let (best, worst, count) = exercise_fn();
    assert_eq!(best, 30);
    assert_eq!(worst, 10);
    assert_eq!(count, 3);
}