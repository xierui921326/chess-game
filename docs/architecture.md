# 架构文档：人机象棋 & 军棋（Tauri 2.0）

| 项 | 内容 |
|----|------|
| 版本 | v0.1.1 |
| 日期 | 2026-08-13 |
| 关联 | [requirements.md](./requirements.md)、[tech-decisions.md](./tech-decisions.md) |

---

## 1. 设计原则

1. **裁判在 Rust**：局面、合法着法、终局判定只信后端；前端是视图与输入。  
2. **棋种插件化**：象棋 / 军棋共享「对局会话」抽象，规则与 AI 各自独立 crate/模块。  
3. **AI 不阻塞 UI**：搜索在异步任务中执行，通过事件推送进度/结果。  
4. **先薄壳后厚引擎**：M0 脚手架只保证 IPC 通路；规则与 AI 逐步加厚。  
5. **文档驱动目录**：初始化时按本文目录约定放置代码，避免后期大搬迁。

---

## 2. 逻辑架构

```text
┌─────────────────────────────────────────────────────────┐
│                     Frontend (WebView)                   │
│  路由/页面 · 棋盘组件 · 交互状态 · 主题/设置 UI           │
└──────────────────────┬──────────────────────────────────┘
                       │ Tauri IPC
                       │ invoke(commands) / listen(events)
┌──────────────────────▼──────────────────────────────────┐
│                   Tauri App Shell (Rust)                 │
│  commands · events · AppState · 取消令牌 · 持久化(可选)   │
└───────────┬─────────────────────────────┬───────────────┘
            │                             │
   ┌────────▼────────┐           ┌────────▼────────┐
   │  game-session    │           │   settings      │
   │  统一对局生命周期  │           │   本地配置       │
   └────────┬────────┘           └─────────────────┘
            │
     ┌──────┴──────┐
     │             │
┌────▼────┐   ┌────▼────┐
│ xiangqi │   │  junqi  │
│ rules   │   │ rules   │
│ + AI    │   │ + AI    │
└─────────┘   └─────────┘
```

### 分层职责

| 层 | 职责 | 不负责 |
|----|------|--------|
| Frontend | 渲染、动画、点击/拖拽、本地 UI 状态 | 最终合法性裁决、AI 搜索 |
| Tauri Shell | 注册命令、管理 `AppState`、转发事件、任务取消 | 具体棋规细节 |
| game-session | 对局创建/走子/悔棋/终局状态机 | 某一棋种的走法生成细节 |
| xiangqi / junqi | 规则、着法生成、评估、搜索 | UI、窗口管理 |

---

## 3. 推荐仓库结构（脚手架后目标形态）

```text
chess-game/
├── docs/                          # 本目录
├── package.json                   # 前端包管理（初始化时生成）
├── src/                           # 前端源码
│   ├── main.ts(x)
│   ├── App.tsx
│   ├── pages/                     # 主菜单、对局页、结果页
│   ├── components/
│   │   ├── board/                 # 通用棋盘壳（尺寸/坐标映射）
│   │   ├── xiangqi/
│   │   └── junqi/
│   ├── lib/tauri/                 # invoke/listen 封装
│   └── styles/
├── src-tauri/                     # Tauri 2 Rust 工程
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── capabilities/              # Tauri 2 权限模型
│   └── src/
│       ├── main.rs / lib.rs
│       ├── commands/              # #[tauri::command]
│       ├── state.rs               # AppState
│       └── error.rs
├── crates/                        # 可选：工作区成员
│   ├── game-core/                 # 通用类型：Side、Move、GameStatus…
│   ├── xiangqi/                   # 象棋规则 + AI
│   └── junqi/                     # 军棋规则 + AI
└── README.md
```

> **说明**：若希望更简单，可将 `xiangqi` / `junqi` 先放在 `src-tauri/src/games/` 下，M2 后再拆 workspace crates。架构上仍保持模块边界。

---

## 4. Tauri 2 边界设计

### 4.1 Commands（前端 → Rust）

建议最小命令集：

| Command | 说明 |
|---------|------|
| `list_games` | 返回支持的棋种元数据 |
| `new_game` | `{ game, side, difficulty }` → 初始 `GameView` |
| `legal_moves` | `{ from? }` → 合法着法列表（用于高亮） |
| `apply_move` | 玩家着法 → 新局面；若轮到 AI 则触发异步思考 |
| `undo` | 悔棋 |
| `resign` | 认输 |
| `get_state` | 拉取当前 `GameView` |
| `cancel_ai` | 取消进行中的 AI 任务 |

所有写操作返回统一 `Result<GameView, AppError>`，错误含可读 `code` / `message`。

### 4.2 Events（Rust → 前端）

| Event | 载荷 | 用途 |
|-------|------|------|
| `ai://thinking` | `{ gameId, elapsedMs? }` | 显示思考中 |
| `ai://move` | `{ gameId, move, view }` | AI 落子完成 |
| `ai://cancelled` | `{ gameId }` | 思考被取消 |
| `game://ended` | `{ gameId, result }` | 终局（也可仅靠 command 返回） |

### 4.3 状态存放

- **权威状态**：Rust `AppState` 内 `HashMap<GameId, Box<dyn GameSession>>` 或枚举会话。  
- **前端状态**：仅缓存最近一次 `GameView` 用于渲染；以 command/event 刷新为准。  
- **持久化**：MVP 可不落盘；设置可用 `tauri-plugin-store`（二期）。

### 4.4 Capabilities（Tauri 2）

遵循最小权限：默认只开放本应用所需的 IPC；文件系统、对话框等插件按需在 `capabilities` 中声明。

---

## 5. 核心领域模型（共享）

```text
GameKind        = Xiangqi | JunqiDark | …(未来)
Side            = Red | Black | (军棋双方命名)
Difficulty      = Easy | Normal | Hard
Move            = 棋种相关结构（坐标 + 可选额外字段）
GameStatus      = WaitingHuman | WaitingAI | Ended { result }
GameView        = 给前端的 DTO（棋盘、轮次、合法提示所需元数据、最后着法）
```

### 会话接口（概念）

```rust
trait GameSession: Send {
    fn kind(&self) -> GameKind;
    fn view(&self) -> GameView;
    fn legal_moves(&self, filter: MoveFilter) -> Vec<MoveDto>;
    fn apply_human_move(&mut self, mv: MoveDto) -> Result<(), RuleError>;
    fn ai_move(&mut self, cfg: AiConfig, cancel: CancelToken) -> Result<MoveDto, AiError>;
    fn undo(&mut self) -> Result<(), RuleError>;
    fn resign(&mut self, side: Side);
}
```

象棋与军棋各自实现该 trait（或等价 enum dispatch），避免前端感知差异过大：DTO 用 `serde` 序列化，棋盘用「格子数组 + 棋子枚举」表达。

---

## 6. 象棋子系统

```text
xiangqi/
  board.rs      # 位棋盘或 90 格数组
  piece.rs
  movegen.rs    # 走法生成
  rules.rs      # 将军检测、合法性过滤
  zobrist.rs    # 可选：重复局面
  eval.rs
  search.rs     # alpha-beta / iterative deepening
  fen.rs        # 可选：局面导入导出
```

**数据流（玩家走子）**

```text
UI 选子 → legal_moves → 高亮
UI 落子 → apply_move（Rust 校验）→ GameView
若轮到 AI → spawn 搜索任务 → emit ai://move → UI 更新
```

---

## 7. 军棋子系统（翻棋 MVP）

```text
junqi/
  board.rs
  piece.rs      # 军衔、可见性（隐藏/公开）
  rules.rs      # 翻子、移动、碰子表
  fog.rs        # 信息不对称：AI 可见信息视图
  eval.rs
  search.rs     # 启发式 / 有限深度期望搜索
```

与象棋差异点：

- 状态含「未知棋子」；AI 只能基于信念状态决策。  
- `GameView` 对玩家侧隐藏对方未翻棋子细节。  
- 碰子结果导致双子移除或单方存活，需在规则表配置化，便于调规则。

---

## 8. 前端架构要点

- **路由**：`/` 主菜单 → `/play/xiangqi` → `/play/junqi`  
- **组件**：通用 `BoardShell`（坐标转换、响应式缩放）+ 棋种皮肤。  
- **交互状态机**：`Idle → Selected → AwaitingServer → OpponentThinking → Ended`  
- **技术选型**：见 `tech-decisions.md`（建议 React + TypeScript + Vite，与 Tauri 官方模板一致）。  
- **样式**：棋盘资源放 `public/assets/{xiangqi,junqi}/`，避免与逻辑耦合。

---

## 9. 并发与取消

```text
apply_move 发现轮到 AI
  → 为该 gameId 创建 CancellationToken
  → tauri::async_runtime::spawn
  → 搜索循环检查 token
  → 完成则 emit；若 new_game/resign/cancel_ai 则 cancel
```

同一 `gameId` 同时只允许一个 AI 任务；新任务启动前取消旧任务。

---

## 10. 错误模型

| code | 含义 |
|------|------|
| `INVALID_MOVE` | 非法着法 |
| `NOT_YOUR_TURN` | 未轮到玩家 |
| `GAME_NOT_FOUND` | 会话不存在 |
| `GAME_ENDED` | 已终局 |
| `AI_CANCELLED` | 思考取消 |
| `INTERNAL` | 未预期错误 |

前端按 `code` 做 UX，不依赖英文 `message` 做分支。

---

## 11. 测试策略

| 层级 | 内容 |
|------|------|
| 单元测试 | `xiangqi` / `junqi` 规则与走法（Rust） |
| 属性/快照 | 关键局面 FEN/自定义格式回归 |
| 集成 | command 层：new → move → ai → end |
| 手工 | `tauri dev` 走完一局验收清单 |

前端以组件测试为辅，不重复测规则。

---

## 12. 安全与反作弊（单机语境）

单机人机仍建议：

- 所有 `apply_move` 经 Rust 校验  
- 不提供「直接 set_board」类命令给生产前端  
- 调试命令用 `#[cfg(debug_assertions)]` 或独立 feature 门控  

---

## 13. 演进路线

1. **M0**：Tauri 2 初始化 + 空 `ping` command 打通 IPC  
2. **M1**：象棋规则 + `GameView` + 简易 UI  
3. **M2**：象棋 AI + 事件 + 悔棋  
4. **M3**：军棋模块并行接入同一 session 框架  
5. **M4**：拆 crates、设置持久化、安装包  

首版仅实现 `JunqiDark`（翻棋，已确认）。传统军棋（布局阶段）作为远期 `GameKind::JunqiClassic` 挂到同一 `GameSession`，无需改壳层。
