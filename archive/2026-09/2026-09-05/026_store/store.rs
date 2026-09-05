// ============================================
// 题目编号: ex026
// 知识点: 结构体所有权（借用与移动）
// 难度: 基础
// 所属章节: 03_结构体
// ============================================

// 题目要求：
// 铁匠铺里售卖武器。请你从 0 定义一个商品结构体 `Item`，并实现方法，
// 重点体会结构体方法的借用规则（&self / &mut self / 多个借用）。
//
// 1. 定义结构体 `Item`（声明为 pub，字段也都为 pub）：
//    - 字段 name: String（商品名）
//    - 字段 price: u32（单价，金币）
//    - 字段 stock: u32（库存数量）
//

pub struct Item {
    pub name: String,
    pub price: u32,
    pub stock: u32,
}
// 2. 为 Item 实现如下项（均为 pub）：
//    - `fn new(name: &str, price: u32, stock: u32) -> Self`：关联函数，
//      内部用 name.to_string() 把 &str 转成 String 再存入
//    - `fn describe(&self) -> String`：返回格式 "名称 x库存 每件price金币"，
//      例如 ("剑", 100, 3) → "剑 x3 每件100金币"
//    - `fn sell(&mut self, count: u32) -> bool`：卖出 count 件，
//      库存足够则扣减库存并返回 true；不够则库存不变，返回 false
//    - `fn total_value(&self) -> u32`：返回库存总价值 = price * stock
//    - `fn cheaper(a: &Item, b: &Item) -> &Item`：自由函数（放在 impl 外），
//      返回单价较低的商品的引用；价格相同返回 a
//

pub fn cheaper<'a>(a: &'a Item, b: &'a Item) -> &'a Item {
    if a.price <= b.price { a } else { b }
}

impl Item {
    pub fn new(name: &str, price: u32, stock: u32) -> Self {
        Self {
            name: name.to_string(),
            price,
            stock,
        }
    }

    pub fn describe(&self) -> String {
        format!("{} x{} 每件{}金币", self.name, self.stock, self.price).to_string()
    }

    pub fn sell(&mut self, count: u32) -> bool {
        if self.stock >= count {
            self.stock -= count;
            true
        } else {
            false
        }
    }

    pub fn total_value(&self) -> u32 {
        self.stock * self.price
    }
}
// 3. 实现入口函数 exercise_fn -> (String, u32)：
//    - 创建一个可变商品："剑"，单价 100，库存 3
//    - 创建一个商品："盾"，单价 80，库存 5
//    - 卖出 1 把剑
//    - 用 cheaper 比较两件商品，取出较便宜商品的名字（注意：取 name 需用
//      .clone() 或格式化为新 String，否则会把字段移动出借用）
//    - 返回 (较便宜商品名, 两件商品当前库存总价值之和)
//
// 示例：
// let mut a = Item::new("剑", 100, 3);   // 卖出后：价格100，库存2
// let b = Item::new("盾", 80, 5);
// cheaper 比较 → 盾更便宜（80 < 100）
// 总价值 = 100*2 + 80*5 = 600
// 期望返回: ("盾", 600)
//
// 提示：
// - describe / total_value 只读，用 &self；sell 要改 stock，必须用 &mut self
// - cheaper 同时借用两个 Item，返回生命周期较短的引用于两个参数都是合法的
// - 所有类型与方法都要 pub，否则集成测试无法访问（报 E0603 私有项错误）
// - new 里记得 name.to_string()，不要直接存 &str

// TODO: 在此定义 Item 结构体、impl 及其方法，并实现下面的入口函数
pub fn exercise_fn() -> (String, u32) {
    // TODO: 按题目要求创建商品、卖出、比较并返回结果
    let mut a = Item::new("剑", 100, 3);
    let b = Item::new("盾", 80, 5);
    a.sell(1);
    let c = cheaper(&a, &b);
    (c.name.clone(), b.total_value() + a.total_value())
}
