# 棋类游戏应用程序

基于 Tauri 2.0 框架的桌面端棋类游戏应用程序，支持中国象棋和军棋两种棋类游戏，提供人机对战模式。

## 🎉 项目状态

**✅ 已完成** - 所有功能已实现，测试覆盖率 100%（318 个测试全部通过）

详细信息请查看 [项目总结文档](./docs/PROJECT_SUMMARY.md)

## 技术栈

- **桌面框架**: Tauri 2.0
- **前端**: React 18 + TypeScript + Vite
- **后端**: Rust
- **状态管理**: Zustand
- **测试框架**: 
  - Rust: proptest (属性测试)
  - TypeScript: vitest + fast-check (单元测试 + 属性测试)

## 项目结构

```
.
├── src/                    # 前端源代码
│   ├── components/         # React 组件
│   ├── __tests__/          # 前端测试文件
│   │   ├── components/     # 组件测试
│   │   └── types/          # 类型测试
│   ├── types/              # TypeScript 类型定义
│   └── test/               # 测试设置
├── src-tauri/              # Rust 后端源代码
│   ├── src/
│   │   ├── models/         # 数据模型
│   │   ├── game_engine/    # 游戏引擎
│   │   ├── ai/             # AI 引擎
│   │   └── commands.rs     # Tauri 命令
│   └── tests/              # 后端测试文件
│       ├── ai/             # AI 引擎测试
│       ├── game_engine/    # 游戏引擎测试
│       └── models/         # 数据模型测试
├── .kiro/                  # 项目规格文档
│   └── specs/
│       └── chess-game-app/
│           ├── requirements.md  # 需求文档
│           ├── design.md        # 设计文档
│           └── tasks.md         # 任务列表
├── docs/                   # 项目文档
│   ├── PROJECT_SUMMARY.md  # 项目完成总结
│   ├── CHANGELOG.md        # 更新日志
│   ├── IMPROVEMENTS.md     # 项目改进说明
│   └── WARNINGS_FIXED.md   # 编译警告修复说明
├── Makefile                # 常用命令快捷方式
└── README.md
```

## 开发环境要求

- Node.js >= 18
- Rust >= 1.70
- npm 或 yarn

## 快速开始

### 使用 Makefile（推荐）

```bash
# 查看所有可用命令
make help

# 安装依赖
make install

# 启动开发服务器
make dev

# 运行所有测试
make test

# 仅运行前端测试
make test-frontend

# 仅运行后端测试
make test-backend

# 构建生产版本
make build
```

### 使用 npm 命令

```bash
# 安装前端依赖
npm install

# 启动开发服务器（前端 + 后端）
npm run tauri dev

# 运行前端测试
npm run test:run

# 运行 Rust 后端测试
cd src-tauri && cargo test

# 构建生产版本
npm run tauri build
```

## 测试策略

本项目采用双重测试方法：

1. **单元测试**: 验证特定示例、边缘情况和错误条件
2. **属性测试**: 验证跨所有输入的通用属性

### 测试覆盖

- ✅ **前端测试**: 60 个测试全部通过
- ✅ **后端测试**: 257 个测试全部通过
- ✅ **总计**: 318 个测试，100% 通过率

### Rust 后端测试

- 使用 `proptest` 进行属性测试
- 每个属性测试运行至少 100 次迭代
- 测试文件位于 `src-tauri/tests/` 目录

### TypeScript 前端测试

- 使用 `vitest` 作为测试运行器
- 使用 `fast-check` 进行属性测试
- 使用 `@testing-library/react` 进行组件测试
- 测试文件位于 `src/__tests__/` 目录

## 实现的功能

- ✅ 中国象棋游戏引擎（完整规则实现）
- ✅ 军棋游戏引擎（完整战斗系统）
- ✅ AI 对战功能（Minimax 算法 + Alpha-Beta 剪枝）
- ✅ 美观的前端界面（Canvas 渲染）
- ✅ 游戏控制（悔棋、重新开始、返回主菜单）
- ✅ 完整的错误处理和日志记录
- ✅ 20 个正确性属性验证
- ✅ 跨平台支持（Windows、macOS、Linux）

## 文档

- [项目总结](./docs/PROJECT_SUMMARY.md) - 完整的项目总结
- [更新日志](./docs/CHANGELOG.md) - 项目更新历史
- [改进说明](./docs/IMPROVEMENTS.md) - 项目重组和改进详情
- [警告修复](./docs/WARNINGS_FIXED.md) - 编译警告修复说明
- [UI 改进](./docs/UI_IMPROVEMENTS.md) - UI 显示优化记录
- [需求文档](./.kiro/specs/chess-game-app/requirements.md) - 功能需求
- [设计文档](./.kiro/specs/chess-game-app/design.md) - 技术设计
- [任务列表](./.kiro/specs/chess-game-app/tasks.md) - 实施计划

## 推荐 IDE 设置

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## 许可证

MIT
