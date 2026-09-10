use learn_rust::exercises::current::{exercise_fn, safe_access};

// 测试说明：验证正常访问数组元素
#[test]
fn test_normal_access() {
    let arr = [10, 20, 30];
    assert_eq!(safe_access(&arr, 0), 10);
    assert_eq!(safe_access(&arr, 1), 20);
    assert_eq!(safe_access(&arr, 2), 30);
}

// 测试说明：验证索引越界时 panic
#[test]
#[should_panic(expected = "索引 5 越界，数组长度为 3")]
fn test_out_of_bounds_panic() {
    let arr = [10, 20, 30];
    safe_access(&arr, 5);
}

// 测试说明：验证索引等于数组长度时 panic
#[test]
#[should_panic(expected = "索引 3 越界，数组长度为 3")]
fn test_boundary_panic() {
    let arr = [10, 20, 30];
    safe_access(&arr, 3);
}

// 测试说明：验证 exercise_fn 正常返回结果
#[test]
fn test_exercise_fn_normal() {
    let arr = [10, 20, 30, 40, 50];
    let indices = [0, 2, 4];
    let result = exercise_fn(&arr, &indices);
    assert_eq!(result, vec![10, 30, 50]);
}

// 测试说明：验证 exercise_fn 处理空索引列表
#[test]
fn test_exercise_fn_empty_indices() {
    let arr = [10, 20, 30];
    let indices: [usize; 0] = [];
    let result = exercise_fn(&arr, &indices);
    assert!(result.is_empty());
}

// 测试说明：验证 exercise_fn 遇到越界索引时 panic
#[test]
#[should_panic(expected = "索引 10 越界，数组长度为 3")]
fn test_exercise_fn_out_of_bounds() {
    let arr = [10, 20, 30];
    let indices = [0, 10];
    exercise_fn(&arr, &indices);
}
