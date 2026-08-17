// Position 结构体 - 表示棋盘上的位置
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Position {
    pub row: u8,
    pub col: u8,
}

impl Position {
    pub fn new(row: u8, col: u8) -> Self {
        Self { row, col }
    }

    /// 检查位置是否在棋盘范围内
    pub fn is_valid_xiangqi(&self) -> bool {
        self.row < 10 && self.col < 9
    }

    /// 检查位置是否在军棋棋盘范围内
    #[allow(dead_code)]
    pub fn is_valid_junqi(&self) -> bool {
        self.row < 12 && self.col < 5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_creation() {
        let pos = Position::new(0, 0);
        assert_eq!(pos.row, 0);
        assert_eq!(pos.col, 0);
    }

    #[test]
    fn test_position_equality() {
        let pos1 = Position::new(3, 4);
        let pos2 = Position::new(3, 4);
        let pos3 = Position::new(3, 5);
        
        assert_eq!(pos1, pos2);
        assert_ne!(pos1, pos3);
    }

    #[test]
    fn test_xiangqi_position_validation() {
        assert!(Position::new(0, 0).is_valid_xiangqi());
        assert!(Position::new(9, 8).is_valid_xiangqi());
        assert!(!Position::new(10, 0).is_valid_xiangqi());
        assert!(!Position::new(0, 9).is_valid_xiangqi());
    }

    #[test]
    fn test_junqi_position_validation() {
        assert!(Position::new(0, 0).is_valid_junqi());
        assert!(Position::new(11, 4).is_valid_junqi());
        assert!(!Position::new(12, 0).is_valid_junqi());
        assert!(!Position::new(0, 5).is_valid_junqi());
    }
}
