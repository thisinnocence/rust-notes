# 01 - 基础语法

运行：

```bash
cargo run --bin 01_basics
```

## 1. 常用关键字速览

按系统编程高频场景先记这些：

- 绑定与可变性：`let`、`mut`、`const`、`static`
- 类型与抽象：`struct`、`enum`、`trait`、`type`、`impl`
- 流程控制：`if`、`else`、`match`、`for`、`while`、`loop`、`break`、`continue`、`return`
- 模块与可见性：`mod`、`use`、`pub`、`crate`、`super`、`self`
- 类型自指：`Self`（当前实现类型）
- 错误与空值：`Result`、`Option`、`match`、`if let`
- 所有权相关：`move`、`ref`、`as`
- 并发与边界：`async`、`await`、`unsafe`

关键点：

- Rust 变量默认不可变；C/C++ 变量默认可变。
- `const` 是编译期常量，不是 C 的宏替换。
- `let score = score;` 是 shadowing（新绑定），不是同一变量改类型。
- `Self` 是类型层关键字；`self` 是值层方法接收者。

## 2. 常见语法符号速览

这部分和关键字同样重要，入门阶段会高频遇到：

- `&` / `&mut`：借用与可变借用。
- `*`：解引用（例如 `*r` 读取引用指向值）。
- `::`：路径分隔符（模块/类型命名空间）。
- `.`：访问字段或调用方法。
- `->`：函数返回类型标记。
- `=>`：`match` 分支或宏规则的映射箭头。
- `a..b` / `a..=b`：区间语法，前者右开、后者闭区间。
- `<...>`：泛型参数列表（类型/生命周期参数）。
- `#[...]`：attribute 语法（如 `#[derive(Debug)]`）。
- `!`：宏调用标记或逻辑非。
- `?`：错误快速返回运算符（在 `Result` 语境）。
- `'a`：生命周期参数标记。

区间补充：

- `..=` 必须连写为一个符号，不能拆成 `.. =`。

速记：

- 关键字解决“这是什么构造”。
- 语法符号解决“这些构造如何拼起来”。

## 3. 基本数据类型速览

| Rust 类型 | 说明 | C/C++ 对照 |
| --- | --- | --- |
| `bool` | 布尔值（`true`/`false`） | `bool` |
| `char` | Unicode 标量值（4 字节） | `char` 在 C/C++ 常是 1 字节字符单元 |
| `i8`..`i128` | 有符号整数 | `int8_t`..`int64_t`（C++ 常见到 64 位） |
| `u8`..`u128` | 无符号整数 | `uint8_t`..`uint64_t` |
| `isize`/`usize` | 与指针位宽一致的整数 | `intptr_t`/`uintptr_t` |
| `f32`/`f64` | 浮点数 | `float`/`double` |
| `()` | 单元类型（无有效值） | 近似于 `void` 语义位 |
| `&str` | 借用字符串切片（UTF-8） | 近似 `const char* + len` 语义 |
| `String` | 拥有所有权的可增长字符串 | 近似 `std::string` |
| `[T; N]` | 固定长度数组 | `T[N]` / `std::array<T, N>` |
| `&[T]` | 切片（借用的连续区间） | 近似 `pointer + length` 视图 |
| `*const T` / `*mut T` | 裸指针（不带借用安全保证） | `const T*` / `T*` |
| `(T1, T2, ...)` | 元组 | C++ 没有同语法，接近 `std::tuple` |

补充：

- Rust 整数默认不做隐式窄化/扩宽转换，跨类型通常要显式转换。
- `str` 本体是 DST（动态大小类型），实际使用通常是 `&str`。

指针与成员访问对照：

- C/C++ 常见 `a->mem`，Rust 统一写 `a.mem`（编译器会自动解引用）。
- 对于裸指针 `*const T` / `*mut T`，需要在 `unsafe` 中显式解引用：`(*p).mem`。
- 日常业务代码优先使用 `&T` / `&mut T`，裸指针多用于 FFI 或底层实现。

## 4. 控制流与表达式风格

本章核心：Rust 的 `if` / `match` 是表达式，不只是语句。

示例重点：

- `if` 直接产出值赋给 `level`。
- `match` 产出值并且要求穷尽分支。
- `for` / `while` / `loop` 三种循环都支持 `break` 与 `continue`。
- `loop` 支持 `break value`，可把值返回给外层绑定。
- 标签循环支持 `break 'outer`，可精确跳出外层循环。

对应代码：

```rust
let stop_at = loop {
    attempts += 1;
    if attempts >= MAX_RETRY {
        break attempts;
    }
};
```

理解方式：

- `loop {}` 语义等价于 Rust 版 `while (true)` / `for(;;)`。
- `break` 可用于三类循环，不与 `loop` 绑定。
- `break value` 主要用于 `loop` 作为表达式时返回值。

## 5. 函数与类型写法

本例函数：

- `add(a, b) -> i32`：普通返回值
- `classify(score) -> &'static str`：返回静态字符串切片
- `maybe_timeout(mode) -> Option<u64>`：显式表达“有值或无值”
- `inspect_ref(v: &i32)`：借用与地址打印

关键点：

- Rust 可省略 `return`，最后一个表达式即返回值。
- `Option<T>` 把“空值”提升到类型层，而不是约定特殊值。
- `name: Type` 用于变量、字段、函数参数，统一且可读。

`&` 的语义：

- `&x` 是借用，得到 `&T`（引用）。
- 不是 C 的裸指针语义；受借用规则保护。
- `&T` 只读，`&mut T` 可写且独占。

`Self` / `self` 的语义：

- `Self`：当前实现块里的“当前类型”，如 `impl TaskRunner` 中 `Self == TaskRunner`。
- `self`：方法接收者，表示当前实例（可写成 `self`、`&self`、`&mut self`）。

## 6. Struct / Trait：Rust 对应“类与接口”的方式

Rust 没有 class，常见建模组合是：

- `struct`：承载数据
- `impl`：实现方法
- `trait`：行为契约

本例映射：

- `TaskRunner`：数据结构
- `impl TaskRunner`：`new`/`run` 方法
- `trait Describe` + `impl Describe for TaskRunner`：接口能力实现

## 7. 语法解码

### 7.1 derive Debug 是什么

- attribute（属性），作用于类型定义。
- `derive(Debug)` 自动生成 `Debug` trait 实现。
- 有了它才能用 `println!("{:?}", value)`。

### 7.2 `println!` / `fmt` 和 `printf` 的关键区别

- `println!` 是宏，不是 C 风格 varargs 函数。
- 编译期检查格式与参数是否匹配。
- `{:?}` 需要 `Debug`；`{}` 需要 `Display`。
- 不匹配会在编译期报错，而非运行期未定义行为。

### 7.3 `!` 是什么

在本章里，`!` 有两类常见含义：

- 宏调用标记：`println!`、`format!`、`matches!`、`write!`
- 逻辑非运算：`!is_fast`

### 7.4 Rust 宏 vs C 宏 vs C++ 模板

- C 宏：文本替换，类型不参与。
- C++ 模板：类型系统内的编译期泛型机制。
- Rust 宏：基于 token/语法规则展开，不是纯文本替换。

### 7.5 `$` 是什么

- `$` 主要出现在宏系统里，属于宏语法的一部分。
- 在 `macro_rules!` 中，`$name:kind` 表示“宏变量 + 片段类型”。
- 例如：`$a:expr` 表示 `$a` 匹配一个表达式，`$name:ident` 表示匹配一个标识符。
- 重复匹配常写成 `$(...)*`、`$(...),+`，用于收集可变参数列表。
- 普通 Rust 表达式里基本不把 `$` 当通用运算符使用。

### 7.6 `=>` 是什么 对照 TS

- 在 Rust 中，`=>` 主要用于 `match` 分支和 `macro_rules!` 规则。
- 不是 TS 那种箭头函数定义语法。

### 7.7 为什么类型写在后面：`name: Type`

- 变量、字段、参数统一语法形态。
- 阅读时先看语义名，再看具体类型。
- 与类型推断结合后更统一。

### 7.8 `_` 和 `'_` 的区别

| 写法 | 典型位置 | 含义 |
| --- | --- | --- |
| `_` | 变量名/模式匹配 | 占位并忽略值（或表示“未使用”） |
| `'_` | 引用类型参数位置，如 `Formatter<'_>` | 匿名生命周期，占位给编译器推断 |

补充：

- `'_` 不是 Python 风格“变量名下划线占位符”。
- `'_` 的语义层级在类型系统（生命周期），`_` 多在值/模式层级。

## 8. 示例覆盖总览

对应示例：[`../src/bin/01_basics.rs`](../src/bin/01_basics.rs)。

- 变量绑定：不可变、可变、shadowing
- 常量：`const MAX_RETRY`
- 控制流：`if`、`match`、`for`、`while`、`loop`、标签 `break`
- 函数：普通返回、`Option` 返回、借用参数
- 类型：`enum Mode`、`struct TaskRunner`
- 方法与 trait：`impl`、`Describe`、`Display`
- 宏：`println!`、`matches!`、`sum_i32!`
- 引用：`&T`、`*r`、`{:p}` 地址打印

## 9. 关键提醒

- 先记“关键词 + 大概作用”，再追求精确语义。
- 控制流要优先建立“表达式化”思维（`if/match` 产值）。
- `struct + impl + trait` 是 Rust 的核心建模骨架。
- `println!`/`format!` 是宏体系，和 C 的 `printf` 路径不同。
