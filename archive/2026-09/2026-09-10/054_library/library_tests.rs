use learn_rust::exercises::current::{borrow_book, exercise_fn, LibraryError};

// 测试说明：验证 BookNotFound 的 Display 输出格式
#[test]
fn test_display_not_found() {
    let err = LibraryError::BookNotFound("三体".to_string());
    assert_eq!(err.to_string(), "图书馆里没有《三体》");
}

// 测试说明：验证 BookBorrowed 的 Display 输出格式
#[test]
fn test_display_borrowed() {
    let err = LibraryError::BookBorrowed("小王子".to_string());
    assert_eq!(err.to_string(), "《小王子》已被借出");
}

// 测试说明：验证成功借到未借出的书
#[test]
fn test_borrow_success() {
    let catalog = ["活着", "小王子", "百年孤独"];
    let borrowed = ["小王子"];
    assert!(borrow_book(&catalog, &borrowed, "活着").is_ok());
}

// 测试说明：验证借不存在的书返回 BookNotFound
#[test]
fn test_borrow_not_found() {
    let catalog = ["活着"];
    let borrowed: [&str; 0] = [];
    assert!(matches!(
        borrow_book(&catalog, &borrowed, "三体"),
        Err(LibraryError::BookNotFound(_))
    ));
}

// 测试说明：验证借已被借出的书返回 BookBorrowed
#[test]
fn test_borrow_borrowed() {
    let catalog = ["活着", "小王子"];
    let borrowed = ["小王子"];
    assert!(matches!(
        borrow_book(&catalog, &borrowed, "小王子"),
        Err(LibraryError::BookBorrowed(_))
    ));
}

// 测试说明：验证 LibraryError 实现了 std::error::Error（可装箱为 Box<dyn Error>）
#[test]
fn test_error_trait() {
    fn as_dyn_error(e: &dyn std::error::Error) -> String {
        e.to_string()
    }
    let err = LibraryError::BookNotFound("三体".to_string());
    assert_eq!(as_dyn_error(&err), "图书馆里没有《三体》");
}

// 测试说明：验证 exercise_fn 成功、已借出、不存在三种情况
#[test]
fn test_exercise_fn_mixed() {
    let catalog = ["活着", "小王子"];
    let borrowed = ["小王子"];
    let results = exercise_fn(&catalog, &borrowed, &["活着", "小王子", "三体"]);
    assert_eq!(results[0], "成功借到《活着》");
    assert_eq!(results[1], "借阅失败：《小王子》已被借出");
    assert_eq!(results[2], "借阅失败：图书馆里没有《三体》");
}

// 测试说明：验证 exercise_fn 处理空借阅列表
#[test]
fn test_exercise_fn_empty() {
    let catalog = ["活着"];
    let books: [&str; 0] = [];
    assert!(exercise_fn(&catalog, &[], &books).is_empty());
}