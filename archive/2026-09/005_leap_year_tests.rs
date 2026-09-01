use learn_rust::exercises::current::exercise_fn;

#[test]
fn test_divisible_by_400() {
    // 测试说明：能被 400 整除的年份是闰年
    assert_eq!(exercise_fn(2000), true, "2000 能被 400 整除，应为闰年");
}

#[test]
fn test_divisible_by_100_not_400() {
    // 测试说明：能被 100 整除但不能被 400 整除的不是闰年
    assert_eq!(exercise_fn(1900), false, "1900 能被 100 整除但不能被 400 整除，不是闰年");
}

#[test]
fn test_divisible_by_4_not_100() {
    // 测试说明：能被 4 整除且不能被 100 整除的是闰年
    assert_eq!(exercise_fn(2024), true, "2024 能被 4 整除且不能被 100 整除，应为闰年");
}

#[test]
fn test_normal_year() {
    // 测试说明：普通年份不是闰年
    assert_eq!(exercise_fn(2023), false, "2023 不是闰年");
}
