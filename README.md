# 棋类游戏应用程序

基于 Tauri 2.0 框架的桌面端棋类游戏应用程序，支持中国象棋和军棋两种棋类游戏，提供人机对战模式。

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
│   ├── hooks/             # 自定义 React Hooks
│   ├── types/             # TypeScript 类型定义
│   ├── utils/             # 工具函数
│   └── test/              # 测试设置
├── src-tauri/             # Rust 后端源代码
│   └── src/
│       ├── models/        # 数据模型
│       ├── game_engine/   # 游戏引擎
│       ├── ai/            # AI 引擎
│       └── commands.rs    # Tauri 命令
├── .kiro/                 # 项目规格文档
│   └── specs/
│       └── chess-game-app/
│           ├── requirements.md  # 需求文档
│           ├── design.md        # 设计文档
│           └── tasks.md         # 任务列表
└── README.md
```

## 开发环境要求

- Node.js >= 18
- Rust >= 1.70
- npm 或 yarn

## 安装依赖

```bash
# 安装前端依赖
npm install

# Rust 依赖会在构建时自动安装
```

## 开发命令

```bash
# 启动开发服务器（前端 + 后端）
npm run tauri dev

# 仅启动前端开发服务器
npm run dev

# 运行前端测试
npm run test

# 运行前端测试（UI 模式）
npm run test:ui

# 运行前端测试（单次运行）
npm run test:run

# 运行 Rust 后端测试
cd src-tauri && cargo test

# 构建生产版本
npm run build
npm run tauri build
```

## 测试策略

本项目采用双重测试方法：

1. **单元测试**: 验证特定示例、边缘情况和错误条件
2. **属性测试**: 验证跨所有输入的通用属性

### Rust 后端测试

- 使用 `proptest` 进行属性测试
- 每个属性测试运行至少 100 次迭代
- 测试文件位于各模块的 `tests` 子模块中

### TypeScript 前端测试

- 使用 `vitest` 作为测试运行器
- 使用 `fast-check` 进行属性测试
- 使用 `@testing-library/react` 进行组件测试

## 当前状态

✅ 任务 1: 项目初始化和基础架构 - 已完成

- Tauri 2.0 项目已创建
- 前端和后端目录结构已设置
- 测试框架已配置（proptest + vitest + fast-check）
- 基础数据模型已定义

## 下一步

- 任务 2: 实现核心数据模型
- 任务 3: 实现中国象棋游戏引擎

## 推荐 IDE 设置

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## 许可证

MIT
