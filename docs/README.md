# 项目文档目录 / chess-game 文档索引

本目录包含棋类游戏应用程序的所有项目文档。针对本仓库的象棋（中国象棋）与军棋人机对战桌面端产品，技术栈目标为 **Tauri 2.0**，文档先行、脚手架后置。

## 📚 文档列表

### 项目级文档
- [PROJECT_SUMMARY.md](./PROJECT_SUMMARY.md) — 项目完成总结（状态、测试、功能、架构、使用说明）
- [CHANGELOG.md](./CHANGELOG.md) — 更新日志
- [IMPROVEMENTS.md](./IMPROVEMENTS.md) — 项目改进说明
- [WARNINGS_FIXED.md](./WARNINGS_FIXED.md) — 编译警告修复说明
- [UI_IMPROVEMENTS.md](./UI_IMPROVEMENTS.md) — UI 改进记录（尺寸调整、楚河汉界、军棋棋盘等）

### 规格与技术文档
| 文档 | 说明 |
|------|------|
| [requirements.md](./requirements.md) | 产品与功能需求、非功能需求、里程碑 |
| [architecture.md](./architecture.md) | 系统架构、模块划分、数据流、Tauri 边界 |
| [tech-decisions.md](./tech-decisions.md) | 关键技术选型与「是否先脚手架」结论 |

## 🔗 相关文档位置
### 规格文档
位于 `.kiro/specs/chess-game-app/` 目录：
- `requirements.md` - 功能需求文档
- `design.md` - 技术设计文档
- `tasks.md` - 实施计划和任务列表

### 根目录文档
- `README.md` - 项目主文档
- `Makefile` - 常用命令快捷方式

## 📖 建议阅读顺序
1. README.md（根目录）- 快速了解项目
2. PROJECT_SUMMARY.md - 项目完成情况与高层总结
3. requirements.md - 明确要实现的功能与约束
4. design.md / architecture.md - 技术设计与分层实现
5. tech-decisions.md - 初始化与选型约束
6. IMPROVEMENTS.md / UI_IMPROVEMENTS.md - 改进与优化记录

## 🛠️ 维护说明
### 添加新文档
新的项目文档应放在本目录下，并更新本 README.md 文件。

### 文档命名规范
- 使用大写字母和下划线：`PROJECT_SUMMARY.md`
- 使用描述性的名称，保持简洁明了

### 文档更新
当项目有重大更新时：
1. 更新相关文档
2. 在 `CHANGELOG.md` 中记录变更
3. 如有必要，更新根目录的 `README.md`

## 📝 文档格式
所有文档使用 Markdown 格式，遵循以下规范：
- 使用中文（简体）为主，必要处保留英文代码或术语
- 清晰的标题层级
- 适当的代码块和列表
- 必要的链接和引用

## 与代码仓库的关系
当前仓库中文档与代码的关系保持明确：文档先行，待需求与架构确认后再用 Tauri CLI 按约定落地工程与代码结构。
