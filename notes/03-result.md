# 03 - Result 与错误处理

运行：

```bash
cargo run --bin 03_result
```

## 1. `Result<T, E>` 到底是什么

先回答你的核心问题：

- `Result` 不是关键字（keyword）。
- `Result` 是标准库里的通用枚举类型（在 prelude 里，默认可直接用）。
- 形态是：`Result<T, E>`，其中：
- `T` = 成功值类型（`Ok(T)`）
- `E` = 错误类型（`Err(E)`）

可以把它理解为“必须被处理的成功/失败联合体”。

## 2. `<T, E>` 尖括号语法是什么

- 这是 Rust 泛型参数语法。
- 和 C++ 模板有强相似性：都是“把类型当参数”。
- 你可以类比：
- Rust `Result<i32, AppError>`
- C++ `Result<int, AppError>`（概念类比）

差异（先记一条就够）：

- Rust 泛型是语言核心 + trait 约束体系，不是头文件模板元编程那套语法风格。

## 3. `?` 运算符到底做了什么

在返回 `Result<..., ...>` 的函数里，`expr?` 等价于：

- 若 `expr` 是 `Ok(v)`，取出 `v` 继续执行。
- 若 `expr` 是 `Err(e)`，立刻 `return Err(...)`。

在本例里：

- `raw.parse()?` 若解析失败，会提前返回错误。
- 因为实现了 `From<ParseIntError> for AppError`，错误会自动转换成 `AppError::Parse`。

你可把 `?` 看成“错误路径的早返回语法糖”，减少样板代码。

## 4. 结合本例看错误链

`parse_validate_and_double` 里有三个阶段：

- `parse_i32(raw)?`：字符串转整数
- `require_positive(n)?`：要求大于 0
- `require_even(n)?`：要求是偶数

任意阶段失败，函数立即返回 `Err`；全部通过才 `Ok(n * 2)`。

## 5. 对照 C/C++ 认知

- 类似“返回值 + 错误码”的强化版本，但由类型系统强制你处理。
- 相比异常机制，这种方式让错误路径在签名上可见（`-> Result<T, E>`）。
- 相比手写 `if (rc != 0)` 链，`?` 让传播路径简洁很多。

## 6. 工程建议（后续会加）

- 应用层可用 `anyhow` 聚合上下文。
- 库层可用 `thiserror` 定义精确错误类型。
