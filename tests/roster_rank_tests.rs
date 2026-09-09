use learn_rust::exercises::current::{exercise_fn, ranking, Vote};

// 测试说明：验证按票数从高到低排序返回名字
#[test]
fn test_ranking_order() {
    let entries = &[
        Vote { name: "小明".into(), votes: 10 },
        Vote { name: "小红".into(), votes: 30 },
        Vote { name: "小刚".into(), votes: 20 },
    ];
    let result = ranking(entries);
    assert_eq!(result, vec!["小红", "小刚", "小明"]);
}

// 测试说明：验证空榜单返回空列表
#[test]
fn test_ranking_empty() {
    assert!(ranking(&[]).is_empty(), "空榜单应返回空列表");
}

// 测试说明：验证每个参赛者只出现一次（名字唯一）
#[test]
fn test_ranking_all_names() {
    let entries = &[
        Vote { name: "a".into(), votes: 1 },
        Vote { name: "b".into(), votes: 2 },
        Vote { name: "c".into(), votes: 3 },
    ];
    let result = ranking(entries);
    assert_eq!(result.len(), 3);
    let mut copy = result.clone();
    copy.sort();
    assert_eq!(copy, vec!["a", "b", "c"]);
}

// 测试说明：验证 exercise_fn 首行是冠军
#[test]
fn test_champion_line() {
    let entries = &[
        Vote { name: "小明".into(), votes: 10 },
        Vote { name: "小红".into(), votes: 30 },
        Vote { name: "小刚".into(), votes: 20 },
    ];
    let result = exercise_fn(entries);
    assert_eq!(result[0], "冠军: 小红");
}

// 测试说明：验证榜单每行格式与票数、名次
#[test]
fn test_board_format() {
    let entries = &[
        Vote { name: "小明".into(), votes: 10 },
        Vote { name: "小红".into(), votes: 30 },
        Vote { name: "小刚".into(), votes: 20 },
    ];
    let result = exercise_fn(entries);
    assert_eq!(result, vec![
        "冠军: 小红",
        "1. 小红（30 票）",
        "2. 小刚（20 票）",
        "3. 小明（10 票）",
    ]);
}

// 测试说明：验证单人时只有冠军 + 一行榜单
#[test]
fn test_single_entry() {
    let entries = &[Vote { name: "独苗".into(), votes: 5 }];
    let result = exercise_fn(entries);
    assert_eq!(result, vec!["冠军: 独苗", "1. 独苗（5 票）"]);
}