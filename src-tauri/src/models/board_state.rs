// BoardState 和 Move 结构体
use serde::{Deserialize, Serialize, Serializer, Deserializer};
use std::collections::HashMap;
use super::{Position, Piece, Player};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub piece: Piece,
    pub captured_piece: Option<Piece>,
    pub timestamp: u64,
}

// 自定义序列化函数，将 HashMap<Position, Piece> 转换为 HashMap<String, Piece>
fn serialize_pieces<S>(pieces: &HashMap<Position, Piece>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let string_map: HashMap<String, Piece> = pieces
        .iter()
        .map(|(pos, piece)| (format!("{},{}", pos.row, pos.col), *piece))
        .collect();
    string_map.serialize(serializer)
}

// 自定义反序列化函数，将 HashMap<String, Piece> 转换为 HashMap<Position, Piece>
fn deserialize_pieces<'de, D>(deserializer: D) -> Result<HashMap<Position, Piece>, D::Error>
where
    D: Deserializer<'de>,
{
    let string_map: HashMap<String, Piece> = HashMap::deserialize(deserializer)?;
    let mut pieces = HashMap::new();
    
    for (key, piece) in string_map {
        let parts: Vec<&str> = key.split(',').collect();
        if parts.len() == 2 {
            if let (Ok(row), Ok(col)) = (parts[0].parse::<u8>(), parts[1].parse::<u8>()) {
                pieces.insert(Position::new(row, col), piece);
            }
        }
    }
    
    Ok(pieces)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardState {
    #[serde(serialize_with = "serialize_pieces", deserialize_with = "deserialize_pieces")]
    pub pieces: HashMap<Position, Piece>,
    pub current_player: Player,
    pub move_history: Vec<Move>,
}

impl BoardState {
    pub fn new() -> Self {
        Self {
            pieces: HashMap::new(),
            current_player: Player::Red,
            move_history: Vec::new(),
        }
    }
}

impl Default for BoardState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{PieceType, XiangqiPiece};

    #[test]
    fn test_board_state_creation() {
        let board = BoardState::new();
        assert_eq!(board.pieces.len(), 0);
        assert_eq!(board.current_player, Player::Red);
        assert_eq!(board.move_history.len(), 0);
    }

    #[test]
    fn test_board_state_with_pieces() {
        let mut board = BoardState::new();
        let pos = Position::new(0, 0);
        let piece = Piece::new(
            PieceType::Xiangqi(XiangqiPiece::General),
            Player::Red,
        );
        board.pieces.insert(pos, piece);
        
        assert_eq!(board.pieces.len(), 1);
        assert_eq!(board.pieces.get(&pos), Some(&piece));
    }

    #[test]
    fn test_move_creation() {
        let from = Position::new(0, 0);
        let to = Position::new(1, 0);
        let piece = Piece::new(
            PieceType::Xiangqi(XiangqiPiece::Soldier),
            Player::Red,
        );
        
        let move_obj = Move {
            from,
            to,
            piece,
            captured_piece: None,
            timestamp: 1234567890,
        };
        
        assert_eq!(move_obj.from, from);
        assert_eq!(move_obj.to, to);
        assert_eq!(move_obj.piece, piece);
        assert_eq!(move_obj.captured_piece, None);
    }
}
