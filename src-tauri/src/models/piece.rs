// Piece 相关结构体 - 表示棋子
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Player {
    Red,
    Black,
}

impl Player {
    /// 获取对手玩家
    pub fn opponent(&self) -> Player {
        match self {
            Player::Red => Player::Black,
            Player::Black => Player::Red,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum XiangqiPiece {
    General,   // 将/帅
    Advisor,   // 士
    Elephant,  // 象/相
    Horse,     // 马
    Chariot,   // 车
    Cannon,    // 炮
    Soldier,   // 兵/卒
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JunqiPiece {
    Flag,       // 军旗
    Landmine,   // 地雷
    Bomb,       // 炸弹
    Commander,  // 司令
    General,    // 军长
    Major,      // 师长
    Colonel,    // 旅长
    Captain,    // 团长
    Battalion,  // 营长
    Company,    // 连长
    Platoon,    // 排长
    Engineer,   // 工兵
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PieceType {
    Xiangqi(XiangqiPiece),
    Junqi(JunqiPiece),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Piece {
    pub piece_type: PieceType,
    pub player: Player,
}

impl Piece {
    pub fn new(piece_type: PieceType, player: Player) -> Self {
        Self { piece_type, player }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_creation() {
        let piece = Piece::new(
            PieceType::Xiangqi(XiangqiPiece::General),
            Player::Red,
        );
        assert_eq!(piece.player, Player::Red);
        assert_eq!(piece.piece_type, PieceType::Xiangqi(XiangqiPiece::General));
    }

    #[test]
    fn test_piece_equality() {
        let piece1 = Piece::new(
            PieceType::Xiangqi(XiangqiPiece::Soldier),
            Player::Black,
        );
        let piece2 = Piece::new(
            PieceType::Xiangqi(XiangqiPiece::Soldier),
            Player::Black,
        );
        let piece3 = Piece::new(
            PieceType::Xiangqi(XiangqiPiece::Soldier),
            Player::Red,
        );
        
        assert_eq!(piece1, piece2);
        assert_ne!(piece1, piece3);
    }

    #[test]
    fn test_all_xiangqi_pieces() {
        let pieces = vec![
            XiangqiPiece::General,
            XiangqiPiece::Advisor,
            XiangqiPiece::Elephant,
            XiangqiPiece::Horse,
            XiangqiPiece::Chariot,
            XiangqiPiece::Cannon,
            XiangqiPiece::Soldier,
        ];
        
        for piece_type in pieces {
            let piece = Piece::new(PieceType::Xiangqi(piece_type), Player::Red);
            assert_eq!(piece.player, Player::Red);
        }
    }

    #[test]
    fn test_all_junqi_pieces() {
        let pieces = vec![
            JunqiPiece::Flag,
            JunqiPiece::Landmine,
            JunqiPiece::Bomb,
            JunqiPiece::Commander,
            JunqiPiece::General,
            JunqiPiece::Major,
            JunqiPiece::Colonel,
            JunqiPiece::Captain,
            JunqiPiece::Battalion,
            JunqiPiece::Company,
            JunqiPiece::Platoon,
            JunqiPiece::Engineer,
        ];
        
        for piece_type in pieces {
            let piece = Piece::new(PieceType::Junqi(piece_type), Player::Black);
            assert_eq!(piece.player, Player::Black);
        }
    }
}
