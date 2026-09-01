mod exercises;

fn main() {
    println!("=== 密码安全检查 ===");
    for pwd in ["Passw0rd", "password", "PASSWORD", "Ab1"] {
        let ok = exercises::current::exercise_fn(pwd);
        println!("{:?}: {}", pwd, if ok { "安全" } else { "不安全" });
    }
}
