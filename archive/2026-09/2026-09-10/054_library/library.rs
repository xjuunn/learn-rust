// ============================================
// 题目编号: ex054
// 知识点: 自定义错误类型
// 难度: 基础
// 所属章节: 06_错误处理
// ============================================
//
// 场景：一个「图书馆借阅系统」。读者尝试借书，可能遇到两种失败：
//   1. 书不存在                  -> BookNotFound("书名")
//   2. 书存在但已被借走          -> BookBorrowed("书名")
// 这两种失败用一个自定义错误类型 LibraryError 表达。
//
// 这是 06 错误处理章节的第四题，重点考察自定义错误类型：
//   - 定义错误枚举（变体携带数据）
//   - 为错误实现 Display（给用户的可读信息）
//   - 为错误实现 std::error::Error trait
//   - 把自定义错误用作 Result 的错误类型
//
// 本题采用「定义类型」形式：请完成下面 4 个任务。
//
// 重要提示：测试（集成测试 crate）只能访问 `pub` 项，
// 所以 LibraryError 及其变体、borrow_book、exercise_fn
// 都必须保持 `pub`（枚举在 pub 时，变体默认可访问）。

use std::fmt;

/// 图书馆的自定义错误类型。请补全这个枚举：
///   变体 BookNotFound(String)：书不存在，携带书名
///   变体 BookBorrowed(String)：书已被借出，携带书名
#[derive(Debug)]
pub enum LibraryError {
    // TODO 任务 1：定义两个变体（携带书名 String）
    // 示例：BookNotFound(String)
    BookNotFound(String),
    BookBorrowed(String),
}

/// 任务 2：为 LibraryError 实现 Display。
/// 输出格式必须严格匹配下方注释：
///   BookNotFound(name) -> "图书馆里没有《{name}》"
///   BookBorrowed(name) -> "《{name}》已被借出"
///
/// 例：LibraryError::BookNotFound("三体") 显示为 "图书馆里没有《三体》"
impl fmt::Display for LibraryError {
    fn fmt(&self, mut f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: 用 match 匹配两个变体，write!(f, ...)
        match self {
            Self::BookBorrowed(name) => write!(&mut f,"《{name}》已被借出"),
            Self::BookNotFound(name)  => write!(&mut f, "图书馆里没有《{name}》"),
        }
    }
}

/// 任务 3：让 LibraryError 成为标准错误（实现 std::error::Error）。
/// 只要一行：impl std::error::Error for LibraryError {}
impl std::error::Error for LibraryError {}

/// 任务 4：实现借书逻辑。
///   1. 书不在 catalog 中 -> 返回 Err(LibraryError::BookNotFound(书名))
///   2. 书在 borrowed 中  -> 返回 Err(LibraryError::BookBorrowed(书名))
///   3. 否则 Ok(())
pub fn borrow_book(catalog: &[&str], borrowed: &[&str], book: &str) -> Result<(), LibraryError> {
    // TODO: 实现借书逻辑
    if !catalog.contains(&book) {
        return Err(LibraryError::BookNotFound(book.to_string()))
    }
    if borrowed.contains(&book) {
        return Err(LibraryError::BookBorrowed(book.to_string()));
    }
    Ok(())
}

/// 入口函数：依次尝试借阅多本书，返回结果文本。
/// 成功输出 "成功借到《{book}》"，失败输出 "借阅失败：{错误}"。
///
/// 例子：
///   exercise_fn(&["活着", "小王子"], &["小王子"], &["活着", "小王子", "三体"]) ->
///     ["成功借到《活着》",
///      "借阅失败：《小王子》已被借出",
///      "借阅失败：图书馆里没有《三体》"]
pub fn exercise_fn(catalog: &[&str], borrowed: &[&str], books: &[&str]) -> Vec<String> {
    books
        .iter()
        .map(|&book| match borrow_book(catalog, borrowed, book) {
            Ok(()) => format!("成功借到《{}》", book),
            Err(err) => format!("借阅失败：{}", err),
        })
        .collect()
}
