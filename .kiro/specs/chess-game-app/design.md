# 设计文档

## 概述

本文档描述了基于 Tauri 2.0 框架的棋类游戏应用程序的技术设计。该应用程序采用前后端分离架构，前端使用现代 Web 技术（React）构建用户界面，后端使用 Rust 实现游戏逻辑和 AI 引擎。

### 技术栈

- **桌面框架**: Tauri 2.0
- **前端**: React 18 + TypeScript
- **后端**: Rust
- **状态管理**: Zustand
- **UI 组件**: 自定义 Canvas 渲染
- **构建工具**: Vite

### 设计原则

1. **关注点分离**: 游戏逻辑、AI 引擎和 UI 渲染相互独立
2. **可扩展性**: 易于添加新的棋类游戏
3. **性能优化**: AI 计算在后端 Rust 中执行，确保高性能
4. **跨平台一致性**: 使用 Tauri 确保在不同平台上的一致体验

## 架构

### 整体架构

应用程序采用三层架构：

```
┌─────────────────────────────────────────┐
│         前端层 (React + TypeScript)      │
│  ┌─────────────┐  ┌──────────────────┐  │
│  │ 游戏选择界面 │  │   游戏界面组件    │  │
│  └─────────────┘  └──────────────────┘  │
│  ┌─────────────────────────────────────┐│
│  │      棋盘渲染器 (Canvas)             ││
│  └─────────────────────────────────────┘│
└─────────────────────────────────────────┘
              ↕ Tauri IPC
┌─────────────────────────────────────────┐
│         后端层 (Rust)                    │
│  ┌─────────────────────────────────────┐│
│  │        游戏引擎核心                  ││
│  │  ┌──────────┐    ┌──────────────┐  ││
│  │  │象棋引擎   │    │  军棋引擎     │  ││
│  │  └──────────┘    └──────────────┘  ││
│  └─────────────────────────────────────┘│
│  ┌─────────────────────────────────────┐│
│  │           AI 引擎                    ││
│  │  ┌──────────┐    ┌──────────────┐  ││
│  │  │Minimax算法│    │  评估函数     │  ││
│  │  └──────────┘    └──────────────┘  ││
│  └─────────────────────────────────────┘│
└─────────────────────────────────────────┘
```

### 通信机制

前端和后端通过 Tauri 的 IPC（进程间通信）机制进行通信：

- **前端 → 后端**: 使用 `invoke` 调用 Rust 命令
- **后端 → 前端**: 使用事件系统推送状态更新

## 组件和接口

### 前端组件

#### 1. GameSelector 组件

游戏选择界面组件。

**职责**:
- 显示可用的游戏选项（象棋、军棋）
- 处理用户的游戏选择
- 导航到相应的游戏界面

**接口**:
```typescript
interface GameSelectorProps {
  onGameSelect: (gameType: 'xiangqi' | 'junqi') => void;
}
```

#### 2. GameBoard 组件

游戏棋盘主组件。

**职责**:
- 管理游戏状态
- 协调棋盘渲染和用户交互
- 与后端通信执行游戏逻辑

**接口**:
```typescript
interface GameBoardProps {
  gameType: 'xiangqi' | 'junqi';
  onBackToMenu: () => void;
}

interface GameBoardState {
  boardState: BoardState;
  selectedPiece: Position | null;
  legalMoves: Position[];
  gameStatus: GameStatus;
  isPlayerTurn: boolean;
}
```

#### 3. BoardRenderer 组件

棋盘渲染组件，使用 Canvas 绘制。

**职责**:
- 渲染棋盘网格
- 渲染棋子
- 显示选中状态和合法移动提示
- 播放移动动画

**接口**:
```typescript
interface BoardRendererProps {
  boardState: BoardState;
  selectedPiece: Position | null;
  legalMoves: Position[];
  onCellClick: (position: Position) => void;
  gameType: 'xiangqi' | 'junqi';
}
```

#### 4. GameControls 组件

游戏控制面板。

**职责**:
- 提供悔棋按钮
- 提供重新开始按钮
- 显示游戏状态信息

**接口**:
```typescript
interface GameControlsProps {
  onUndo: () => void;
  onRestart: () => void;
  gameStatus: GameStatus;
  canUndo: boolean;
}
```

### 后端组件（Rust）

#### 1. GameEngine trait

游戏引擎的通用接口。

**职责**:
- 定义所有游戏引擎必须实现的方法
- 提供游戏逻辑的统一接口

**接口**:
```rust
pub trait GameEngine {
    fn new_game() -> Self;
    fn get_board_state(&self) -> BoardState;
    fn get_legal_moves(&self, position: Position) -> Vec<Position>;
    fn make_move(&mut self, from: Position, to: Position) -> Result<MoveResult, GameError>;
    fn is_game_over(&self) -> bool;
    fn get_winner(&self) -> Option<Player>;
    fn undo_move(&mut self) -> Result<(), GameError>;
}
```

#### 2. XiangqiEngine 结构体

中国象棋游戏引擎实现。

**职责**:
- 实现象棋的所有规则
- 验证走法合法性
- 检测将军、将死、困毙状态
- 管理象棋游戏状态

**关键方法**:
```rust
impl XiangqiEngine {
    pub fn is_in_check(&self, player: Player) -> bool;
    pub fn is_checkmate(&self, player: Player) -> bool;
    pub fn is_stalemate(&self) -> bool;
    fn validate_piece_move(&self, piece: Piece, from: Position, to: Position) -> bool;
    fn can_generals_face(&self, from: Position, to: Position) -> bool;
}
```

#### 3. JunqiEngine 结构体

军棋游戏引擎实现。

**职责**:
- 实现军棋的所有规则
- 处理棋子战斗逻辑
- 管理特殊棋子（地雷、炸弹、军旗）
- 处理铁路线移动规则

**关键方法**:
```rust
impl JunqiEngine {
    pub fn resolve_battle(&self, attacker: Piece, defender: Piece) -> BattleResult;
    pub fn can_move_on_railway(&self, piece: Piece, from: Position, to: Position) -> bool;
    pub fn is_flag_captured(&self, player: Player) -> bool;
    fn get_piece_rank(&self, piece: Piece) -> u8;
}
```

#### 4. AIEngine 结构体

AI 对手引擎。

**职责**:
- 使用 Minimax 算法计算最优走法
- 实现 Alpha-Beta 剪枝优化
- 为不同游戏提供评估函数
- 支持可配置的搜索深度

**接口**:
```rust
pub struct AIEngine {
    search_depth: u8,
}

impl AIEngine {
    pub fn new(difficulty: Difficulty) -> Self;
    pub fn calculate_best_move<T: GameEngine>(&self, game: &T) -> Option<Move>;
    fn minimax<T: GameEngine>(&self, game: &T, depth: u8, alpha: i32, beta: i32, maximizing: bool) -> i32;
    fn evaluate_position<T: GameEngine>(&self, game: &T) -> i32;
}
```

#### 5. Tauri Commands

前端调用的 Rust 命令。

**接口**:
```rust
#[tauri::command]
fn start_new_game(game_type: String) -> Result<GameState, String>;

#[tauri::command]
fn get_legal_moves(game_id: String, position: Position) -> Result<Vec<Position>, String>;

#[tauri::command]
fn make_player_move(game_id: String, from: Position, to: Position) -> Result<MoveResult, String>;

#[tauri::command]
fn make_ai_move(game_id: String) -> Result<MoveResult, String>;

#[tauri::command]
fn undo_move(game_id: String) -> Result<GameState, String>;

#[tauri::command]
fn restart_game(game_id: String) -> Result<GameState, String>;
```

## 数据模型

### 通用数据结构

#### Position

表示棋盘上的位置。

```typescript
// TypeScript
interface Position {
  row: number;
  col: number;
}
```

```rust
// Rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    pub row: u8,
    pub col: u8,
}
```

#### Piece

表示棋子。

```typescript
// TypeScript
interface Piece {
  type: PieceType;
  player: Player;
}

type Player = 'red' | 'black';

// 象棋棋子类型
type XiangqiPieceType = 
  | 'general'    // 将/帅
  | 'advisor'    // 士
  | 'elephant'   // 象/相
  | 'horse'      // 马
  | 'chariot'    // 车
  | 'cannon'     // 炮
  | 'soldier';   // 兵/卒

// 军棋棋子类型
type JunqiPieceType =
  | 'flag'       // 军旗
  | 'landmine'   // 地雷
  | 'bomb'       // 炸弹
  | 'commander'  // 司令
  | 'general'    // 军长
  | 'major'      // 师长
  | 'colonel'    // 旅长
  | 'captain'    // 团长
  | 'battalion'  // 营长
  | 'company'    // 连长
  | 'platoon'    // 排长
  | 'engineer';  // 工兵
```

```rust
// Rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Piece {
    pub piece_type: PieceType,
    pub player: Player,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Player {
    Red,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PieceType {
    Xiangqi(XiangqiPiece),
    Junqi(JunqiPiece),
}
```

#### BoardState

表示棋盘状态。

```typescript
// TypeScript
interface BoardState {
  pieces: Map<string, Piece>; // key: "row,col"
  currentPlayer: Player;
  moveHistory: Move[];
}

interface Move {
  from: Position;
  to: Position;
  piece: Piece;
  capturedPiece?: Piece;
  timestamp: number;
}
```

```rust
// Rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardState {
    pub pieces: HashMap<Position, Piece>,
    pub current_player: Player,
    pub move_history: Vec<Move>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub piece: Piece,
    pub captured_piece: Option<Piece>,
    pub timestamp: u64,
}
```

#### GameStatus

表示游戏状态。

```typescript
// TypeScript
type GameStatus =
  | { type: 'ongoing' }
  | { type: 'check'; player: Player }
  | { type: 'checkmate'; winner: Player }
  | { type: 'stalemate' }
  | { type: 'victory'; winner: Player };
```

```rust
// Rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameStatus {
    Ongoing,
    Check { player: Player },
    Checkmate { winner: Player },
    Stalemate,
    Victory { winner: Player },
}
```

#### MoveResult

表示移动操作的结果。

```typescript
// TypeScript
interface MoveResult {
  success: boolean;
  newBoardState: BoardState;
  gameStatus: GameStatus;
  capturedPiece?: Piece;
}
```

```rust
// Rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveResult {
    pub success: bool,
    pub new_board_state: BoardState,
    pub game_status: GameStatus,
    pub captured_piece: Option<Piece>,
}
```

### 游戏特定数据

#### 象棋特定

```rust
// 象棋棋盘尺寸
pub const XIANGQI_ROWS: u8 = 10;
pub const XIANGQI_COLS: u8 = 9;

// 九宫格范围
pub const PALACE_ROWS: Range<u8> = 0..3; // 和 7..10
pub const PALACE_COLS: Range<u8> = 3..6;
```

#### 军棋特定

```rust
// 军棋棋盘尺寸
pub const JUNQI_ROWS: u8 = 12;
pub const JUNQI_COLS: u8 = 5;

// 特殊位置
pub const CAMP_POSITIONS: [(u8, u8); 10] = [
    // 营地位置坐标
];

pub const RAILWAY_POSITIONS: HashSet<(u8, u8)> = [
    // 铁路线位置坐标
];

// 棋子等级（用于战斗判定）
pub fn get_piece_rank(piece: JunqiPiece) -> u8 {
    match piece {
        JunqiPiece::Commander => 9,
        JunqiPiece::General => 8,
        JunqiPiece::Major => 7,
        JunqiPiece::Colonel => 6,
        JunqiPiece::Captain => 5,
        JunqiPiece::Battalion => 4,
        JunqiPiece::Company => 3,
        JunqiPiece::Platoon => 2,
        JunqiPiece::Engineer => 1,
        _ => 0,
    }
}
```

## 正确性属性

*属性是一个特征或行为，应该在系统的所有有效执行中保持为真——本质上是关于系统应该做什么的形式化陈述。属性作为人类可读规范和机器可验证正确性保证之间的桥梁。*


### 核心游戏逻辑属性

**属性 1：移动合法性验证**

*对于任何*游戏类型（象棋或军棋）和任何棋盘状态，当玩家尝试移动棋子时，系统应该只接受符合该游戏规则的合法移动，并拒绝所有非法移动。

**验证需求：2.2, 3.2**

**属性 2：游戏状态正确性**

*对于任何*合法移动，执行该移动后，游戏状态应该正确反映该移动的所有影响，包括棋盘布局、当前玩家、移动历史和游戏阶段。

**验证需求：5.2, 7.2**

**属性 3：象棋游戏状态检测**

*对于任何*象棋游戏状态，系统应该正确检测并报告游戏状态（进行中、将军、将死、困毙），包括识别哪个玩家处于将军状态。

**验证需求：2.3, 2.4, 2.5**

**属性 4：象棋特殊规则**

*对于任何*象棋移动，如果该移动会导致将帅照面（同一列且中间无棋子），系统应该拒绝该移动。

**验证需求：2.6**

**属性 5：军棋战斗逻辑**

*对于任何*两个军棋棋子的战斗，系统应该根据棋子类型和等级正确判定战斗结果，包括特殊棋子（地雷、炸弹、工兵）的特殊交互规则。

**验证需求：3.3, 3.5**

**属性 6：军棋工兵铁路移动**

*对于任何*军棋游戏状态，当工兵位于铁路线上时，系统应该允许工兵沿铁路线移动到任何可达的铁路位置（无棋子阻挡）。

**验证需求：3.4**

**属性 7：军棋游戏结束条件**

*对于任何*军棋游戏状态，当一方的军旗被对方棋子夺取时，系统应该判定游戏结束并宣布夺旗方获胜。

**验证需求：3.6**

### UI 和交互属性

**属性 8：棋盘渲染完整性**

*对于任何*游戏状态，棋盘渲染器应该渲染所有存在的棋子，且每个棋子的位置、类型和所属玩家应该与游戏状态一致。

**验证需求：4.1**

**属性 9：棋子选择和合法移动显示**

*对于任何*被选中的棋子，系统应该高亮显示该棋子，并显示该棋子的所有合法移动位置，且显示的位置应该与游戏引擎计算的合法移动完全一致。

**验证需求：4.3, 4.4, 5.1**

**属性 10：玩家颜色区分**

*对于任何*棋盘状态，渲染器应该使用不同的视觉属性（颜色或标记）来区分红方和黑方的棋子。

**验证需求：4.5**

**属性 11：非法移动拒绝**

*对于任何*非法移动尝试，系统应该拒绝该移动，保持游戏状态不变，且不改变当前玩家。

**验证需求：5.3**

**属性 12：棋子选择切换**

*对于任何*已选中的棋子，当玩家再次点击该棋子时，系统应该取消选择状态，清除高亮和合法移动显示。

**验证需求：5.4**

### AI 属性

**属性 13：AI 移动合法性**

*对于任何*游戏状态，当轮到 AI 行动时，AI 引擎返回的移动必须是符合游戏规则的合法移动。

**验证需求：6.3**

**属性 14：AI 难度级别**

*对于任何*难度设置，AI 引擎应该使用与该难度对应的搜索深度，且更高难度应该使用更大的搜索深度。

**验证需求：6.5**

### 状态管理属性

**属性 15：游戏状态完整性**

*对于任何*时刻，游戏状态应该包含所有必要的信息：完整的棋盘布局、当前轮到哪个玩家、完整的移动历史记录和当前游戏阶段。

**验证需求：7.1**

**属性 16：游戏结束记录**

*对于任何*导致游戏结束的状态，系统应该正确记录游戏结果（获胜方或平局），并将该信息包含在游戏状态中。

**验证需求：7.3**

**属性 17：移动历史记录**

*对于任何*执行的移动，该移动的完整信息（起始位置、目标位置、移动的棋子、被吃掉的棋子）应该被添加到移动历史记录中。

**验证需求：7.4**

**属性 18：悔棋往返一致性**

*对于任何*游戏状态，执行一个合法移动然后立即悔棋，应该恢复到原始游戏状态（棋盘布局、当前玩家、移动历史都相同）。

**验证需求：7.5**

### 错误处理属性

**属性 19：错误消息生成**

*对于任何*导致错误的操作（非法移动、无效输入等），系统应该生成包含错误描述的错误消息。

**验证需求：10.1**

**属性 20：错误日志记录**

*对于任何*发生的错误或异常，系统应该将错误信息（包括错误类型、时间戳和上下文）写入日志文件。

**验证需求：10.4**

## 错误处理

### 错误类型

应用程序应该处理以下类型的错误：

1. **非法移动错误**
   - 错误码：`ILLEGAL_MOVE`
   - 触发条件：玩家尝试执行不符合游戏规则的移动
   - 处理方式：拒绝移动，显示错误提示，保持游戏状态不变

2. **无效输入错误**
   - 错误码：`INVALID_INPUT`
   - 触发条件：用户输入无效的位置或命令
   - 处理方式：显示错误提示，请求重新输入

3. **游戏状态错误**
   - 错误码：`INVALID_STATE`
   - 触发条件：游戏状态不一致或损坏
   - 处理方式：记录错误日志，提供重新开始游戏的选项

4. **AI 计算错误**
   - 错误码：`AI_ERROR`
   - 触发条件：AI 引擎计算失败或超时
   - 处理方式：记录错误，执行随机合法移动或通知玩家

5. **IPC 通信错误**
   - 错误码：`IPC_ERROR`
   - 触发条件：前后端通信失败
   - 处理方式：重试通信，失败后显示错误消息

### 错误处理策略

```rust
// Rust 错误类型定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameError {
    IllegalMove { from: Position, to: Position, reason: String },
    InvalidInput { message: String },
    InvalidState { message: String },
    AIError { message: String },
    IPCError { message: String },
}

impl GameError {
    pub fn error_code(&self) -> &str {
        match self {
            GameError::IllegalMove { .. } => "ILLEGAL_MOVE",
            GameError::InvalidInput { .. } => "INVALID_INPUT",
            GameError::InvalidState { .. } => "INVALID_STATE",
            GameError::AIError { .. } => "AI_ERROR",
            GameError::IPCError { .. } => "IPC_ERROR",
        }
    }
    
    pub fn user_message(&self) -> String {
        match self {
            GameError::IllegalMove { reason, .. } => 
                format!("非法移动：{}", reason),
            GameError::InvalidInput { message } => 
                format!("无效输入：{}", message),
            GameError::InvalidState { message } => 
                format!("游戏状态错误：{}。请重新开始游戏。", message),
            GameError::AIError { message } => 
                format!("AI 计算错误：{}", message),
            GameError::IPCError { message } => 
                format!("通信错误：{}", message),
        }
    }
}
```

### 日志记录

所有错误应该被记录到日志文件中，包含以下信息：

- 时间戳
- 错误类型和错误码
- 错误消息
- 游戏状态快照（如果适用）
- 堆栈跟踪（对于严重错误）

```rust
pub fn log_error(error: &GameError, game_state: Option<&GameState>) {
    let timestamp = SystemTime::now();
    let log_entry = LogEntry {
        timestamp,
        error_code: error.error_code().to_string(),
        message: error.user_message(),
        game_state: game_state.cloned(),
    };
    
    // 写入日志文件
    write_to_log_file(&log_entry);
}
```

## 测试策略

### 双重测试方法

本项目采用单元测试和基于属性的测试相结合的方法，以确保全面的测试覆盖：

- **单元测试**：验证特定示例、边缘情况和错误条件
- **基于属性的测试**：验证跨所有输入的通用属性

两者是互补的，都是实现全面覆盖所必需的。单元测试捕获具体的错误，基于属性的测试验证一般正确性。

### 基于属性的测试配置

**测试库选择**：
- **Rust 后端**：使用 `proptest` 库进行基于属性的测试
- **TypeScript 前端**：使用 `fast-check` 库进行基于属性的测试

**测试配置**：
- 每个属性测试最少运行 **100 次迭代**（由于随机化）
- 每个属性测试必须引用其设计文档中的属性
- 标签格式：`// Feature: chess-game-app, Property {number}: {property_text}`

**属性测试实现要求**：
- 每个正确性属性必须由**单个**基于属性的测试实现
- 测试应该生成随机输入来验证属性
- 测试失败时应该提供清晰的反例

### 单元测试平衡

- 单元测试对特定示例和边缘情况很有帮助
- 避免编写过多的单元测试 - 基于属性的测试处理大量输入的覆盖
- 单元测试应该关注：
  - 演示正确行为的特定示例
  - 组件之间的集成点
  - 边缘情况和错误条件
- 基于属性的测试应该关注：
  - 对所有输入都成立的通用属性
  - 通过随机化实现全面的输入覆盖

### 测试组织

#### Rust 后端测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    
    // 单元测试示例
    #[test]
    fn test_xiangqi_initial_setup() {
        let game = XiangqiEngine::new_game();
        assert_eq!(game.get_board_state().pieces.len(), 32);
    }
    
    // 基于属性的测试示例
    // Feature: chess-game-app, Property 1: 移动合法性验证
    proptest! {
        #[test]
        fn prop_only_legal_moves_accepted(
            board_state in arbitrary_xiangqi_state(),
            move_attempt in arbitrary_move()
        ) {
            let mut game = XiangqiEngine::from_state(board_state);
            let result = game.make_move(move_attempt.from, move_attempt.to);
            
            if result.is_ok() {
                // 如果移动被接受，它必须是合法的
                let legal_moves = game.get_legal_moves(move_attempt.from);
                prop_assert!(legal_moves.contains(&move_attempt.to));
            }
        }
    }
}
```

#### TypeScript 前端测试

```typescript
import fc from 'fast-check';
import { describe, it, expect } from 'vitest';

describe('GameBoard Component', () => {
  // 单元测试示例
  it('should initialize with empty selection', () => {
    const board = new GameBoard('xiangqi');
    expect(board.selectedPiece).toBeNull();
  });
  
  // 基于属性的测试示例
  // Feature: chess-game-app, Property 9: 棋子选择和合法移动显示
  it('should show legal moves for selected piece', () => {
    fc.assert(
      fc.property(
        fc.record({
          boardState: arbitraryBoardState(),
          position: arbitraryPosition()
        }),
        ({ boardState, position }) => {
          const board = new GameBoard('xiangqi');
          board.setBoardState(boardState);
          
          if (board.hasPieceAt(position)) {
            board.selectPiece(position);
            const displayedMoves = board.getDisplayedLegalMoves();
            const engineMoves = board.engine.getLegalMoves(position);
            
            // 显示的移动应该与引擎计算的完全一致
            expect(displayedMoves).toEqual(engineMoves);
          }
        }
      ),
      { numRuns: 100 }
    );
  });
});
```

### 测试覆盖目标

- **核心游戏逻辑**：100% 的规则验证函数
- **AI 引擎**：所有评估和搜索函数
- **状态管理**：所有状态转换和历史记录功能
- **错误处理**：所有错误路径和恢复机制

### 集成测试

除了单元测试和基于属性的测试，还应该包括：

1. **前后端集成测试**：验证 Tauri IPC 通信
2. **完整游戏流程测试**：从游戏开始到结束的完整流程
3. **UI 交互测试**：使用 Playwright 或类似工具进行端到端测试

### 性能测试

- **AI 响应时间**：确保 AI 在 5 秒内返回移动
- **渲染性能**：确保动画流畅（60 FPS）
- **内存使用**：监控长时间游戏会话的内存使用

### 测试数据生成

为基于属性的测试创建生成器：

```rust
// Rust 生成器示例
fn arbitrary_xiangqi_state() -> impl Strategy<Value = BoardState> {
    // 生成有效的象棋棋盘状态
    // 确保将帅存在，棋子数量合理等
}

fn arbitrary_move() -> impl Strategy<Value = Move> {
    // 生成随机移动（可能合法也可能非法）
}
```

```typescript
// TypeScript 生成器示例
const arbitraryBoardState = (): fc.Arbitrary<BoardState> => {
  // 生成有效的棋盘状态
};

const arbitraryPosition = (): fc.Arbitrary<Position> => {
  return fc.record({
    row: fc.integer({ min: 0, max: 9 }),
    col: fc.integer({ min: 0, max: 8 })
  });
};
```
