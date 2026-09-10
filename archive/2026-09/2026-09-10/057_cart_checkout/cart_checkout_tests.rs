use learn_rust::exercises::current::{exercise_fn, item_final, parse_discount, parse_price, CartItem};

fn item(name: &str, price: Option<&str>, discount: Option<&str>) -> CartItem {
    CartItem {
        name: name.to_string(),
        price_text: price.map(|s| s.to_string()),
        discount_text: discount.map(|s| s.to_string()),
    }
}

// 测试说明：验证正常结算（价格 + 折扣）
#[test]
fn test_item_final_with_discount() {
    let it = item("苹果", Some("10"), Some("0.8"));
    let result = item_final(&it);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "苹果 最终价 8.00 元（原价 10.00 元）");
}

// 测试说明：验证无折扣时按原价结算（默认因子应为 1.0）
#[test]
fn test_item_final_no_discount() {
    let it = item("牛奶", Some("15"), None);
    let result = item_final(&it);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "牛奶 最终价 15.00 元（原价 15.00 元）");
}

// 测试说明：验证原价缺失时返回 Err 而非 panic
#[test]
fn test_item_final_missing_price() {
    let it = item("面包", None, Some("0.9"));
    let result = item_final(&it);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("缺价格"));
}

// 测试说明：验证原价非法时返回错误
#[test]
fn test_item_final_bad_price() {
    let it = item("糖果", Some("abc"), None);
    let err = item_final(&it).unwrap_err();
    assert!(err.contains("价格非法"), "实际: {}", err);
}

// 测试说明：验证折扣比例非法（如大于 1）时返回错误
#[test]
fn test_item_final_bad_discount() {
    let it = item("电脑", Some("5000"), Some("2"));
    let err = item_final(&it).unwrap_err();
    assert!(err.contains("折扣比例非法"), "实际: {}", err);
}

// 测试说明：验证 parse_discount 边界（0 和 1 合法）
#[test]
fn test_discount_boundary() {
    assert!(matches!(parse_discount("0"), Ok(0.0)));
    assert!(matches!(parse_discount("1"), Ok(1.0)));
    assert!(parse_discount("1.5").is_err());
}

// 测试说明：验证 parse_price 正常解析
#[test]
fn test_parse_price_ok() {
    assert!(matches!(parse_price("12.5"), Ok(12.5)));
    assert!(parse_price("abc").is_err());
}

// 测试说明：验证 exercise_fn 混合结算且不 panic
#[test]
fn test_exercise_fn_mixed() {
    let items = [
        item("苹果", Some("10"), Some("0.8")),
        item("牛奶", Some("15"), None),
        item("面包", None, Some("0.9")),
        item("糖果", Some("abc"), None),
    ];
    let results = exercise_fn(&items);
    assert_eq!(results[0], "苹果 最终价 8.00 元（原价 10.00 元）");
    assert_eq!(results[1], "牛奶 最终价 15.00 元（原价 15.00 元）");
    assert!(results[2].contains("缺价格"));
    assert!(results[3].contains("价格非法"));
}

// 测试说明：验证 exercise_fn 处理空购物车
#[test]
fn test_exercise_fn_empty() {
    let items: [CartItem; 0] = [];
    assert!(exercise_fn(&items).is_empty());
}