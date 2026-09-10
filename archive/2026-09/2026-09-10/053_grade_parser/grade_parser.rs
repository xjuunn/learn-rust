// ============================================
// 题目编号: ex053
// 知识点: ? 运算符
// 难度: 基础
// 所属章节: 06_错误处理
// ============================================
//
// 场景：一个「班级成绩录入系统」。每行记录格式为 "姓名:分数"，
// 例如 "小红:90"。系统需要把每行解析成 (姓名, 分数)。
//
// 这是 06 错误处理章节的第三题，重点考察 `?` 运算符：
//   - `?` 在 Result 上的作用（Ok 取出值、Err 直接向上传播）
//   - 让错误处理代码更简洁，减少嵌套 match
//
// 本题采用「定义函数」形式：
//   - parse_score 已经帮你写好（parse 失败时返回 Err(String)）
//   - 入口 exercise_fn 已经写好（内部用 match 处理每条记录）
//   - 你需要从 0 实现 parse_record，练习用 `?` 传播错误
//
// 注意：parse_record 中解析分数时必须使用 `?` 运算符，
// 不得用 match 单独处理分数解析错误（这是本题的考察点）。

/// 解析分数部分为 i32。非法输入返回 Err("分数必须是数字：'xxx'")
///
/// 例子：
///   parse_score("90")  -> Ok(90)
///   parse_score(" 88 ") -> Ok(88)
///   parse_score("abc") -> Err("分数必须是数字：'abc'")
pub fn parse_score(s: &str) -> Result<i32, String> {
    s.trim()
        .parse::<i32>()
        .map_err(|_| format!("分数必须是数字：'{}'", s.trim()))
}

/// 解析一整行成绩记录，返回 (姓名, 分数)。
///
/// 规则：
///   1. 用 `split(':')` 分割。若分割后的段数不是恰好 2 段，
///      返回 Err("解析失败：格式应为'姓名:分数'，实际为 '{line}'")
///   2. 姓名取第 0 段并 trim 去空白
///   3. 分数调用 parse_score 解析第 1 段，并用 `?` 让错误向上传播
///
/// 例子：
///   parse_record("小红:90")       -> Ok(("小红", 90))
///   parse_record(" 小刚 : 88 ")   -> Ok(("小刚", 88))
///   parse_record("小红")         -> Err("解析失败：格式应为'姓名:分数'，实际为 '小红'")
///   parse_record("小红:abc")     -> Err("分数必须是数字：'abc'")
///   parse_record("小红:90:81")   -> Err("解析失败：...")
pub fn parse_record(line: &str) -> Result<(String, i32), String> {
    // TODO: 请在此处实现 parse_record（签名不可更改）
    // 提示：
    //   1. let parts: Vec<&str> = line.split(':').collect();
    //   2. 判断 parts.len() != 2 时返回上面的格式错误
    //   3. let name = parts[0].trim();
    //   4. let score = parse_score(parts[1])?;   // 用 ? 传播错误
    //   5. 返回 Ok((name.to_string(), score))
    let parts: Vec<&str> = line.split(':').collect();
    if parts.len() != 2 {
        return Err(format!("解析失败：格式应为'姓名:分数'，实际为 '{line}'"));
    }
    let result = parse_score(parts[1])?;
    Ok((parts[0].trim().to_string(),result))
}

/// 入口函数：解析多行成绩记录，返回每行的处理结果文本。
/// 合法行输出 "姓名：分数 分"，非法行输出 "跳过：错误信息"。
///
/// 例子：
///   exercise_fn(&["小红:90", "小明:abc"]) ->
///     ["小红：90 分", "跳过：分数必须是数字：'abc'"]
pub fn exercise_fn(lines: &[&str]) -> Vec<String> {
    lines
        .iter()
        .map(|&line| match parse_record(line) {
            Ok((name, score)) => format!("{}：{} 分", name, score),
            Err(msg) => format!("跳过：{}", msg),
        })
        .collect()
}