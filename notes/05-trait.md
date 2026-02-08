# 05 - Struct / Enum / Trait

运行：

```bash
cargo run --bin 05_trait
```

## 1. 本课核心

- `struct`：承载数据字段。
- `enum`：承载离散状态，并要求匹配分支穷尽。
- `trait`：定义行为契约，类型通过 `impl` 实现该行为。

### 1.1 trait 的概念、起源与常见使用领域

- 概念：trait 本质是“能力接口（behavior contract）”，关注类型“能做什么”，而非“继承自谁”。
- 起源脉络：来自函数式语言里的 type class 思想（如 Haskell），后来被 Rust 吸收并与系统编程需求结合。
- 常见语言对应：
  - Rust `trait` 接近 Go `interface` 的“按行为抽象”思路。
  - 在 Java/C# 语境里可类比 `interface`，但 Rust trait 同时承担泛型约束角色。
- 常见领域：
  - 标准库抽象（`Iterator`、`Read`/`Write`、`Display`/`Debug`）。
  - 基础设施与中间件（驱动接口、插件系统、协议适配层）。
  - 泛型算法库（通过 trait 约束复用算法而不依赖继承树）。

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

## 5. 重点关注的迁移点

- 不建议先寻找“class 继承”对应物，建议先用 `struct + trait` 建模。
- trait 在 Rust 里既是“接口”，也是泛型约束（这点比 Go/Java 更统一）。
- `enum + match` 是状态建模主力，能替代很多 if/else + magic number。

## 6. 扩展学习方向

- 对比 `&impl Trait`（静态分发）和 `&dyn Trait`（动态分发）。
- 补一节对象安全（object safety）和性能模型。

## 6.1 Trait 进阶

### 对象安全 Object Safety

- 只有对象安全 trait 才能做 `dyn Trait`。
- 常见不满足对象安全的情况：
- 方法返回 `Self`。
- 方法是泛型方法（`fn f<T>(...)`）。

结论：

- 想做运行时多态（`dyn Trait`）时，先检查 trait 是否对象安全。

### 关联类型 Associated Type

- 关联类型常用于表达“实现者决定具体类型”。
- 和泛型参数相比，关联类型通常让接口更稳定、可读性更好。

典型对比：

- 泛型版本：`trait Parser<T> { ... }`
- 关联类型版本：`trait Parser { type Output; ... }`

### `impl Trait` vs `dyn Trait` 再强调

| 方式 | 分发 | 成本模型 | 适用场景 |
| --- | --- | --- | --- |
| `impl Trait` | 静态分发 | 单态化，通常更利于内联优化 | 热路径、性能敏感 |
| `dyn Trait` | 动态分发 | vtable 间接调用 | 插件化、运行时多态 |

### 工程实践建议

- 默认采用 `impl Trait`（性能与实现复杂度更平衡）。
- 确认需要运行时可替换能力时再引入 `dyn Trait`。

## 7. 对比 C/C++ class 继承：为什么现代语言更偏这个方向

常见关注点是“为什么不用传统继承树，而是 `struct + trait + enum`”。

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

## 8. 配套代码

对应示例：[`../src/bin/05_trait.rs`](../src/bin/05_trait.rs)。

- `print_desc_static(&impl Describe)`：静态分发示例。
- `print_desc_dynamic(&dyn Describe)`：动态分发示例。
- `trait ValueType { type Output; ... }`：关联类型示例。
- `trait ChunkSource { type Chunk<'a>; ... }`：GAT（泛型关联类型）最小示例。
- 文件末尾注释里有对象安全失败示例（不参与编译）。

## 9. 更深一层：blanket impl / orphan rule / coherence

### 9.1 blanket impl 批量实现

- 指“为一类满足约束的类型统一实现 trait”。
- 常见形式：`impl<T: Bound> Trait for T`。
- 优势：减少样板，实现可复用能力注入。

### 9.2 orphan rule 孤儿规则

- 不能“给外部 trait 实现到外部类型”上。
- 至少 trait 或类型有一方必须是本 crate 定义。
- 目的：避免跨 crate 冲突实现。

### 9.3 coherence 一致性

- 编译器要求“某个类型的某个 trait 实现必须唯一可判定”。
- 防止出现多个同样匹配的实现导致歧义。

## 10. 默认方法与扩展 trait

- trait 可给方法提供默认实现，减少重复代码。
- 扩展 trait 常用于给已有类型追加“领域便捷方法”。
- 这是 Rust 里替代“继承工具类”的常见方式。

## 11. trait object 还是 enum + match

| 方案 | 优势 | 代价 |
| --- | --- | --- |
| `dyn Trait` | 运行时可扩展，插件化友好 | 动态分发开销、对象安全限制 |
| `enum + match` | 穷尽分支、静态优化更好 | 新增变体需要改调用方匹配 |

经验：

- 已知有限类型集合时，通常采用 `enum + match`。
- 需要运行时扩展时，通常采用 `dyn Trait`。
