// 数据模型模块
// 包含 Position, Piece, Player, BoardState, Move 等核心数据结构

pub mod position;
pub mod piece;
pub mod board_state;
pub mod game_status;
pub mod move_result;

// 属性测试已移动到 tests/ 目录
// #[cfg(test)]
// mod board_state_property_tests;

// 重新导出常用类型
pub use position::Position;
pub use piece::{Piece, Player, PieceType, XiangqiPiece, JunqiPiece};
pub use board_state::{BoardState, Move};
pub use game_status::GameStatus;
// pub use move_result::MoveResult; // 未使用，已注释

// 注意：这些类型目前显示为未使用，但它们将在后续任务中被游戏引擎和命令模块使用
