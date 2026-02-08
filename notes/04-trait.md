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
