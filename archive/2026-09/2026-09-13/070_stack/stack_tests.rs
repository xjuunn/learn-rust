use learn_rust::exercises::current::{exercise_fn, Stack};

#[test]
fn test_new_is_empty() {
    // 测试说明：新栈为空且长度为 0
    let s = Stack::<i32>::new();
    assert!(s.is_empty());
    assert_eq!(s.len(), 0);
}

#[test]
fn test_push_len_and_lifo() {
    // 测试说明：入栈后长度正确，出栈按后进先出
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);
    assert_eq!(s.len(), 3);
    assert_eq!(s.pop(), Some(3));
    assert_eq!(s.pop(), Some(2));
    assert_eq!(s.pop(), Some(1));
    assert!(s.is_empty());
}

#[test]
fn test_pop_empty_returns_none() {
    // 测试说明：空栈出栈返回 None
    let mut s = Stack::<i32>::new();
    assert_eq!(s.pop(), None);
}

#[test]
fn test_peek_borrows_without_remove() {
    // 测试说明：peek 借出栈顶但不移除，随后 pop 仍可取到该元素
    let mut s = Stack::new();
    s.push(7);
    assert_eq!(s.peek(), Some(&7));
    assert_eq!(s.len(), 1);
    assert_eq!(s.pop(), Some(7));
}

#[test]
fn test_dup_copies_independent() {
    // 测试说明：dup 返回深拷贝，原栈变化不影响副本
    let mut s = Stack::new();
    s.push(5);
    s.push(8);
    let copy = s.dup();
    s.pop();
    assert_eq!(copy.len(), 2);
    assert_eq!(copy.peek(), Some(&8));
    assert_eq!(s.len(), 1);
}

#[test]
fn test_peak_empty_is_none() {
    // 测试说明：空栈 peek 返回 None
    let s = Stack::<String>::new();
    assert_eq!(s.peek(), None);
}

#[test]
fn test_exercise_fn_demo() {
    // 测试说明：综合演示（peek 9 / pop 9 / 副本长度 2 / 未空）
    let (top, popped, copy_len, is_empty) = exercise_fn();
    assert_eq!(top, Some(9));
    assert_eq!(popped, Some(9));
    assert_eq!(copy_len, 2);
    assert!(!is_empty);
}