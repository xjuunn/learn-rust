// 练习模块
// AI 每次出题时更新本文件：
//   1. 用 `pub mod <题目文件名>` 声明当前题目模块
//   2. 用 `pub use <题目名> as current;` 转发为固定名 current

pub mod password;
pub use password as current;
