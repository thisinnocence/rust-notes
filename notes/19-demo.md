# 19 - CLI 学生管理小练习（CRUD + Search + Order）

运行：

```bash
cargo run --bin 19_demo
```

本节目标：在一个最小 CLI 程序里，把“增删改查 + 快速查找 + 排序视图”串起来。

## 1. 练习目标

- 练习数据建模：`struct Student`。
- 练习命令分发：把 CLI 字符串映射到不同处理函数。
- 练习基本 CRUD：
  - `add`（Create）
  - `list`（Read）
  - `mod`（Update）
  - `remove`（Delete）
- 扩展：
  - `search`：支持按 id / name 查询。
  - `order`：支持按字段排序输出。

## 2. 命令约定

- `add <name> <age> <class>`：新增学生。
- `list`：按 id 升序列出所有学生。
- `remove <id>`：按 id 删除。
- `mod <id> <name> <age> <class>`：按 id 修改。
- `search id <id>`：按 id 查询单条记录。
- `search name <name>`：按 name 精确匹配查询。
- `order <id|name|age|class> <asc|desc>`：排序视图。
- `help`：查看帮助。
- `quit` / `exit`：退出程序。

说明：

- 命令名 `mod` 只是字符串，和 Rust 关键字 `mod` 不冲突。
- 为了保持练习简单，`name` 和 `class` 默认不含空格。

## 3. 底层数据结构（尽量高效）

示例使用 `StudentStore`，维护三类索引：

- `HashMap<u32, Student>`：主索引，按 id 快速查找（平均 O(1)）。
- `BTreeSet<u32>`：id 有序索引，用于稳定 `list` 输出。
- `BTreeMap<String, BTreeSet<u32>>`：name 索引，用于 `search name` 精确匹配。

这比单纯 `Vec<Student>` 更接近真实业务的“主索引 + 二级索引”思路。

## 4. 主流程

1. 读取用户输入。
2. `split_whitespace` 解析命令和参数。
3. `match` 分发到 `add/list/remove/mod/search/order`。
4. 输出结果并进入下一轮。

## 5. 一段示例交互

```text
sms> add alice 18 class1
ok: added id=1
sms> add bob 19 class2
ok: added id=2
sms> search id 2
id   name         age  class
2    bob          19   class2
sms> search name alice
id   name         age  class
1    alice        18   class1
sms> order age desc
id   name         age  class
2    bob          19   class2
1    alice        18   class1
sms> mod 2 bobby 20 class3
ok: modified id=2
sms> remove 1
ok: removed id=1
sms> list
id   name         age  class
2    bobby        20   class3
sms> quit
bye
```

## 6. 配套代码

对应示例：[`../src/bin/19_demo.rs`](../src/bin/19_demo.rs)。

- `StudentStore`：主存与索引维护（add/remove/modify 时同步更新）。
- `handle_command`：命令分发中心。
- `search` / `order`：查询与排序命令实现。
- `parse_*`：输入解析与错误提示。
