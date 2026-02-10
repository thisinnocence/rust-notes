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

## 6. C++17 vs Rust 核心实现对照

本仓库这节有两份等价实现：

- Rust：[`../src/bin/19_demo.rs`](../src/bin/19_demo.rs)
- C++17：[`../src/bin/19_demo.cc`](../src/bin/19_demo.cc)

核心对比（同功能，不同语言表达）：

| 维度 | Rust 写法 | C++17 写法 |
| --- | --- | --- |
| 主存 + 索引模型 | `HashMap` + `BTreeSet` + `BTreeMap` | `unordered_map` + `set` + `map` |
| 命令分发 | `match parts[0]` | `if/else` 分支 |
| 可变性表达 | 默认不可变，`mut` 显式声明 | 默认可变，`const` 约束只读 |
| 空值/查找失败 | `Option<T>`（`Some/None`） | 指针判空或 `bool` 返回值 |
| 字符串与集合 API | 更偏函数式链式（`iter/filter_map/collect`） | 容器 + 算法组合（`std::sort` 等） |
| 约束检查时机 | 编译期借用检查更强（避免别名可变冲突） | 主要靠代码约束与运行期行为 |

这份 demo 里最典型的一点：

- Rust 在 `modify` 中需要显式处理“借用阶段分离”（先查旧值，再更新索引，再写回），编译器会阻止潜在别名可变冲突。
- C++17 同类逻辑通常直接按步骤写，灵活度高，但更依赖工程规范保证不出错。

### 6.1 Rust 在“编译阶段/编码当下”要求你先想清楚什么

这一节是关键：Rust 不是让你“晚点再修”，而是要求你在写代码时就把边界讲清楚。

| 必须提前考虑的问题 | Rust 在这份 demo 里的体现 | C++17 常见做法（对照） |
| --- | --- | --- |
| 谁拥有数据（ownership） | `StudentStore` 持有主存与索引；函数签名里 `&self` / `&mut self` 区分读写 | 通常靠约定控制谁改数据，语言层默认更宽松 |
| 能否同时存在多个可变访问 | `modify` 里不能一边持有 `get_mut`，一边再改别的索引；必须分阶段写 | 同类写法常可直接编译，通过评审/测试兜底 |
| 可选值是否被处理 | `get`、`find`、解析函数返回 `Option`，调用方必须 `match`/分支处理 | 可用指针/迭代器/布尔返回，是否全面检查更依赖人 |
| 输入解析失败路径是否完整 | `parse_id` / `parse_age` 失败时立即返回并提示，不允许“忽略错误继续跑” | 也可以写得严谨，但语言不会强制统一风格 |
| 索引与主存一致性更新顺序 | `add/remove/modify` 里显式同步 `by_id`、`ids`、`name_index`，顺序清晰 | 同样能实现，但“先改哪边、漏改哪边”更靠规范防错 |
| 排序比较器是否稳定可判定 | `order` 里明确 tie-break（同 `name/age/class` 再按 `id`） | 也能做，但常见遗漏 tie-break 导致行为不稳定 |

可以把 Rust 的体验概括为：

- 你在写代码时就要回答“我是否真的安全地持有/修改了这份数据”。
- 编译器会把“说不清楚”的地方直接卡住，不让代码进入运行期。

对照 C++17：

- 你可以更快写出“能编译”的版本。
- 但是否覆盖所有边界，更多依赖测试、review 和长期维护纪律。

### 6.2 C++ 的灵活性在哪里，以及对应隐患

C++ 的优势确实是“表达自由度高、迁移成本低”。

这份 demo 语境下，常见灵活点：

- 你可以先写流程，再逐步补类型约束和边界处理。
- 指针、引用、迭代器、容器组合方式非常自由。
- `if/else` 和返回值风格可以快速拼出可运行版本。
- 与历史 C/C++ 代码、第三方库、平台 API 融合阻力更小。

但这些灵活性通常伴随“需要默认约定好”的隐式前提：

| 灵活点 | 运行期隐患或隐式约定 |
| --- | --- |
| 读写路径自由 | 需要团队约定“谁可改主存、谁可改索引”，否则容易出现索引与主存不一致 |
| 错误处理风格自由 | 必须约定“每次都检查返回值/判空”，否则漏判导致运行期错误或脏数据 |
| 比较器实现自由 | 若未统一 tie-break 规则，排序结果可能不稳定，行为随数据分布变化 |
| 生命周期管理自由 | 需要约定对象引用/指针何时失效，避免悬垂引用或越界访问 |
| 高性能优化自由 | 需要约定性能优化不能破坏语义一致性，否则会出现“快但错”的实现 |

换句话说：

- C++ 的“快”常体现在开发起步和系统兼容性。
- Rust 的“稳”常体现在把大量隐式约定转成编译期显式约束。

语法层面可以这样快速类比：

- 模块/路径：Rust `std::env::var` 对应 C++ 的 `std::...` 命名空间路径。
- 结构体初始化：Rust `Student { id, name, ... }` 对应 C++ 聚合初始化 `{id, name, ...}`。
- 错误分支：Rust 常见 `match`；C++ 习惯 `if/else` + 返回值检查。

结论：

- 同样的业务结构，两边都能写得清晰。
- Rust 倾向“更强编译期约束”；C++ 倾向“更高表达自由度”。
- 在团队协作里，Rust 往往把更多边界错误前移到编译期。

## 7. 配套代码

对应示例：[`../src/bin/19_demo.rs`](../src/bin/19_demo.rs)。

- `StudentStore`：主存与索引维护（add/remove/modify 时同步更新）。
- `handle_command`：命令分发中心。
- `search` / `order`：查询与排序命令实现。
- `parse_*`：输入解析与错误提示。
