# 18 - 常用标准库速览

运行：

```bash
cargo run --bin 18_stdlib
```

## 1. Rust 标准库和 STL 的关系

- Rust 标准库可以类比 C++ STL + 一部分语言配套运行时接口。
- 两者都提供容器、字符串、迭代、排序、时间、I/O 等基础能力。
- 差异在于：Rust API 和所有权/借用模型深度绑定，类型约束更强。

## 2. 常见类型对标

| Rust | C++ STL 对标 | 常见用途 |
| --- | --- | --- |
| `Vec<T>` | `std::vector<T>` | 动态数组、顺序存储 |
| `String` | `std::string` | 拥有所有权的可变字符串 |
| `&str` | `std::string_view`（语义近似） | 借用字符串切片 |
| `HashMap<K, V>` | `std::unordered_map<K, V>` | 哈希表 |
| `BTreeMap<K, V>` | `std::map<K, V>` | 有序映射 |
| `HashSet<T>` | `std::unordered_set<T>` | 哈希集合 |
| `BTreeSet<T>` | `std::set<T>` | 有序集合 |
| `VecDeque<T>` | `std::deque<T>` | 双端队列 |
| `BinaryHeap<T>` | `std::priority_queue<T>` | 堆与优先队列 |
| `Option<T>` | `std::optional<T>` | 可空值建模 |
| `Result<T, E>` | 近似 `expected<T, E>` 思路 | 错误处理 |

## 3. 字符串常用方法

`String` 常用：

- `push_str`：追加字符串
- `replace`：替换子串
- `split_whitespace`：按空白切分
- `trim`：去掉首尾空白
- `parse::<T>()`：解析为目标类型

`&str` 常用：

- `len`、`is_empty`
- `starts_with`、`ends_with`
- `contains`
- `find`

## 4. 排序常用方法

- `sort()`：稳定排序（需要元素实现 `Ord`）
- `sort_by(...)`：自定义比较器
- `sort_by_key(...)`：按 key 排序
- `sort_unstable()`：不稳定排序，通常更快

## 5. 迭代器风格（和 STL 算法思路对照）

- Rust 倾向 `iter().map().filter().collect()` 管道式写法。
- 和 STL 算法库思想接近，但 Rust 写法更统一在迭代器 trait 上。

## 6. 配套代码

对应示例：[`../src/bin/18_stdlib.rs`](../src/bin/18_stdlib.rs)。

- `string_demo`：字符串常用方法。
- `sort_demo`：稳定排序、自定义排序、按 key 排序。
- `map_set_demo`：`HashMap` 与 `HashSet` 基本操作。

## 7. 小结

- 相比 C，Rust 的字符串操作在安全性与易用性上通常是显著提升。
- 相比 C++，Rust 在默认边界检查、所有权语义和 API 一致性上常更直接，学习曲线更平滑。
- 工程上可用一句话概括：字符串日常开发效率通常高于 C，很多场景也会比 C++ 更省心。
