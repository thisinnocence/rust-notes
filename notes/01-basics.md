# 01 - 基础语法（对照 C/C++）

运行：

```bash
cargo run --bin 01_basics
```

## 1. 关键字与变量绑定

示例覆盖了这些常见关键字：

- `let`：创建变量绑定。
- `mut`：声明可变绑定。
- `const`：编译期常量（本例 `MAX_RETRY`）。
- `enum` / `struct` / `impl` / `trait`：定义类型与行为。
- `fn`：定义函数。
- `match`：穷尽匹配。

和 C/C++ 对照：

- Rust 的变量默认不可变；C/C++ 默认可变。
- `const` 更接近“真正不可变值”，不是 C 宏替换。
- `let score = score;` 是 shadowing（新绑定），不是同一个变量改类型。

## 2. 控制流

示例覆盖：

- `if` 表达式：直接返回值给 `level`。
- `for`：区间迭代 `0..3`。
- `while`：条件循环。
- `loop` + `break value`：无限循环并返回值。
- `match`：按区间分类分数。

和 C/C++ 对照：

- Rust 的 `if` / `match` 是表达式，不只是语句。
- `match` 必须穷尽，减少漏分支。

## 3. 函数

本例函数：

- `add(a, b) -> i32`：普通返回值。
- `classify(score) -> &'static str`：返回静态字符串切片。
- `maybe_timeout(mode) -> Option<u64>`：用 `Option` 表达“有或无”。
- `inspect_ref(v: &i32)`：展示借用与地址打印。

和 C/C++ 对照：

- 没有 `return` 也能返回：最后一个表达式即返回值。
- `Option<T>` 比 `nullptr` / 特殊值更类型安全。

## 4. "类" 与 "接口" 在 Rust 的对应

Rust 没有 class，常用组合是：

- `struct`：承载数据（类似 class 的字段部分）。
- `impl`：给类型实现方法（类似成员函数）。
- `trait`：行为契约（接口能力）。

本例中：

- `TaskRunner` 是数据结构。
- `impl TaskRunner` 里有 `new` 和 `run`。
- `trait Describe` + `impl Describe for TaskRunner` 对应“接口实现”。

## 5. 语法解码（你提到的疑问）

### `#[derive(Debug)]` 是什么

- 这是 attribute（属性），作用在后面的类型定义上。
- `derive(Debug)` 表示“自动生成 Debug trait 的实现”。
- 生成后才能写 `println!("{:?}", value)`。

对照 C++：

- 有点像给类型自动生成一个“调试打印能力”，避免手写格式化函数。

### `name: String` 为什么类型在后面

- Rust 的变量/字段声明统一是 `name: Type`。
- 你会在函数参数里看到同样风格：`fn f(x: i32)`。
- 这和 Go 语法形式相似，但语义是 Rust 的所有权/借用模型。

类型后置写法的常见好处：

- 统一性强：变量、字段、函数参数都用同一模式，阅读成本更低。
- 先看名字再看类型：读代码时先知道“这个值是干什么的”，再看“它是什么类型”。
- 类型可以更复杂时更清晰：例如 `handler: Arc<Mutex<Vec<Task>>>`，先定位语义名再解析类型层级。
- 配合类型推断更自然：局部变量常可省类型，必须标注时再补 `name: Type`，风格一致。

### `&` 是不是取地址

- `&x` 在 Rust 中是“借用 x，得到引用 `&T`”。
- 它不是 C 的裸指针，也不是随意可空、可悬垂的地址值。
- 引用受借用规则保护（生命周期、可变性、别名规则）。

### 有没有引用的概念

- 有，而且是核心概念：
- `&T`：不可变借用（只读）。
- `&mut T`：可变借用（可写，且同一时刻只能有一个）。
- `*r`：解引用，读取引用指向的值。
- `{:p}`：打印引用指向的地址，便于你做底层直觉对照。

## 6. 你可重点关注

- 为什么 Rust 把“默认不可变 + 穷尽匹配”放到语法层。
- `struct + impl + trait` 如何替代传统 OOP class hierarchy。
- `Option/Result` 如何把空值与错误做成类型系统的一部分。
