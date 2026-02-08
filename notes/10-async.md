# 10 - 异步编程与 IO 密集型策略（对照 Go / JS / C++）

本章回答你这个核心问题：

- Rust 协程模型更像 Go、JS，还是 C++？
- IO 密集型场景下，Rust 的最佳策略是什么？

## 1. 先给结论

- Rust 的 `async/await` 不是 Go 的“语言内置协程 + 调度器”。
- Rust 也不是 JS 那种“默认单线程事件循环 + Promise 微任务”模型。
- Rust 更像“语言支持 async 状态机 + 运行时由库提供（如 tokio）”。

一句话：

- Rust 有协程能力，但执行模型是“可选择的 runtime 方案”，不是语言强绑单一运行时。

## 2. Rust 协程到底是什么

- `async fn` 会被编译成一个 `Future` 状态机。
- `await` 是状态机挂起点（yield point）。
- 真正调度这些 `Future` 的是 executor/runtime（例如 `tokio`、`async-std`）。

这意味着：

- 语言负责“怎么表达异步”。
- 运行时库负责“怎么调度执行”。

## 3. IO 密集型场景的推荐策略（Rust）

| 场景 | 推荐策略 | 备注 |
| --- | --- | --- |
| 高并发网络服务 | `tokio` + async socket/IO | 适合连接数高、单请求 CPU 占比低 |
| 混合任务（IO + 少量 CPU） | async 主流程 + `spawn_blocking` 隔离阻塞 | 避免阻塞 reactor/worker |
| 重 CPU 计算 | 独立线程池（如 rayon）或专门 worker | 不要把 CPU 热点硬塞进 async task |
| 低延迟系统组件 | 控制任务粒度与分配次数 | 减少上下文切换与内存抖动 |

## 4. 对比 Go / JS / C++

| 语言 | 协程/异步模型 | 调度模型 | IO 密集体验 |
| --- | --- | --- | --- |
| Go | goroutine（语言级） | runtime 内置 M:N 调度 + netpoll | 开箱即用，非常顺手 |
| JS/Node.js | Promise + event loop | 通常单线程主循环 + 事件队列 | IO 并发强，但 CPU 重任务需额外手段 |
| C++20 | coroutine（语言机制） | 无统一官方 runtime，生态分散 | 能做但工程统一性较弱 |
| Rust | `async/await` + `Future` | runtime 由库提供（tokio 等） | 工程化成熟，控制力与性能平衡较好 |

## 5. Rust 和 Go/JS 的关键差异

### 5.1 对比 Go

- Go：语言+runtime 一体，开发体验统一。
- Rust：语言 + 可替换 runtime，灵活但需要更明确工程选择。

收益与代价：

- Rust 在系统级可控性（无强制 GC runtime）更强。
- 但团队需要理解 executor、阻塞隔离、任务边界等细节。

### 5.2 对比 JS

- JS 心智：Promise 链 + 单线程事件循环。
- Rust 心智：`Future` 状态机 + 多线程 runtime（常见配置）+ 显式非阻塞 API。

收益与代价：

- Rust 不受“主线程单点”约束，吞吐与资源控制更强。
- 但需要更严格区分 async 安全与阻塞代码。

### 5.3 对比 C++

- C++20 有协程语法，但 runtime 和生态分层更碎。
- Rust 的 async 生态（tokio、tracing、tower、hyper 等）在工程协作上更一致。

## 6. 常见误区（Rust async）

- 误区 1：async 天生更快。
- 实际：只有 IO 等待占比高时收益明显；CPU 密集可能更慢。

- 误区 2：把阻塞调用放进 async 没关系。
- 实际：会卡住 worker，吞吐显著下降。

- 误区 3：任务越细越好。
- 实际：任务过细会增加调度与分配开销。

## 7. 对系统程序员的落地建议

- 第一阶段：先用同步实现把协议和数据结构做正确。
- 第二阶段：只把 IO 边界 async 化（socket、timer、channel）。
- 第三阶段：明确标注阻塞段，统一走 `spawn_blocking` 或专门线程池。
- 第四阶段：用指标驱动优化（P99 延迟、吞吐、上下文切换、分配次数）。

一句话：

- Rust 协程在 IO 密集场景非常有竞争力，但关键不是“写成 async”，而是“写成可调度、可隔离、可观测的 async”。
