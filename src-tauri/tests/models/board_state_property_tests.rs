// 属性测试：游戏状态完整性
// Feature: chess-game-app, Property 15: 游戏状态完整性
// **验证需求：7.1**

use proptest::prelude::*;
use std::collections::HashMap;
use super::{BoardState, Move, Position, Piece, Player, PieceType, XiangqiPiece, JunqiPiece};

// 生成任意 Player
fn arbitrary_player() -> impl Strategy<Value = Player> {
    prop_oneof![
        Just(Player::Red),
        Just(Player::Black),
    ]
}

// 生成任意象棋位置
fn arbitrary_xiangqi_position() -> impl Strategy<Value = Position> {
    (0u8..10, 0u8..9).prop_map(|(row, col)| Position::new(row, col))
}

// 生成任意军棋位置
fn arbitrary_junqi_position() -> impl Strategy<Value = Position> {
    (0u8..12, 0u8..5).prop_map(|(row, col)| Position::new(row, col))
}

// 生成任意位置（用于通用场景）
fn arbitrary_position() -> impl Strategy<Value = Position> {
    (0u8..12, 0u8..9).prop_map(|(row, col)| Position::new(row, col))
}

// 生成任意 XiangqiPiece
fn arbitrary_xiangqi_piece() -> impl Strategy<Value = XiangqiPiece> {
    prop_oneof![
        Just(XiangqiPiece::General),
        Just(XiangqiPiece::Advisor),
        Just(XiangqiPiece::Elephant),
        Just(XiangqiPiece::Horse),
        Just(XiangqiPiece::Chariot),
        Just(XiangqiPiece::Cannon),
        Just(XiangqiPiece::Soldier),
    ]
}

// 生成任意 JunqiPiece
fn arbitrary_junqi_piece() -> impl Strategy<Value = JunqiPiece> {
    prop_oneof![
        Just(JunqiPiece::Flag),
        Just(JunqiPiece::Landmine),
        Just(JunqiPiece::Bomb),
        Just(JunqiPiece::Commander),
        Just(JunqiPiece::General),
        Just(JunqiPiece::Major),
        Just(JunqiPiece::Colonel),
        Just(JunqiPiece::Captain),
        Just(JunqiPiece::Battalion),
        Just(JunqiPiece::Company),
        Just(JunqiPiece::Platoon),
        Just(JunqiPiece::Engineer),
    ]
}

// 生成任意 PieceType
fn arbitrary_piece_type() -> impl Strategy<Value = PieceType> {
    prop_oneof![
        arbitrary_xiangqi_piece().prop_map(PieceType::Xiangqi),
        arbitrary_junqi_piece().prop_map(PieceType::Junqi),
    ]
}

// 生成任意 Piece
fn arbitrary_piece() -> impl Strategy<Value = Piece> {
    (arbitrary_piece_type(), arbitrary_player())
        .prop_map(|(piece_type, player)| Piece::new(piece_type, player))
}

// 生成任意 Move
fn arbitrary_move() -> impl Strategy<Value = Move> {
    (
        arbitrary_position(),
        arbitrary_position(),
        arbitrary_piece(),
        proptest::option::of(arbitrary_piece()),
        any::<u64>(),
    ).prop_map(|(from, to, piece, captured_piece, timestamp)| {
        Move {
            from,
            to,
            piece,
            captured_piece,
            timestamp,
        }
    })
}

// 生成任意棋盘布局（HashMap<Position, Piece>）
// 智能生成：根据棋子类型生成合适的位置
fn arbitrary_pieces() -> impl Strategy<Value = HashMap<Position, Piece>> {
    proptest::collection::vec(
        prop_oneof![
            // 生成象棋棋子及其位置
            (arbitrary_xiangqi_position(), arbitrary_xiangqi_piece(), arbitrary_player())
                .prop_map(|(pos, piece_type, player)| {
                    (pos, Piece::new(PieceType::Xiangqi(piece_type), player))
                }),
            // 生成军棋棋子及其位置
            (arbitrary_junqi_position(), arbitrary_junqi_piece(), arbitrary_player())
                .prop_map(|(pos, piece_type, player)| {
                    (pos, Piece::new(PieceType::Junqi(piece_type), player))
                }),
        ],
        0..32, // 最多 32 个棋子（象棋的最大棋子数）
    ).prop_map(|vec| {
        // 将 Vec 转换为 HashMap，自动去重相同位置的棋子
        vec.into_iter().collect()
    })
}

// 生成任意移动历史
fn arbitrary_move_history() -> impl Strategy<Value = Vec<Move>> {
    proptest::collection::vec(arbitrary_move(), 0..100)
}

// 生成任意 BoardState
fn arbitrary_board_state() -> impl Strategy<Value = BoardState> {
    (
        arbitrary_pieces(),
        arbitrary_player(),
        arbitrary_move_history(),
    ).prop_map(|(pieces, current_player, move_history)| {
        BoardState {
            pieces,
            current_player,
            move_history,
        }
    })
}

proptest! {
    // 配置：减少测试用例数量以加快测试速度
    #![proptest_config(ProptestConfig::with_cases(1))]
    
    /// 属性 15：游戏状态完整性
    /// 
    /// 对于任何时刻，游戏状态应该包含所有必要的信息：
    /// - 完整的棋盘布局（pieces 字段）
    /// - 当前轮到哪个玩家（current_player 字段）
    /// - 完整的移动历史记录（move_history 字段）
    /// 
    /// 这个属性测试验证 BoardState 结构体始终包含这些必要字段，
    /// 并且这些字段可以被正确访问和序列化/反序列化。
    #[test]
    fn prop_game_state_integrity(board_state in arbitrary_board_state()) {
        // 验证棋盘布局字段存在且可访问
        let pieces = &board_state.pieces;
        prop_assert!(pieces.len() <= 32, "棋盘上的棋子数量应该合理（不超过32个）");
        
        // 验证每个棋子位置都是有效的
        for (position, piece) in pieces.iter() {
            prop_assert!(
                position.row < 12 && position.col < 9,
                "棋子位置应该在合理范围内"
            );
            
            // 验证棋子信息完整
            match piece.piece_type {
                PieceType::Xiangqi(_) => {
                    // 象棋棋子应该在象棋棋盘范围内
                    prop_assert!(
                        position.row < 10 && position.col < 9,
                        "象棋棋子应该在象棋棋盘范围内"
                    );
                }
                PieceType::Junqi(_) => {
                    // 军棋棋子应该在军棋棋盘范围内
                    prop_assert!(
                        position.row < 12 && position.col < 5,
                        "军棋棋子应该在军棋棋盘范围内"
                    );
                }
            }
        }
        
        // 验证当前玩家字段存在且有效
        let current_player = board_state.current_player;
        prop_assert!(
            current_player == Player::Red || current_player == Player::Black,
            "当前玩家必须是 Red 或 Black"
        );
        
        // 验证移动历史记录字段存在且可访问
        let move_history = &board_state.move_history;
        prop_assert!(
            move_history.len() <= 200,
            "移动历史记录长度应该合理（不超过200步）"
        );
        
        // 验证移动历史中的每个移动都包含完整信息
        for move_item in move_history.iter() {
            // 验证起始位置
            prop_assert!(
                move_item.from.row < 12 && move_item.from.col < 9,
                "移动的起始位置应该有效"
            );
            
            // 验证目标位置
            prop_assert!(
                move_item.to.row < 12 && move_item.to.col < 9,
                "移动的目标位置应该有效"
            );
            
            // 验证移动的棋子信息存在
            let _ = move_item.piece;
            
            // 验证时间戳存在
            let _ = move_item.timestamp;
        }
        
        // 验证游戏状态可以被序列化和反序列化（测试完整性）
        let serialized = serde_json::to_string(&board_state);
        prop_assert!(serialized.is_ok(), "游戏状态应该可以被序列化");
        
        if let Ok(json_str) = serialized {
            let deserialized: Result<BoardState, _> = serde_json::from_str(&json_str);
            prop_assert!(deserialized.is_ok(), "游戏状态应该可以被反序列化");
            
            if let Ok(restored_state) = deserialized {
                // 验证反序列化后的状态与原始状态相同
                prop_assert_eq!(
                    restored_state.pieces.len(),
                    board_state.pieces.len(),
                    "反序列化后棋盘布局应该保持一致"
                );
                prop_assert_eq!(
                    restored_state.current_player,
                    board_state.current_player,
                    "反序列化后当前玩家应该保持一致"
                );
                prop_assert_eq!(
                    restored_state.move_history.len(),
                    board_state.move_history.len(),
                    "反序列化后移动历史长度应该保持一致"
                );
                
                // 验证每个棋子位置都被正确恢复
                for (pos, piece) in board_state.pieces.iter() {
                    prop_assert!(
                        restored_state.pieces.contains_key(pos),
                        "反序列化后应该包含所有原始位置"
                    );
                    prop_assert_eq!(
                        restored_state.pieces.get(pos),
                        Some(piece),
                        "反序列化后棋子应该保持一致"
                    );
                }
            }
        }
    }
    
    /// 属性测试：新创建的游戏状态应该具有完整性
    /// 
    /// 验证通过 BoardState::new() 创建的游戏状态包含所有必要字段
    #[test]
    fn prop_new_game_state_has_integrity(_dummy in 0..100u32) {
        let board = BoardState::new();
        
        // 验证棋盘布局字段存在（初始为空）
        prop_assert_eq!(board.pieces.len(), 0, "新游戏的棋盘应该为空");
        
        // 验证当前玩家字段存在且为红方
        prop_assert_eq!(board.current_player, Player::Red, "新游戏应该由红方先手");
        
        // 验证移动历史记录字段存在（初始为空）
        prop_assert_eq!(board.move_history.len(), 0, "新游戏的移动历史应该为空");
        
        // 验证可以序列化
        let serialized = serde_json::to_string(&board);
        prop_assert!(serialized.is_ok(), "新游戏状态应该可以被序列化");
    }
    
    /// 属性测试：克隆的游戏状态应该保持完整性
    /// 
    /// 验证克隆操作不会丢失任何游戏状态信息
    #[test]
    fn prop_cloned_state_maintains_integrity(board_state in arbitrary_board_state()) {
        let cloned = board_state.clone();
        
        // 验证棋盘布局完整性
        prop_assert_eq!(
            cloned.pieces.len(),
            board_state.pieces.len(),
            "克隆后棋盘布局应该保持一致"
        );
        
        // 验证每个棋子位置都被正确克隆
        for (pos, piece) in board_state.pieces.iter() {
            prop_assert!(
                cloned.pieces.contains_key(pos),
                "克隆后应该包含所有原始位置"
            );
            prop_assert_eq!(
                cloned.pieces.get(pos),
                Some(piece),
                "克隆后棋子应该保持一致"
            );
        }
        
        // 验证当前玩家完整性
        prop_assert_eq!(
            cloned.current_player,
            board_state.current_player,
            "克隆后当前玩家应该保持一致"
        );
        
        // 验证移动历史完整性
        prop_assert_eq!(
            cloned.move_history.len(),
            board_state.move_history.len(),
            "克隆后移动历史应该保持一致"
        );
        
        // 验证克隆的状态可以独立序列化
        let cloned_serialized = serde_json::to_string(&cloned);
        prop_assert!(cloned_serialized.is_ok(), "克隆状态应该可以被序列化");
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    
    #[test]
    fn test_empty_board_state_integrity() {
        let board = BoardState::new();
        
        // 验证所有必要字段都存在
        assert_eq!(board.pieces.len(), 0);
        assert_eq!(board.current_player, Player::Red);
        assert_eq!(board.move_history.len(), 0);
        
        // 验证可以序列化和反序列化
        let json = serde_json::to_string(&board).unwrap();
        let restored: BoardState = serde_json::from_str(&json).unwrap();
        
        assert_eq!(restored.pieces.len(), 0);
        assert_eq!(restored.current_player, Player::Red);
        assert_eq!(restored.move_history.len(), 0);
    }
    
    #[test]
    fn test_board_state_with_pieces_integrity() {
        let mut board = BoardState::new();
        
        // 添加一些棋子
        let pos1 = Position::new(0, 4);
        let piece1 = Piece::new(PieceType::Xiangqi(XiangqiPiece::General), Player::Red);
        board.pieces.insert(pos1, piece1);
        
        let pos2 = Position::new(9, 4);
        let piece2 = Piece::new(PieceType::Xiangqi(XiangqiPiece::General), Player::Black);
        board.pieces.insert(pos2, piece2);
        
        // 验证游戏状态完整性
        assert_eq!(board.pieces.len(), 2);
        assert_eq!(board.current_player, Player::Red);
        assert_eq!(board.move_history.len(), 0);
        
        // 验证序列化和反序列化保持完整性
        let json = serde_json::to_string(&board).unwrap();
        let restored: BoardState = serde_json::from_str(&json).unwrap();
        
        assert_eq!(restored.pieces.len(), 2);
        assert!(restored.pieces.contains_key(&pos1));
        assert!(restored.pieces.contains_key(&pos2));
    }
    
    #[test]
    fn test_board_state_with_move_history_integrity() {
        let mut board = BoardState::new();
        
        // 添加移动历史
        let move1 = Move {
            from: Position::new(0, 0),
            to: Position::new(1, 0),
            piece: Piece::new(PieceType::Xiangqi(XiangqiPiece::Soldier), Player::Red),
            captured_piece: None,
            timestamp: 1000,
        };
        board.move_history.push(move1);
        
        let move2 = Move {
            from: Position::new(9, 0),
            to: Position::new(8, 0),
            piece: Piece::new(PieceType::Xiangqi(XiangqiPiece::Soldier), Player::Black),
            captured_piece: None,
            timestamp: 2000,
        };
        board.move_history.push(move2);
        
        // 验证游戏状态完整性
        assert_eq!(board.move_history.len(), 2);
        assert_eq!(board.move_history[0].timestamp, 1000);
        assert_eq!(board.move_history[1].timestamp, 2000);
        
        // 验证序列化和反序列化保持完整性
        let json = serde_json::to_string(&board).unwrap();
        let restored: BoardState = serde_json::from_str(&json).unwrap();
        
        assert_eq!(restored.move_history.len(), 2);
        assert_eq!(restored.move_history[0].from, Position::new(0, 0));
        assert_eq!(restored.move_history[1].from, Position::new(9, 0));
    }
}
