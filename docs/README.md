# chess-game 文档索引

本目录存放象棋（中国象棋）与军棋的人机对战桌面端产品文档，技术栈目标为 **Tauri 2.0**。

## 文档列表

| 文档 | 说明 |
|------|------|
| [requirements.md](./requirements.md) | 产品与功能需求、非功能需求、里程碑 |
| [architecture.md](./architecture.md) | 系统架构、模块划分、数据流、Tauri 边界 |
| [tech-decisions.md](./tech-decisions.md) | 关键技术选型与「是否先脚手架」结论 |

## 建议阅读顺序

1. `tech-decisions.md` — 明确初始化时机与选型约束  
2. `requirements.md` — 明确「做什么」  
3. `architecture.md` — 明确「怎么分层实现」  

## 与代码仓库的关系

当前仓库仅有占位 `README.md`。**文档先行，脚手架后置**：待需求与架构确认后，再用 Tauri 2 CLI 初始化工程，并按本文档目录约定落地代码。
