# 04 - Struct / Enum / Trait

运行：

```bash
cargo run --bin 04_trait
```

知识点：
- `struct` 承载数据。
- `enum` 承载离散状态并强制穷尽处理。
- `trait` 类似“行为接口”，通过 `impl` 给类型实现能力。

对照 C++：
- trait 像接口 + 泛型约束的组合，不等同于传统虚函数体系。
- `impl Trait` 参数常用于静态分发（单态化），性能模型接近模板。

后续方向：
- 再对比 `dyn Trait`（动态分发）和对象安全。
- 增加模块化拆分（`mod` + 多文件）。
