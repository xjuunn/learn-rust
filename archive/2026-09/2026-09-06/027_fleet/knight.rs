// 1. 新建文件 `src/exercises/knight.rs`，在其中定义 `Knight` 结构体（全部 pub）：
//    - 顶部加 `#[derive(Clone, Debug)]`（克隆 + 调试打印）
//    - 字段 name: String（姓名）、hp: u32（生命）、atk: u32（攻击）
//    - 关联函数 `pub fn new(name: &str, hp: u32, atk: u32) -> Self`
//    - 方法 `pub fn take_damage(&mut self, dmg: u32)`：生命减少 dmg，最低为 0

#[derive(Clone, Debug)]
pub struct Knight {
    pub name: String,
    pub hp: u32,
    pub atk: u32,
}

impl Knight {
    pub fn new(name: &str, hp: u32, atk: u32) -> Self {
        Self {
            name: name.to_string(),
            hp,
            atk,
        }
    }

    pub fn take_damage(&mut self, dmg: u32) {
        if self.hp < dmg {
            self.hp = 0
        } else {
            self.hp -= dmg;
        }
    }
}
