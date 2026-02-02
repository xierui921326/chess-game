// GameEngine trait - 游戏引擎的通用接口
use crate::models::*;
use serde::{Serialize, Deserialize};

/// 游戏错误类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GameError {
    /// 非法移动错误
    IllegalMove { 
        from: Position, 
        to: Position, 
        reason: String 
    },
    /// 无效输入错误
    InvalidInput { 
        message: String 
    },
    /// 游戏状态错误
    InvalidState { 
        message: String 
    },
    /// AI 计算错误
    AIError { 
        message: String 
    },
    /// IPC 通信错误
    IPCError { 
        message: String 
    },
}

impl GameError {
    /// 获取错误码
    pub fn error_code(&self) -> &str {
        match self {
            GameError::IllegalMove { .. } => "ILLEGAL_MOVE",
            GameError::InvalidInput { .. } => "INVALID_INPUT",
            GameError::InvalidState { .. } => "INVALID_STATE",
            GameError::AIError { .. } => "AI_ERROR",
            GameError::IPCError { .. } => "IPC_ERROR",
        }
    }
    
    /// 获取用户友好的错误消息
    pub fn user_message(&self) -> String {
        match self {
            GameError::IllegalMove { from, to, reason } => 
                format!("非法移动：从 ({}, {}) 到 ({}, {}) - {}", 
                    from.row, from.col, to.row, to.col, reason),
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

pub type GameResult<T> = Result<T, GameError>;

pub trait GameEngine {
    fn new_game() -> Self;
    fn get_board_state(&self) -> &BoardState;
    fn get_legal_moves(&self, position: Position) -> Vec<Position>;
    fn make_move(&mut self, from: Position, to: Position) -> GameResult<()>;
    fn is_game_over(&self) -> bool;
    fn get_winner(&self) -> Option<Player>;
    fn undo_move(&mut self) -> GameResult<()>;
    fn get_game_status(&self) -> GameStatus;
}
