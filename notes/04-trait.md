# 04 - Struct / Enum / Trait

运行：

```bash
cargo run --bin 04_trait
```

## 1. 本课核心

- `struct`：承载数据字段。
- `enum`：承载离散状态，并要求匹配分支穷尽。
- `trait`：定义行为契约，类型通过 `impl` 实现该行为。

## 2. 结合当前示例看

- `Config` 是 `struct`，保存线程数和后端类型。
- `Backend` 是 `enum`，当前有 `Epoll` / `IoUring` 两个变体。
- `Describe` 是 `trait`，`Config` 通过 `impl Describe for Config` 提供具体行为。
- `print_desc(item: &impl Describe)` 表示参数只要实现了 `Describe` 即可。

## 3. 对照 Go

- `struct` 对应 Go 的 `struct`（都用于聚合字段）。
- `trait` 的使用体验接近 Go `interface`：
- 关注“行为能力”而不是继承树。
- Rust 里是显式 `impl Trait for Type`；Go 是方法集自动满足接口。
- Rust `enum` 比 Go 常见的“常量 + switch”更强：
- 枚举值可携带数据，`match` 强制穷尽。

## 4. 对照 Java

- Rust 的 `struct + impl` 可以类比 Java 的 `class + methods`（但 Rust 无传统继承层级）。
- Rust 的 `trait` 可类比 Java 的 `interface`。
- `impl Trait` 参数常对应“按接口编程”的思路。
- Rust 默认更偏静态分发（类似泛型实例化），Java 接口调用常见动态分派。
- Rust `enum` 能力明显强于 Java 早期 `enum` 常量模型，更接近“代数数据类型”。

## 5. 你可重点关注的迁移点

- 不要先找“class 继承”对应物，先用 `struct + trait` 建模。
- trait 在 Rust 里既是“接口”，也是泛型约束（这点比 Go/Java 更统一）。
- `enum + match` 是状态建模主力，能替代很多 if/else + magic number。

## 6. 后续方向

- 对比 `&impl Trait`（静态分发）和 `&dyn Trait`（动态分发）。
- 补一节对象安全（object safety）和性能模型。

## 7. 对比 C/C++ class 继承：为什么现代语言更偏这个方向

你关心的是“为什么不用传统继承树，而是 `struct + trait + enum`”。

### 7.1 传统继承的优势与问题

C++ class 继承的优势：

- 代码复用直接。
- 面向对象建模直观。
- 运行时多态成熟（虚函数体系）。

但在大型工程里常见问题：

- 层级变深后，语义耦合高，重构成本大。
- 基类演进容易影响全链路子类（脆弱基类问题）。
- 多重继承、虚继承、对象切片、生命周期管理增加复杂度。

### 7.2 Rust 路径：组合优先，能力抽象优先

Rust 倾向把“数据”和“行为能力”分开建模：

- `struct` 负责数据形状。
- `trait` 负责能力契约。
- `enum` 负责状态空间和分支穷尽。

这套模型的工程优势：

- 组合优先，减少继承树刚性耦合。
- trait 可做接口，也可做泛型约束，抽象层更统一。
- `enum + match` 让状态扩展和分支处理更显式。

### 7.3 为什么现代语言整体在往这个方向走

高层原因不是“语法流行”，而是工程问题驱动：

- 大型系统更重视可维护性和可演进性，而不是一次性继承设计的优雅。
- 组合式抽象对微服务、组件化、库生态更友好。
- 类型系统与接口约束逐渐成为“可验证设计”的核心手段。
- 并发与可靠性要求上升后，语言更倾向减少隐式共享和隐式行为。

### 7.4 给 C/C++ 系统程序员的落点

- class 继承不是“错”，它在局部模型里依然有效。
- 但在长期演进系统中，`struct + trait + enum` 通常更利于分层、替换和验证。
- 可以把 Rust 这套方式理解为：把 OO 的“多态能力”保留，把“继承耦合成本”尽量剥离。
