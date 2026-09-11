use learn_rust::exercises::current::{exercise_fn, place_order, Dish, OrderError, OrderItem};

fn dish(name: &str, price: f64) -> Dish {
    Dish { name: name.to_string(), price }
}

fn item(name: &str, count: u32) -> OrderItem {
    OrderItem { name: name.to_string(), count }
}

// 测试说明：验证正常结账计算总价
#[test]
fn test_place_order_ok() {
    let menu = [dish("宫保鸡丁", 28.0), dish("米饭", 3.0)];
    let items = [item("宫保鸡丁", 2), item("米饭", 3)];
    let result = place_order(&menu, &items);
    assert!(result.is_ok());
    assert!((result.unwrap() - 65.0).abs() < 1e-9);
}

// 测试说明：验证菜品不在菜单时返回 DishNotFound
#[test]
fn test_place_order_dish_not_found() {
    let menu = [dish("宫保鸡丁", 28.0)];
    let items = [item("红烧肉", 1)];
    assert_eq!(
        place_order(&menu, &items),
        Err(OrderError::DishNotFound("红烧肉".to_string()))
    );
}

// 测试说明：验证数量为 0 时返回 InvalidCount（边界情况）
#[test]
fn test_place_order_invalid_count() {
    let menu = [dish("宫保鸡丁", 28.0)];
    let items = [item("宫保鸡丁", 0)];
    assert_eq!(place_order(&menu, &items), Err(OrderError::InvalidCount(0)));
}

// 测试说明：验证第一个错误立即返回整体失败（后续项不再影响）
#[test]
fn test_place_order_first_error_wins() {
    let menu = [dish("宫保鸡丁", 28.0)];
    let items = [item("红烧肉", 2), item("宫保鸡丁", 1)];
    assert!(matches!(
        place_order(&menu, &items),
        Err(OrderError::DishNotFound(_))
    ));
}

// 测试说明：验证空菜单项订单总价为 0（边界情况）
#[test]
fn test_place_order_empty_items() {
    let menu = [dish("宫保鸡丁", 28.0)];
    let items: [OrderItem; 0] = [];
    let result = place_order(&menu, &items);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0.0);
}

// 测试说明：验证 OrderError 的 Display 输出
#[test]
fn test_error_display() {
    assert_eq!(
        OrderError::DishNotFound("红烧肉".to_string()).to_string(),
        "菜单里没有《红烧肉》"
    );
    assert_eq!(OrderError::InvalidCount(0).to_string(), "数量必须大于 0，当前为 0");
}

// 测试说明：验证 exercise_fn 混合成功与失败的订单
#[test]
fn test_exercise_fn_mixed() {
    let menu = [dish("宫保鸡丁", 28.0), dish("米饭", 3.0)];
    let orders: Vec<Vec<OrderItem>> = vec![
        vec![item("宫保鸡丁", 2), item("米饭", 3)],
        vec![item("红烧肉", 1)],
        vec![item("宫保鸡丁", 0)],
    ];
    let refs: Vec<&[OrderItem]> = orders.iter().map(|v| v.as_slice()).collect();
    let results = exercise_fn(&menu, &refs);
    assert_eq!(results[0], "订单合计 65.00 元");
    assert!(results[1].contains("菜单里没有《红烧肉》"));
    assert!(results[2].contains("数量必须大于 0"));
}

// 测试说明：验证 exercise_fn 处理空订单列表（边界情况）
#[test]
fn test_exercise_fn_empty() {
    let menu = [dish("宫保鸡丁", 28.0)];
    let orders: [&[OrderItem]; 0] = [];
    assert!(exercise_fn(&menu, &orders).is_empty());
}

// 测试说明：验证 exercise_fn 单份订单正确输出金额格式
#[test]
fn test_exercise_fn_single() {
    let menu = [dish("米饭", 3.0)];
    let orders = [&[item("米饭", 1)][..]];
    let results = exercise_fn(&menu, &orders);
    assert_eq!(results, vec!["订单合计 3.00 元"]);
}