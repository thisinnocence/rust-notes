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

### `!` 这个符号是什么意思

在你当前看到的代码里，`!` 主要有两种含义：

- 宏调用标记：`println!`、`format!`、`matches!`、`write!`。
- 逻辑非运算符：`!is_fast`，把 `true/false` 取反。

为什么宏要写 `!`：

- 这是 Rust 区分“普通函数调用”和“宏展开”的语法标记。
- 宏在编译阶段展开，能接受可变参数和特定语法形态（例如 `println!("x={}", x)` 这种格式串 + 参数列表）。
- 你可以先把它理解为“带语法能力的代码生成器入口”。

和 C/C++ 对照：

- C 的 `printf` 是普通函数（或库函数调用），变参靠 ABI 与格式串约定。
- Rust 的 `println!` 是宏，先展开，再做编译期格式与类型检查。
- 所以 Rust 在这类打印场景里更强调编译期安全。

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
