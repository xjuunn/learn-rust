// ============================================
// 题目编号: ex056
// 知识点: 错误处理综合应用（配置文件解析）
// 难度: 基础
// 所属章节: 06_错误处理
// ============================================
//
// 场景：一个程序启动时要读取配置文件，每行格式 "key=value"。
// 例如 "speed=100"、"verbose=true"、"name=opencode"。
// 配置项的 value 可能是布尔、整数或普通文本，需要逐行判断类型。
//
// 这是 06 错误处理章节的综合应用题，融合本章全部知识：
//   - 自定义错误类型 ConfigError（含 Display / Error 实现）
//   - map_err 把底层解析错误转换成 ConfigError
//   - match 对 Result 的安全解构（不 panic）
//   - 边界情况的处理（空 key / 空 value / 无 '='）
//
// 本题采用「实现/扩展」形式：
//   - parse_line 已给（行格式校验，产出 key 和 value）
//   - 你需要实现 config_bool、config_number、exercise_fn 三处
//
// 别忘了：类型与函数需要 `pub`（集成测试只能访问 pub 项）。

use std::fmt;

/// 配置文件的自定义错误类型
#[derive(Debug)]
pub enum ConfigError {
    /// 行格式错误（无 '=' 或 key/value 为空），携带整行原文
    BadLine(String),
    /// 布尔解析失败，携带非法值原文
    BadBool(String),
    /// 数字解析失败，携带非法值原文
    BadNumber(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::BadLine(line) => write!(f, "格式错误：'{}'", line),
            ConfigError::BadBool(value) => {
                write!(f, "'{}' 不是合法的布尔值（应为 true/false）", value)
            }
            ConfigError::BadNumber(value) => write!(f, "'{}' 不是合法的整数", value),
        }
    }
}

impl std::error::Error for ConfigError {}

/// 拆分一行 "key=value" 为 (key, value)。
/// 无 '=' 或 key/value 任一为空时，返回 Err(ConfigError::BadLine(整行))。
///
/// 例子：
///   parse_line("speed=100") -> Ok(("speed", "100"))
///   parse_line("name=opencode") -> Ok(("name", "opencode"))
///   parse_line("badline") -> Err(BadLine("badline"))
///   parse_line("=100") -> Err(BadLine("=100"))
///   parse_line("key=") -> Err(BadLine("key="))
pub fn parse_line(line: &str) -> Result<(&str, &str), ConfigError> {
    let Some(eq) = line.find('=') else {
        return Err(ConfigError::BadLine(line.to_string()));
    };
    let key = line[..eq].trim();
    let value = line[eq + 1..].trim();
    if key.is_empty() || value.is_empty() {
        return Err(ConfigError::BadLine(line.to_string()));
    }
    Ok((key, value))
}

/// 把 value 解析为 bool。只接受 "true" / "false"（去掉首尾空白后），
/// 其余一律返回 Err(ConfigError::BadBool(value))。
///
/// 例子：
///   config_bool("true")  -> Ok(true)
///   config_bool(" false ") -> Ok(false)
///   config_bool("yes")   -> Err(BadBool("yes"))
pub fn config_bool(value: &str) -> Result<bool, ConfigError> {
    // TODO 1：用 match 匹配 value.trim()，只能是 "true"/"false"
    match value.trim() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(ConfigError::BadBool(value.to_string())),
    }
}

/// 把 value 解析为整数。支持正负号（-5、+7、42）。
/// 非法的返回 Err(ConfigError::BadNumber(value))。
///
/// 例子：
///   config_number("100") -> Ok(100)
///   config_number("-5")  -> Ok(-5)
///   config_number("abc") -> Err(BadNumber("abc"))
pub fn config_number(value: &str) -> Result<i32, ConfigError> {
    // TODO 2：value.trim().parse::<i32>()，失败时 map_err 成 ConfigError::BadNumber
    if let Ok(n) = value.trim().parse::<i32>() {
        return Ok(n);
    } else {
        return Err(ConfigError::BadNumber(value.to_string()));
    };
}

/// 入口函数：解析多行配置文本，返回每行的处理结果。
///
/// 每一行的处理规则：
///   1. parse_line 失败 -> "跳过：{错误信息}"
///   2. 先尝试 config_bool，成功 -> "key: bool 值 {b}"
///   3. 否则尝试 config_number，成功 -> "key: 整数 {n}"
///   4. 都不是 -> "key: 文本 {value}"
///
/// 例子：
///   exercise_fn("speed=100\nverbose=true\nname=opencode\nbadline") ->
///     ["speed: 整数 100",
///      "verbose: bool 值 true",
///      "name: 文本 opencode",
///      "跳过：格式错误：'badline'"]
pub fn exercise_fn(text: &str) -> Vec<String> {
    // TODO 3：遍历 text.lines()，对每行按上述规则处理
    // 提示：用 match parse_line(line) 拆出 (key, value)，
    //       再用 match config_bool(value) / match config_number(value) 判定类型
    let mut list: Vec<String> = Vec::new();
    for item in text.lines() {
        // let (key,value) = parse_line(text)?;
        // let () = match parse_line(text) {
        //     Ok(key,value) => ,
        // };
        let (key, value): (&str, &str) = match parse_line(item) {
            Ok((key, value)) => (key, value),
            Err(_) => {
                list.push(format!("跳过：格式错误：'{item}'"));
                continue;
            },
        };
        match config_bool(value) {
            Ok(b) => {
                list.push(format!("{}: bool 值 {}", key, b));
                continue;
            }
            Err(_) => {}
        }
        match config_number(value) {
            Ok(v) => {
                list.push(format!("{}: 整数 {}", key, v));
                continue;
            }
            Err(_) => {}
        }
        list.push(format!("{}: 文本 {}", key, value));
    }
    list
}
