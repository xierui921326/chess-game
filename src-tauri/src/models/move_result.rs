// MoveResult 结构体 - 表示移动操作的结果
use serde::{Deserialize, Serialize};
use super::{BoardState, GameStatus, Piece};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveResult {
    pub success: bool,
    pub new_board_state: BoardState,
    pub game_status: GameStatus,
    pub captured_piece: Option<Piece>,
}

impl MoveResult {
    pub fn new(
        success: bool,
        new_board_state: BoardState,
        game_status: GameStatus,
        captured_piece: Option<Piece>,
    ) -> Self {
        Self {
            success,
            new_board_state,
            game_status,
            captured_piece,
        }
    }
}
