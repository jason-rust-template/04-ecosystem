# 错误处理
- anyhow
- thiserror
- snafu

# 日志处理：tracing-subscriber
- tracing: 记录各种日志
- tracing-subscriber: 输出日志
- open-telemetry*: 和 open-telemetry 生态互动
- 养成良好的 tracing/metrics习惯

- note:
- cat /tmp/logs/ecosystem.log.2024-06-26 命令用于在 Unix-like 系统中显示指定文件的内容。这里的 cat 是   "concatenate and display file contents" 的缩写，它将文件的内容输出到终端或另一个输出设备。

- tail -f /tmp/logs/ecosystem.log.2024-06-26
- tail -f 命令在 Unix-like 系统中用于显示文件的最后几行内容，并且会持续监听文件的变化，实时显示新写入的内容

# docker 启动 jeagertracing  本地日志打到线上去
- docker run -d -p16686:16686 -p4317:4317 COLLECTOR_OTLP_ENABLED=true jaegertracing all-in-one:latest

# 宏
- derive_builder derive_more strum
- derive-builder: 构建数据结构的 builder
- derive_more: 标准库 trait 的自动实现
- strum: enum 相关的 trait 的自动实现

# 数据转换: serde 生态
- serde 介绍
- serde 背后发生了什么
- serde-json / serde-yaml / toml / bincode / ...
- serde-with
- serde 使用的注意事项
