### 简介
这是一个实现了 falcon-agent 数据转发至 n9e-server 功能的 Rust 学习项目。项目使用了 axum 和 reqwest 这两个强大的 Rust 库，实现了 falcon-agent 数据转发至 n9e-server 的功能。
项目的初衷是为了解决 falcon-agent 在转发自定义监控时内存占用过高，以及在 SAAS 化环境下数据隔离的问题。可以通过环境变量和配置文件轻松地调整上报地址，以满足数据隔离的需求。

### 功能
- 兼容 falcon-agent 接口：http://127.0.0.1:1988/v1/push
- 支持 falcon 协议接口：http://127.0.0.1:1988/openfalcon/push
- 支持 opentsdb 协议接口：http://127.0.0.1:1988/opentsdb/put
- 支持 prometheus 协议接口：http://127.0.0.1:1988/prometheus/v1/write

### cargo
```text
cargo +nightly udeps
cargo build
cargo update
```