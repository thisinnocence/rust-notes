//! 19_demo: 通过一个 CLI 小练习串起基础业务逻辑（CRUD + 查询 + 排序）。
//!
//! 运行：
//! cargo run --bin 19_demo
//!
//! 命令：
//! - add <name> <age> <class>
//! - list
//! - remove <id>
//! - mod <id> <name> <age> <class>
//! - search id <id>
//! - search name <name>
//! - order <id|name|age|class> <asc|desc>
//! - help
//! - quit / exit

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct Student {
    id: u32,
    name: String,
    age: u8,
    class_name: String,
}

#[derive(Debug, Default)]
// 存储设计总览：
// - 主数据只放在 `by_id`（HashMap）里，这是唯一完整记录存储。
// - `ids`/`name_index` 是轻量索引层，只保存 id（主键）来加速查询/排序。
// - 可以理解为“一份主存 + 多份索引”，而不是复制多份 Student 全量数据。
// - 思路上有点像 arena 的“句柄化访问”（用 id 回主存取值），
//   但这里本质是索引化存储，不是 arena allocator。
struct StudentStore {
    // 主索引：按 id O(1) 查找。
    by_id: HashMap<u32, Student>,
    // 有序 id 索引：便于稳定 list（默认按 id 升序）。
    ids: BTreeSet<u32>,
    // 名称索引：便于 `search name <name>` 精确匹配。
    name_index: BTreeMap<String, BTreeSet<u32>>,
    next_id: u32,
}

impl StudentStore {
    fn new() -> Self {
        Self {
            by_id: HashMap::new(),
            ids: BTreeSet::new(),
            name_index: BTreeMap::new(),
            next_id: 1,
        }
    }

    fn add(&mut self, name: &str, age: u8, class_name: &str) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        let student = Student {
            id,
            name: name.to_string(),
            age,
            class_name: class_name.to_string(),
        };

        self.by_id.insert(id, student.clone());
        self.ids.insert(id);
        self.name_index
            .entry(student.name.clone())
            .or_default()
            .insert(id);

        id
    }

    fn get_by_id(&self, id: u32) -> Option<&Student> {
        self.by_id.get(&id)
    }

    fn list_by_id(&self) -> Vec<&Student> {
        self.ids
            .iter()
            .filter_map(|id| self.by_id.get(id))
            .collect::<Vec<&Student>>()
    }

    fn search_by_name_exact(&self, name: &str) -> Vec<&Student> {
        match self.name_index.get(name) {
            Some(id_set) => id_set
                .iter()
                .filter_map(|id| self.by_id.get(id))
                .collect::<Vec<&Student>>(),
            None => Vec::new(),
        }
    }

    fn remove(&mut self, id: u32) -> bool {
        let removed = match self.by_id.remove(&id) {
            Some(v) => v,
            None => return false,
        };

        self.ids.remove(&id);
        self.remove_name_index(&removed.name, id);
        true
    }

    fn modify(&mut self, id: u32, name: &str, age: u8, class_name: &str) -> bool {
        let old_name = match self.by_id.get(&id) {
            Some(v) => v.name.clone(),
            None => return false,
        };
        if old_name != name {
            self.remove_name_index(&old_name, id);
            self.name_index
                .entry(name.to_string())
                .or_default()
                .insert(id);
        }

        let student = self
            .by_id
            .get_mut(&id)
            .expect("id checked above, get_mut must succeed");
        student.name = name.to_string();
        student.age = age;
        student.class_name = class_name.to_string();
        true
    }

    fn remove_name_index(&mut self, name: &str, id: u32) {
        if let Some(set) = self.name_index.get_mut(name) {
            set.remove(&id);
            if set.is_empty() {
                self.name_index.remove(name);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum SortField {
    Id,
    Name,
    Age,
    Class,
}

#[derive(Debug, Clone, Copy)]
enum SortDirection {
    Asc,
    Desc,
}

fn print_help() {
    println!("commands:");
    println!("  add <name> <age> <class>              add a student");
    println!("  list                                  list all students by id");
    println!("  remove <id>                           remove by id");
    println!("  mod <id> <name> <age> <class>         modify by id");
    println!("  search id <id>                        search by id (O(1) index)");
    println!("  search name <name>                    search by exact name");
    println!("  order <id|name|age|class> <asc|desc>  ordered view");
    println!("  help                                  show help");
    println!("  quit | exit                           leave repl");
}

fn print_students(students: &[&Student]) {
    if students.is_empty() {
        println!("(empty)");
        return;
    }

    println!("{:<4} {:<12} {:<4} {:<12}", "id", "name", "age", "class");
    for s in students {
        println!(
            "{:<4} {:<12} {:<4} {:<12}",
            s.id, s.name, s.age, s.class_name
        );
    }
}

fn parse_id(raw: &str) -> Option<u32> {
    match raw.parse::<u32>() {
        Ok(v) => Some(v),
        Err(_) => {
            println!("error: invalid id `{raw}`");
            None
        }
    }
}

fn parse_age(raw: &str) -> Option<u8> {
    match raw.parse::<u8>() {
        Ok(v) => Some(v),
        Err(_) => {
            println!("error: invalid age `{raw}`");
            None
        }
    }
}

fn parse_sort_field(raw: &str) -> Option<SortField> {
    match raw {
        "id" => Some(SortField::Id),
        "name" => Some(SortField::Name),
        "age" => Some(SortField::Age),
        "class" => Some(SortField::Class),
        _ => {
            println!("error: invalid field `{raw}`");
            None
        }
    }
}

fn parse_sort_direction(raw: &str) -> Option<SortDirection> {
    match raw {
        "asc" => Some(SortDirection::Asc),
        "desc" => Some(SortDirection::Desc),
        _ => {
            println!("error: invalid direction `{raw}`");
            None
        }
    }
}

fn order_students(store: &StudentStore, field: SortField, direction: SortDirection) {
    let mut rows = store.by_id.values().collect::<Vec<&Student>>();
    rows.sort_by(|a, b| {
        let ord = match field {
            SortField::Id => a.id.cmp(&b.id),
            SortField::Name => a.name.cmp(&b.name).then_with(|| a.id.cmp(&b.id)),
            SortField::Age => a.age.cmp(&b.age).then_with(|| a.id.cmp(&b.id)),
            SortField::Class => a
                .class_name
                .cmp(&b.class_name)
                .then_with(|| a.id.cmp(&b.id)),
        };
        match direction {
            SortDirection::Asc => ord,
            SortDirection::Desc => match ord {
                Ordering::Less => Ordering::Greater,
                Ordering::Equal => Ordering::Equal,
                Ordering::Greater => Ordering::Less,
            },
        }
    });
    print_students(&rows);
}

// 返回值：true 表示继续循环；false 表示退出。
fn handle_command(line: &str, store: &mut StudentStore) -> bool {
    let parts = line.split_whitespace().collect::<Vec<&str>>();
    if parts.is_empty() {
        return true;
    }

    match parts[0] {
        "add" => {
            if parts.len() != 4 {
                println!("usage: add <name> <age> <class>");
                return true;
            }
            if let Some(age) = parse_age(parts[2]) {
                let id = store.add(parts[1], age, parts[3]);
                println!("ok: added id={id}");
            }
        }
        "list" => {
            if parts.len() != 1 {
                println!("usage: list");
                return true;
            }
            let rows = store.list_by_id();
            print_students(&rows);
        }
        "remove" => {
            if parts.len() != 2 {
                println!("usage: remove <id>");
                return true;
            }
            if let Some(id) = parse_id(parts[1]) {
                if store.remove(id) {
                    println!("ok: removed id={id}");
                } else {
                    println!("error: id={id} not found");
                }
            }
        }
        // 这里命令名写 `mod`，仅是字符串命令，和 Rust 关键字不冲突。
        "mod" => {
            if parts.len() != 5 {
                println!("usage: mod <id> <name> <age> <class>");
                return true;
            }
            let id = match parse_id(parts[1]) {
                Some(v) => v,
                None => return true,
            };
            let age = match parse_age(parts[3]) {
                Some(v) => v,
                None => return true,
            };
            if store.modify(id, parts[2], age, parts[4]) {
                println!("ok: modified id={id}");
            } else {
                println!("error: id={id} not found");
            }
        }
        "search" => {
            if parts.len() < 3 {
                println!("usage: search id <id> | search name <name>");
                return true;
            }
            match parts[1] {
                "id" => {
                    if parts.len() != 3 {
                        println!("usage: search id <id>");
                        return true;
                    }
                    let id = match parse_id(parts[2]) {
                        Some(v) => v,
                        None => return true,
                    };
                    match store.get_by_id(id) {
                        Some(s) => print_students(&[s]),
                        None => println!("(empty)"),
                    }
                }
                "name" => {
                    if parts.len() != 3 {
                        println!("usage: search name <name>");
                        return true;
                    }
                    let rows = store.search_by_name_exact(parts[2]);
                    print_students(&rows);
                }
                _ => println!("usage: search id <id> | search name <name>"),
            }
        }
        "order" => {
            if parts.len() != 3 {
                println!("usage: order <id|name|age|class> <asc|desc>");
                return true;
            }
            let field = match parse_sort_field(parts[1]) {
                Some(v) => v,
                None => return true,
            };
            let direction = match parse_sort_direction(parts[2]) {
                Some(v) => v,
                None => return true,
            };
            order_students(store, field, direction);
        }
        "help" => print_help(),
        "quit" | "exit" => return false,
        _ => println!("unknown command. type `help`"),
    }

    true
}

fn main() -> io::Result<()> {
    // 内存态 demo：不落盘，退出后数据清空。
    let mut store = StudentStore::new();

    println!("student-cli demo");
    println!("type `help` to see commands");

    let stdin = io::stdin();
    loop {
        print!("sms> ");
        io::stdout().flush()?;

        let mut line = String::new();
        if stdin.read_line(&mut line)? == 0 {
            // EOF（如 Ctrl-D）时退出。
            println!();
            break;
        }
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if !handle_command(line, &mut store) {
            break;
        }
    }

    println!("bye");
    Ok(())
}
