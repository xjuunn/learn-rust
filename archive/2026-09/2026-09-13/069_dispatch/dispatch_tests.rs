use learn_rust::exercises::current::{count_of, dedup, exercise_fn, highest};

#[test]
fn test_highest_normal() {
    // 测试说明：highest 返回最大元素的引用（此处为字典序最大）
    let items = vec!["battle", "patrol", "rescue"];
    assert_eq!(highest(&items), Some(&"rescue"));
}

#[test]
fn test_highest_empty() {
    // 测试说明：空列表返回 None
    let items: Vec<i32> = vec![];
    assert_eq!(highest(&items), None);
}

#[test]
fn test_highest_single() {
    // 测试说明：单元素列表返回该元素本身
    let items = vec![42];
    assert_eq!(highest(&items), Some(&42));
}

#[test]
fn test_count_of_common() {
    // 测试说明：统计元素出现次数
    let items = vec!["battle", "patrol", "battle"];
    assert_eq!(count_of(&items, &"battle"), 2);
}

#[test]
fn test_count_of_absent() {
    // 测试说明：不存在的元素计数为 0
    let items = vec![1, 2, 3];
    assert_eq!(count_of(&items, &9), 0);
}

#[test]
fn test_dedup_sorts_and_removes() {
    // 测试说明：去重后的列表升序排列
    assert_eq!(dedup(&[3, 1, 2, 3, 1, 2]), vec![1, 2, 3]);
}

#[test]
fn test_dedup_all_same() {
    // 测试说明：全部相同元素只剩一个
    assert_eq!(dedup(&[5, 5, 5]), vec![5]);
}

#[test]
fn test_dedup_empty() {
    // 测试说明：空列表返回空
    let items: Vec<i32> = vec![];
    assert_eq!(dedup(&items), Vec::<i32>::new());
}

#[test]
fn test_exercise_fn_demo() {
    // 测试说明：综合演示入口返回（最高任务, battle 次数, 去重列表）
    let (top, count, codes) = exercise_fn();
    assert_eq!(top, "rescue");
    assert_eq!(count, 2);
    assert_eq!(codes, vec![1, 2, 3]);
}