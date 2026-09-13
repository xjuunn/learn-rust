mod exercises;

fn main() {
    println!("=== 小镇公告广场 ===");
    let (msg1, msg2, title1) = exercises::current::exercise_fn();
    println!("守卫公告: {}", msg1);
    println!("法师公告: {}", msg2);
    println!("守卫生份标签: {}", title1);
    let sage = exercises::current::CourtWizard {
        name: String::from("Elena"),
    };
    println!("法师覆盖标签: {}", exercises::current::Announce::title(&sage));
    println!("默认方法不覆盖则沿用，覆盖则按自定义");
}