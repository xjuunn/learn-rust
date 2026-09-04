//! git 提交热力图
//!
//! 读取当前仓库的 git 提交记录，统计每天的提交数量，
//! 并在终端中绘制 GitHub 风格的贡献热力图。

use std::collections::HashMap;
use std::process::Command;

/// 热力等级对应的文本色块（由浅到深，越深代表提交越多）
const LEVEL_BLOCKS: [&str; 5] = [
    "··", // 0 次
    "░░", // 1 次
    "▒▒", // 2 次
    "▓▓", // 3 次
    "██", // 4+ 次
];

/// 热力等级对应的终端字体颜色（256 色），由深到浅
const ANSI_FG: [&str; 5] = [
    "\x1b[38;5;240m", // 0 次：灰
    "\x1b[38;5;22m",  // 1 次：深绿
    "\x1b[38;5;28m",  // 2 次：绿
    "\x1b[38;5;34m",  // 3 次：亮绿
    "\x1b[38;5;46m",  // 4+ 次：最亮绿
];
const RESET: &str = "\x1b[0m";

/// 热力图入口，返回进程退出码（0 表示成功）
/// `mock` 为 true 时使用模拟数据（不动 git），否则读取真实 git 提交记录
pub fn run(mock: bool) -> i32 {
    let daily = if mock {
        mock_data()
    } else {
        match collect_daily_commits() {
            Ok(map) => map,
            Err(msg) => {
                eprintln!("读取 git 提交记录失败: {}", msg);
                return 1;
            }
        }
    };

    if daily.is_empty() {
        println!("当前没有任何可显示的热力图数据。");
        return 0;
    }

    render(&daily);
    0
}

/// 收集所有提交日期并统计每天提交次数
fn collect_daily_commits() -> Result<HashMap<String, usize>, String> {
    let output = Command::new("git")
        .args(["log", "--date=short", "--format=%ad"])
        .output()
        .map_err(|e| format!("无法执行 git 命令: {}", e))?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(format!("git log 执行失败: {}", err.trim()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut daily: HashMap<String, usize> = HashMap::new();
    for line in stdout.lines() {
        let date = line.trim();
        if !date.is_empty() {
            *daily.entry(date.to_string()).or_insert(0) += 1;
        }
    }
    Ok(daily)
}

/// 生成半年的模拟提交数据（2026-03 ~ 2026-08），仅用于预览效果，不改动任何 git 记录。
/// 数据随机但分布合理：工作日提交较多，周末大多空缺或很少，偶有高峰日。
/// 若不足半年（真实数据场景），则只显示已有的日期范围。
fn mock_data() -> HashMap<String, usize> {
    let mut daily: HashMap<String, usize> = HashMap::new();
    let mut rng = MockRng::new(0x5EED_2026);

    // 2026-03 到 2026-08，共 6 个月
    for m in 3..=8 {
        let days_in_month = match m {
            3 | 5 | 7 | 8 => 31,
            4 | 6 => 30,
            _ => unreachable!(),
        };

        for d in 1..=days_in_month {
            let weekday = weekday_of(2026, m, d);
            let key = format!("{:04}-{:02}-{:02}", 2026, m, d);

            let count = match weekday {
                // 周一到周五：工作日活跃
                0..=4 => {
                    if rng.next() % 100 < 15 {
                        0
                    } else {
                        2 + (rng.next() % 6) as usize
                    }
                }
                // 周六：偶尔 1~2 次
                5 => {
                    if rng.next() % 100 < 30 {
                        1 + (rng.next() % 2) as usize
                    } else {
                        0
                    }
                }
                // 周日：几乎都空缺
                _ => {
                    if rng.next() % 100 < 10 {
                        1
                    } else {
                        0
                    }
                }
            };

            if count > 0 {
                daily.insert(key, count);
            }
        }
    }

    daily
}

/// 简单的伪随机数生成器（线性同余），无需外部依赖。
/// 对同一份数据多次生成结果一致，便于复现观察。
struct MockRng {
    state: u64,
}

impl MockRng {
    fn new(seed: u64) -> Self {
        MockRng { state: seed }
    }

    fn next(&mut self) -> u64 {
        // LCG 参数（Numerical Recipes 风格）
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.state >> 33) as u64
    }
}

/// 渲染 GitHub 风格热力图
fn render(daily: &HashMap<String, usize>) {
    let (start, end) = date_range(daily);
    let max_count = daily.values().copied().max().unwrap_or(1).max(1);
    let total: usize = daily.values().sum();

    println!();
    println!("Git 提交热力图");
    println!("范围: {} ~ {}", start, end);
    println!("活跃天数: {} 天, 总提交数: {} 次", daily.len(), total);
    println!();

    // 从 start 到 end 生成连续的每一天
    let mut current = parse_date(&start);
    let end_date = parse_date(&end);
    let mut seq: Vec<(String, usize)> = Vec::new();
    loop {
        let key = fmt_date(current);
        let count = *daily.get(&key).unwrap_or(&0);
        seq.push((key, count));
        if current == end_date {
            break;
        }
        current = current.next_day();
    }

    // 按周分列：每 7 天一列，行依次为 周一..周日
    let start_wd = parse_date(&start).weekday;

    let mut cols: Vec<Vec<(String, usize)>> = Vec::new();
    let mut week: Vec<(String, usize)> = Vec::new();

    // 补足起始周周一之前的空格
    for _ in 0..start_wd {
        week.push((String::new(), 0));
    }
    for item in &seq {
        week.push(item.clone());
        if week.len() == 7 {
            cols.push(std::mem::take(&mut week));
        }
    }
    if !week.is_empty() {
        while week.len() < 7 {
            week.push((String::new(), 0));
        }
        cols.push(week);
    }

    let weekday_names = ["一", "二", "三", "四", "五", "六", "日"];

    // 月份标题行：取每列首个非空日期的月份；同一月只显示一次，跨月才切换
    let mut month_header = String::from("        ");
    let mut prev_month = String::new();
    for col in &cols {
        let month = col
            .iter()
            .find(|(d, _)| !d.is_empty())
            .map(|(d, _)| d[5..7].to_string())
            .unwrap_or_default();
        if month != prev_month && !month.is_empty() {
            month_header.push_str(&format!("{:^3}", month));
        } else {
            month_header.push_str("   ");
        }
        prev_month = month;
    }
    println!("{}", month_header);

    for row in 0..7 {
        let label = format!("周{}", weekday_names[row]);
        let row_str = cols
            .iter()
            .map(|col| {
                if col[row].0.is_empty() {
                    "   ".to_string()
                } else {
                    block(level(col[row].1, max_count))
                }
            })
            .collect::<String>();
        println!("{:5} {}", label, row_str);
    }

    println!();
    println!("色块图例（色块越深代表当天提交越多）:");
    println!(
        "  {} 0 次   {} 1 次   {} 2 次   {} 3 次   {} 4+ 次",
        block(0),
        block(1),
        block(2),
        block(3),
        block(4)
    );
    println!();
}

/// 返回指定等级的色块：字体颜色 + 文本字符 + 颜色重置
fn block(level: usize) -> String {
    let lvl = level.min(4);
    format!("{}{} {}", ANSI_FG[lvl], LEVEL_BLOCKS[lvl], RESET)
}

/// 根据提交数映射 0..=4 的等级
fn level(count: usize, max: usize) -> usize {
    if count == 0 {
        return 0;
    }
    let ratio = count as f64 / max as f64;
    if ratio > 0.75 {
        4
    } else if ratio > 0.5 {
        3
    } else if ratio > 0.25 {
        2
    } else {
        1
    }
}

/// 日期范围（默认显示最近半年窗口）。
/// 起始日期取「最早的提交日期」与「最近提交日期往前推半年」两者中较晚者，
/// 这样当提交总时长远不足半年时，自然只显示从最早的提交开始的全部分布。
fn date_range(daily: &HashMap<String, usize>) -> (String, String) {
    let mut dates: Vec<&String> = daily.keys().collect();
    dates.sort();
    let end = dates.last().unwrap().to_string();
    let earliest = dates.first().unwrap().to_string();

    // 从最近的提交日期往前推 6 个月
    let end_date = parse_date(&end);
    let six_months_ago = month_offset(end_date, -6);
    let start_six = fmt_date(six_months_ago);

    // 起始取更晚的那个日期（保证窗口覆盖所有提交）
    let start = max_date_string(&start_six, &earliest);
    (start, end)
}

/// 日期字符串按字典序比较（YYYY-MM-DD 格式可直接比较），返回较晚的日期
fn max_date_string(a: &str, b: &str) -> String {
    if a >= b { a.to_string() } else { b.to_string() }
}

/// 将日期增加指定的月份偏移量（保持日期不变，若目标月无此日则取该月最后一天）
fn month_offset(d: Date, offset: i32) -> Date {
    let mut m = d.m + offset;
    let mut y = d.y;
    while m > 12 {
        m -= 12;
        y += 1;
    }
    while m < 1 {
        m += 12;
        y -= 1;
    }
    let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let leap = (y % 4 == 0 && y % 100 != 0) || y % 400 == 0;
    let dim = if m == 2 && leap {
        29
    } else {
        days_in_month[(m - 1) as usize]
    };
    let day = d.d.min(dim);
    Date { y, m, d: day, weekday: weekday_of(y, m, day) }
}

fn fmt_date(d: Date) -> String {
    format!("{:04}-{:02}-{:02}", d.y, d.m, d.d)
}

/// 简易日期结构，避免引入 chrono 依赖
#[derive(Clone, Copy, PartialEq)]
struct Date {
    y: i32,
    m: i32,
    d: i32,
    weekday: usize, // 0=周一 ... 6=周日
}

fn parse_date(s: &str) -> Date {
    let parts: Vec<&str> = s.split('-').collect();
    let y: i32 = parts[0].parse().unwrap_or(1970);
    let m: i32 = parts[1].parse().unwrap_or(1);
    let d: i32 = parts[2].parse().unwrap_or(1);
    let weekday = weekday_of(y, m, d);
    Date { y, m, d, weekday }
}

/// 计算给定日期是星期几（0=周一 ... 6=周日）
fn weekday_of(y: i32, m: i32, d: i32) -> usize {
    let days = days_from_civil(y, m, d);
    // 1970-01-01 是星期四；以周日=0 计，星期四=3
    let wd_sun0 = (days + 3).rem_euclid(7) as usize; // 0=周日 ... 6=周六
    (wd_sun0 + 6) % 7 // 转为 0=周一 ... 6=周日
}

/// Howard Hinnant 的 civil date 转 day 数（自 1970-01-01 的天数，星期四对应 0）
fn days_from_civil(y: i32, m: i32, d: i32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y } as i64;
    let era = (if y >= 0 { y } else { y - 399 }) / 400;
    let yoe = y - era * 400;
    let mp = ((m + 9) % 12) as i64;
    let doy = (153 * mp + 2) / 5 + d as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

impl Date {
    fn next_day(mut self) -> Date {
        self.d += 1;
        let days_in_month = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let leap = (self.y % 4 == 0 && self.y % 100 != 0) || self.y % 400 == 0;
        let dim = if self.m == 2 && leap {
            29
        } else {
            days_in_month[(self.m - 1) as usize]
        };
        if self.d > dim {
            self.d = 1;
            self.m += 1;
            if self.m > 12 {
                self.m = 1;
                self.y += 1;
            }
        }
        self.weekday = (self.weekday + 1) % 7;
        self
    }
}
