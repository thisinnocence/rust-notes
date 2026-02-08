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
- `while`：条件循环（示例含 `continue` 与 `break`）。
- `loop` + `break value`：无条件循环并返回值。
- 标签循环：`break 'outer` 直接跳出外层循环。
- `match`：按区间分类分数。

和 C/C++ 对照：

- Rust 的 `if` / `match` 是表达式，不只是语句。
- `match` 必须穷尽，减少漏分支。

关于你问的这段：

```rust
let stop_at = loop {
    attempts += 1;
    if attempts >= MAX_RETRY {
        break attempts;
    }
};
```

- 你的理解正确：`loop {}` 语义上就是 Rust 版 `while (true)` / `for(;;)`。
- `break` 不需要和 `loop` 强绑定，`for` / `while` / `loop` 都可以用 `break`。
- 但 `break value`（带返回值）主要用于 `loop` 表达式，用于把值返回给外层绑定（这里是 `stop_at`）。
- `for` / `while` 通常只用 `break;`（不带值），它们本身不作为“返回某个值的表达式”来用。
- `continue` 也适用于三类循环，表示跳过本轮进入下一轮。
- 若是嵌套循环，可用标签写法 `break 'outer` 精确跳出指定层。

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

和 `printf` / `{fmt}` 的对应关系（重点）：

- `println!` 里的格式位不是“随便填”的，和类型实现的 trait 绑定。
- `{:?}` 需要类型实现 `Debug` trait；`#[derive(Debug)]` 就是在自动补这个实现。
- `{}` 需要类型实现 `Display` trait；通常要你手写 `impl Display for Type`。
- C 的 `printf("%d", x)` 主要靠格式符和参数类型约定；Rust 是编译期 trait 检查，不匹配直接编译报错。
- 你可以把它理解为：Rust 把“能不能这样打印”变成了类型系统规则，不是运行时碰运气。

本例里可直接对照看：

- `println!("runner mode debug={:?}", runner.mode);` 使用 `Debug`。
- `println!("runner mode display={}", runner.mode);` 使用 `Display`（由我们手写实现）。

`fmt` 底层原理（对照 `printf`）：

- `println!` / `format!` 首先是宏，不是 C 那种 varargs 函数调用。
- 宏在编译期解析格式串，把它变成 `format_args!` 生成的 `Arguments` 结构。
- 编译期会检查占位符和参数数量是否匹配、参数是否实现了所需 trait（如 `Display` / `Debug`）。
- 运行时并不是“按 `%d/%s` 推断内存布局”去读可变参数，而是按 `Arguments` 中记录的格式片段逐个调用对应 trait 的 `fmt` 方法输出。
- 每个参数以“已知类型 + trait 行为”参与格式化，不依赖 C ABI 的可变参数约定。

性能和行为上你可这样理解：

- `println!` 通常直接把格式化结果写到输出流，不必先构造 `String`。
- `format!` 会构造并返回一个 `String`（会有分配）。
- 类型/占位符不匹配在编译期报错，而不是运行时未定义行为。

### `!` 这个符号是什么意思

在你当前看到的代码里，`!` 主要有两种含义：

- 宏调用标记：`println!`、`format!`、`matches!`、`write!`。
- 自定义宏调用：`sum_i32!(10, 20)`（由 `macro_rules!` 定义）。
- 逻辑非运算符：`!is_fast`，把 `true/false` 取反。

为什么宏要写 `!`：

- 这是 Rust 区分“普通函数调用”和“宏展开”的语法标记。
- 宏在编译阶段展开，能接受可变参数和特定语法形态（例如 `println!("x={}", x)` 这种格式串 + 参数列表）。
- 你可以先把它理解为“带语法能力的代码生成器入口”。

和 C/C++ 对照：

- C 的 `printf` 是普通函数（或库函数调用），变参靠 ABI 与格式串约定。
- Rust 的 `println!` 是宏，先展开，再做编译期格式与类型检查。
- 所以 Rust 在这类打印场景里更强调编译期安全。

### Rust 宏 vs C 宏 vs C++ 模板

你可以用下面这个映射快速建立直觉：

- C 宏（`#define`）：预处理器文本替换，类型不参与，容易出现副作用和优先级坑。
- C++ 模板：类型系统内的编译期泛型机制，做类型多态和代码生成，不是文本替换。
- Rust 宏（`macro_rules!`）：基于 token/语法模式匹配并展开，受 Rust 语法约束，不是纯文本替换。

关键差异：

- 和 C 宏比：Rust 宏更“语法安全”，不直接按字符串替换。
- 和 C++ 模板比：Rust 宏更偏“语法生成”；模板更偏“类型泛化”。
- Rust 的泛型（`fn<T>`）才更接近 C++ 模板的主要用途。

落地理解（当前例子）：

- `sum_i32!(10, 20)` 演示了一个最小 `macro_rules!` 宏。
- 它在编译期展开成表达式，再进入正常类型检查流程。

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
