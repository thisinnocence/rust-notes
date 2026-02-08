# 03 - 生命周期

本章目标：把 Lifetimes 从“语法负担”变成“接口契约工具”。

---

## 1. 先定一个统一心智模型

生命周期标注（如 `'a`）**不负责管理内存释放**，只负责描述：

- 这个引用从哪里借来；
- 这个引用最晚能用到哪里；
- 返回引用与哪些输入引用存在绑定关系。

一句话：`'a` 是编译期约束变量，不是运行期资源管理器。

---

## 2. 先用 C++ 问题定义 Rust 的价值

### 2.1 C++ 中常见“合法但危险”的代码

```cpp
const std::string& longer(const std::string& a, const std::string& b) {
    return a.size() > b.size() ? a : b;
}
```

这在 C++ 类型系统里是合法的，但调用方可以写出悬挂引用：

```cpp
auto& r = longer(std::string("tmp"), long_lived);
// r 可能绑定到临时对象，后续使用 UB
```

### 2.2 Rust 的做法：把“口头约定”写进类型

```rust
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}
```

这不是让 `a` 和 `b` 活得一样久，而是告诉编译器：

- 返回值必须来自输入借用；
- 调用点上，返回值最多活到两者可共同成立的那段区间。

---

## 3. `'a` 到底是什么（逐符号拆解）

```rust
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str
```

- `<'a>`：声明一个生命周期参数；
- `a: &'a str`：`a` 是在 `'a` 约束下的借用；
- `b: &'a str`：`b` 也受同一个 `'a` 约束；
- `-> &'a str`：返回值与输入共享这套约束。

关键点：

- `'a` 只是名字，占位符而已，可写成 `'x`、`'input`、`'left`，语义不变；
- 生命周期不进入运行期对象布局，几乎可视作零运行时开销；
- 它和 `std::shared_ptr` 完全不是一类机制，不能“延寿”。

---

## 4. 逻辑链示例：从报错到修正

### 4.1 例子 A：返回局部引用（Rust 直接拒绝）

```rust
fn bad_ref<'a>() -> &'a str {
    let s = String::from("hello");
    &s
}
```

编译器拒绝的本质原因：

- `s` 在函数结束即销毁；
- 返回引用试图在所有者销毁后继续存在；
- 这正是 C++ 里经典的悬挂引用。

正确改法（返回拥有值）：

```rust
fn good_owned() -> String {
    String::from("hello")
}
```

### 4.2 例子 B：多输入引用 + 返回引用

```rust
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

fn main() {
    let outer = String::from("outer");
    let r: &str;

    {
        let inner = String::from("in");
        r = longer(outer.as_str(), inner.as_str());
    }

    // println!("{r}"); // 编译错误：r 可能引用 inner，inner 已销毁
}
```

这段是最关键训练点：  
你在函数体里知道“有时会返回 `outer`”，但类型系统按“所有路径都安全”检查。只要有路径返回 `inner`，`r` 就不能越过 `inner` 的作用域。

### 4.3 例子 C：分离生命周期（表达“返回只借自 x”）

```rust
fn pick_x<'a, 'b>(x: &'a str, _y: &'b str) -> &'a str {
    x
}
```

这里不要求 `x`、`y` 共享同一生命周期，因为返回值明确只绑定 `x`。  
这就是生命周期参数的真正价值：**精准表达引用来源关系**。

---

## 5. Borrow Checker 在检查什么

可以用这条规则理解 90% 报错：

- 引用的有效区间，必须完全落在被借用值的有效区间内。

再叠加别名规则：

- 同时多个只读借用 `&T` 可以；
- 或单个可变借用 `&mut T`；
- 不能同时“可变 + 只读”借用同一值。

所以生命周期并不只管 UAF，也管“别名 + 可变性”安全。

---

## 6. 生命周期省略（Elision）规则：写少但语义没少

编译器常用三条规则：

1. 每个输入引用参数获得独立生命周期参数；
2. 若只有一个输入引用参数，输出引用默认继承它；
3. 若是方法且有 `&self`/`&mut self`，输出引用默认绑定到 `self`。

### 6.1 对照示例

显式写法：

```rust
fn head<'a>(s: &'a str) -> &'a str { &s[..1] }
```

省略写法：

```rust
fn head(s: &str) -> &str { &s[..1] }
```

方法中的 `self` 绑定：

```rust
struct Holder {
    data: String,
}

impl Holder {
    fn as_str(&self) -> &str {
        &self.data
    }
}
```

`as_str` 的输出自动绑定到 `&self` 的借用期。

---

## 7. 结构体中带引用：为什么要 `struct X<'a>`

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```

可直译成：

- `ImportantExcerpt` 持有的是“借来的视图”，不是拥有数据；
- 该实例绝不能活得比 `part` 指向的数据更久。

这是 Rust 对 C++ “类里存裸指针/引用成员”场景的强约束版本：  
不靠 code review，靠类型系统阻断悬挂对象。

---

## 8. NLL：为什么你有时感觉“编译器变聪明了”

Rust 采用 Non-Lexical Lifetimes（NLL）：  
借用通常在“最后一次使用”后就结束，而不是必须等到花括号结束。

```rust
fn main() {
    let mut s = String::from("abc");
    let r = &s;
    println!("{r}"); // r 最后一次使用

    let m = &mut s;  // NLL 下这里通常允许
    m.push('d');
}
```

这能减少“明明安全却报错”的情况，但不改变核心借用规则。

---

## 9. 报生命周期错时的工程化排查顺序

1. **先定位返回引用借自谁**：返回值必须有明确来源；
2. **再看作用域是否重叠**：被借用值是否活得足够久；
3. **再评估是否该返回拥有值**：`String/Vec<T>` 往往更稳定；
4. **最后再加生命周期参数**：参数是“表达关系”，不是“修复魔法”。

---

## 10. 什么时候该果断放弃借用，改返回拥有值

| 场景 | 建议 |
| --- | --- |
| 跨线程、跨层缓存、长期持有 | 优先拥有值（`String` / `Arc<T>`） |
| 接口因 lifetimes 明显难读 | 先拥有值，后续按性能热点回退借用 |
| 热路径且零拷贝收益明确 | 再设计借用 API（`&str` / `&[u8]`） |

对 C++ 老手尤其重要：  
不要一开始就“引用优化到位”，先把 Rust 的借用图画清楚。

---

## 11. 小结

- 生命周期不是析构控制，而是引用关系声明；
- 你不需要“实现生命周期”，只需要“证明引用关系正确”；
- 如果你能判断 C++ 引用何时悬挂，就能写出正确 Rust lifetime；
- 写不通时先用拥有语义跑通，再按热点优化成借用语义。

---

## 12. 配套代码

对应示例：[`../src/bin/03_lifetime.rs`](../src/bin/03_lifetime.rs)。

- `pick_first`：显式生命周期关联输入与输出；
- `pick_first_elided`：省略规则等价写法；
- `View<'a>`：结构体持引用；
- `choose_owned`：返回拥有值以降低接口复杂度。
