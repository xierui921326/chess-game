// GameStatus 枚举 - 表示游戏状态
use serde::{Deserialize, Serialize};
use super::Player;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum GameStatus {
    Ongoing,
    Check { player: Player },
    Checkmate { winner: Player },
    Stalemate,
    Victory { winner: Player },
}
