//! Day 2 最小练习
//!
//! 运行:
//! cargo run --bin day-02
//!
//! 目标:
//! - 先理解语法长什么样
//! - 再自己改字段、改分支、改返回值
//! - 不追求复杂场景，只追求能看懂、能改动
//!

const APP_NAME: &str = "day-02-simple";

// `'a` 只是生命周期参数的常用名字，像泛型里常写 `T` 一样。
// 这里改成 `'b`、`'x`、`'aaa` 都可以，只要前后一致即可。
// 重点不是名字本身，而是“哪些引用绑在同一个生命周期参数上”。
//
// 1. `User<'a>` 这个类型里面含有一个字段 `name: &'a str`。
// 2. 这个字段是借用，不拥有数据。
// 3. 所以 `User<'a>` 的有效使用时间，必须受这个借用约束。
// 4. 换句话说，`User<'a>` 不能活得比它内部那个 `&'a str` 更久。
// 5. 这里必须显式写 `'a`，因为 `struct` 字段里的引用不会套用函数那种“生命周期省略规则”。
// 6. 编译器需要你在类型定义上明确写出：这个借用和哪个生命周期参数绑定。
// 7. 所以这里不能直接写 `name: &str`，必须写成 `name: &'a str`。
// 8. 反例可以这样理解：
//    let user;
//    {
//        let s = String::from("Tom");
//        user = User::new(&s, 20);
//    } // s 在这里销毁
//    println!("{}", user.name);
//    如果允许 `user` 比 `&s` 活得更久，这里就会读到悬垂引用。
#[derive(Debug)]
struct User<'a> {
    // 先声明 `'a`
    name: &'a str, // 再使用 `'a`
    age: u8,
}

impl<'a> User<'a> {
    fn new(name: &'a str, age: u8) -> Self {
        // TODO 1:
        // 把下面这行改成 `Self { name, age }`
        Self { name, age }
    }

    fn intro(&self) {
        println!("name={}, age={}", self.name, self.age);
    }

    fn is_adult(&self) -> bool {
        // TODO 2:
        // 用一个布尔表达式判断是否成年，比如 `self.age >= 18`
        self.age >= 18
    }
}

#[derive(Debug)]
enum Status {
    Todo,
    Doing,
    Done,
}

fn status_text(status: Status) -> &'static str {
    match status {
        // TODO 3:
        // 把这个分支改成返回 `"todo"`
        Status::Todo => "todo",
        Status::Doing => "doing",
        Status::Done => "done",
    }
}

fn maybe_nickname(name: &str) -> Option<&str> {
    // TODO 4:
    // 现在规则是长度 <= 3 时返回 Some(name)，否则返回 None
    if name.len() <= 3 { Some(name) } else { None }
}

fn parse_age(raw: &str) -> Result<u8, &'static str> {
    raw.parse::<u8>().map_err(|_| "age 解析失败")
}

// 这里 left、right、返回值都写成 `'a`，表示它们在同一条借用关系上。
// 如果你写成 `<'aaa>(left: &'aaa str, right: &'aaa str) -> &'aaa str`，语义也一样。
// 如果有两条不同的借用线，才会写成 `'a`、`'b` 这种不同名字。
fn longer_name<'a>(left: &'a str, right: &'a str) -> &'a str {
    if left.len() >= right.len() {
        left
    } else {
        right
    }
}

fn print_day2_checklist() {
    println!();
    println!("== Day 2 checklist ==");
    println!("[ ] 我能看懂 struct 和 impl 的关系");
    println!("[ ] 我能看懂 enum + match 的最基本写法");
    println!("[ ] 我能解释 Option 表示“可能有，也可能没有”");
    println!("[ ] 我能解释 Result 表示“成功或失败”");
    println!("[ ] 我能解释 `longer_name<'a>` 里的 `'a` 在表达什么");
}

fn main() {
    println!("app={APP_NAME}");

    let user = User::new("Tom", 20);
    user.intro();
    println!("adult={}", user.is_adult());

    let status = Status::Doing;
    println!("status={}", status_text(status));

    let nick = maybe_nickname(user.name);
    println!("nickname={nick:?}");

    let parsed = parse_age("x");
    println!("parsed_age={parsed:?}");

    let picked = longer_name("rust", "go");
    println!("longer_name={picked}");

    // 额外可选练习：
    // 1. 把 Status::Doing 改成别的分支
    // 2. 把 parse_age("18") 改成 parse_age("xx")
    // 3. 把 maybe_nickname 的规则改掉
    // 4. 给 User 加一个新字段，再补 new / intro

    let _ = Status::Todo;
    let _ = Status::Done;

    print_day2_checklist();
}
