// ============================================
// 题目编号: ex008
// 知识点: 基础语法综合（字符串与循环）
// 难度: 基础
// 所属章节: 01_基础语法
// ============================================

// 题目要求：
// 实现一个函数，接收一个字符串切片 &str 作为密码，判断它是否"安全"。
//
// 安全密码需同时满足：
// 1. 长度至少 8 个字符
// 2. 至少包含一个大写字母
// 3. 至少包含一个小写字母
// 4. 至少包含一个数字（0-9）
//
// 示例：
// 输入："Passw0rd" → 输出：true（长度8，有大小写和数字）
// 输入："password" → 输出：false（没有大写字母）
// 输入："PASSWORD" → 输出：false（没有小写字母）
// 输入："Pass1234" → 输出：false（长度只有8但没有小写字母... 实际有'asse'小写）→ true
//
// 提示：
// - 长度用 .len()（注意处理中文字符时字节数与字符不同，本题按字符数用 .chars().count()）
// - 判断字符类型：c.is_uppercase(), c.is_lowercase(), c.is_ascii_digit()
// - 用多个 bool 标志位记录各类字符是否出现

pub fn exercise_fn(s: &str) -> bool {
    if s.len() < 8 {
        return false;
    }
    let mut has_uppercase = false;
    let mut has_lowercase = false;
    let mut has_number = false;
    for c in s.chars() {
        let c = c as u32;
        if c >= 'a' as u32 && c <= 'z' as u32 {
            has_lowercase = true;
        }
        if c >= 'A' as u32 && c <= 'Z' as u32 {
            has_uppercase = true;
        }
        if c >= '0' as u32 && c<= '9' as u32 {
            has_number = true;
        }
    }
    has_uppercase && has_lowercase && has_number

}
