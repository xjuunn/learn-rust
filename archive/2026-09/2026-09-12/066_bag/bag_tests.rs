use learn_rust::exercises::current::{exercise_fn, Bag};

#[test]
fn test_push_and_len() {
    // 测试说明：push 放入物品后 len 返回件数
    let mut bag = Bag::new();
    bag.push(1);
    bag.push(2);
    bag.push(3);
    assert_eq!(bag.len(), 3);
}

#[test]
fn test_pop_lifo_order() {
    // 测试说明：pop 按后进先出取出，并从背包移除
    let mut bag = Bag::new();
    bag.push(String::from("药水"));
    bag.push(String::from("地图"));
    assert_eq!(bag.pop(), Some(String::from("地图")));
    assert_eq!(bag.len(), 1);
    assert_eq!(bag.pop(), Some(String::from("药水")));
    assert_eq!(bag.len(), 0);
}

#[test]
fn test_pop_empty_returns_none() {
    // 测试说明：空背包 pop 返回 None，不 panic
    let mut bag: Bag<i32> = Bag::new();
    assert_eq!(bag.pop(), None);
}

#[test]
fn test_peek_last_borrows() {
    // 测试说明：peek_last 借引用偷看最后一件，不改变背包状态
    let mut bag = Bag::new();
    bag.push(7);
    assert_eq!(bag.peek_last(), Some(&7));
    assert_eq!(bag.len(), 1);
}

#[test]
fn test_map_changes_item_type() {
    // 测试说明：泛型方法 map 把 Bag<i32> 变为 Bag<String>，顺序不变
    let mut bag = Bag::new();
    bag.push(1);
    bag.push(2);
    bag.push(3);
    let formatted = bag.map(|x| format!("第{}件", x));
    assert_eq!(formatted.len(), 3);
    assert_eq!(formatted.peek_last(), Some(&String::from("第3件")));
}

#[test]
fn test_exercise_fn_demo() {
    // 测试说明：综合演示入口返回（取出值, 剩余件数, map后最后一件）
    let (popped, left, last) = exercise_fn();
    assert_eq!(popped, 30);
    assert_eq!(left, 2);
    assert_eq!(last, 200);
}