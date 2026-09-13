use learn_rust::exercises::current::{exercise_fn, Duo};

#[test]
fn test_new_access_fields() {
    // 测试说明：new 构造后两个不同类型字段可直接访问
    let d = Duo::new(7, String::from("代号甲"));
    assert_eq!(d.first, 7);
    assert_eq!(d.second, "代号甲");
}

#[test]
fn test_swap_swaps_types() {
    // 测试说明：swap 交换值的同时交换类型参数位置（Duo<A,B> -> Duo<B,A>）
    let d = Duo::new(String::from("秘文"), 42);
    let swapped = d.swap();
    assert_eq!(swapped.first, 42);
    assert_eq!(swapped.second, "秘文");
}

#[test]
fn test_map_first_changes_first_type() {
    // 测试说明：map_first 只加工 first 类型，second 原样保留
    let d = Duo::new(3, "key");
    let mapped = d.map_first(|n| n * 100);
    assert_eq!(mapped.first, 300);
    assert_eq!(mapped.second, "key");
}

#[test]
fn test_map_second_changes_second_type() {
    // 测试说明：map_second 只加工 second 类型，first 原样保留
    let d = Duo::new("msg", 5);
    let mapped = d.map_second(|k| format!("密钥{}", k));
    assert_eq!(mapped.first, "msg");
    assert_eq!(mapped.second, "密钥5");
}

#[test]
fn test_chain_swap_and_map() {
    // 测试说明：swap 与 map 可任意串联组合
    let d = Duo::new(10, 2.5);
    let r = d.map_first(|n| n as f64).swap(); // Duo<f64,i32> -> map_first -> Duo<f64,f64> -> swap -> Duo<f64,f64>
    assert_eq!(r.first, 2.5);
    assert_eq!(r.second, 10.0);
}

#[test]
fn test_exercise_fn_demo() {
    // 测试说明：综合演示入口返回（密文数, 加工后文案, 密文字符数）
    let (digits, slogan, len) = exercise_fn();
    assert_eq!(digits, 5);
    assert_eq!(slogan, "精锐剑士");
    assert_eq!(len, 2);
}