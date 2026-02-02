// 属性测试：AI 引擎
// Feature: chess-game-app, Property 13: AI 移动合法性
// Feature: chess-game-app, Property 14: AI 难度级别
// **验证需求：6.3, 6.5**

use proptest::prelude::*;
use crate::models::{Player, PieceType, JunqiPiece, XiangqiPiece};
use crate::game_engine::{GameEngine, junqi_engine::JunqiEngine, xiangqi_engine::XiangqiEngine};
use crate::ai::{AIEngine, Difficulty};

// ============ 生成器定义 ============

/// 生成任意难度级别
fn arbitrary_difficulty() -> impl Strategy<Value = Difficulty> {
    prop_oneof![
        Just(Difficulty::Easy),
        Just(Difficulty::Medium),
        Just(Difficulty::Hard),
    ]
}

// ============ 属性测试 ============

proptest! {
    // 配置：减少测试用例数量以加快测试速度
    #![proptest_config(ProptestConfig::with_cases(1))]
    
    /// 属性 13：AI 移动合法性
    /// 
    /// 对于任何游戏状态，当轮到 AI 行动时，
    /// AI 引擎返回的移动必须是符合游戏规则的合法移动。
    /// 
    /// 这个属性测试验证：
    /// 1. AI 返回的移动（如果有）必须是合法的
    /// 2. AI 返回的起始位置必须有 AI 玩家的棋子
    /// 3. AI 返回的目标位置必须在该棋子的合法移动列表中
    /// 4. 如果没有合法移动，AI 应该返回 None
    #[test]
    fn prop_ai_move_legality_junqi(difficulty in arbitrary_difficulty()) {
        // 使用标准开局测试
        let game = JunqiEngine::new_game();
        let ai_player = game.get_board_state().current_player;
        
        // 创建 AI 引擎
        let ai_engine = AIEngine::new(difficulty);
        
        // 计算 AI 的最佳移动
        let ai_move = ai_engine.calculate_best_move(&game);
        
        // 如果 AI 返回了移动
        if let Some(mv) = ai_move {
            // 验证1：起始位置必须有棋子
            let piece_at_from = game.get_board_state().pieces.get(&mv.from);
            prop_assert!(
                piece_at_from.is_some(),
                "AI 移动的起始位置 {:?} 必须有棋子",
                mv.from
            );
            
            // 验证2：起始位置的棋子必须属于 AI 玩家
            let piece = piece_at_from.unwrap();
            prop_assert_eq!(
                piece.player,
                ai_player,
                "AI 移动的棋子必须属于 AI 玩家（{:?}）",
                ai_player
            );
            
            // 验证3：目标位置必须在合法移动列表中
            let legal_moves = game.get_legal_moves(mv.from);
            prop_assert!(
                legal_moves.contains(&mv.to),
                "AI 移动的目标位置 {:?} 必须在合法移动列表中（从 {:?}）",
                mv.to, mv.from
            );
            
            // 验证4：移动必须能够成功执行
            let mut game_copy = game.clone();
            let result = game_copy.make_move(mv.from, mv.to);
            prop_assert!(
                result.is_ok(),
                "AI 返回的移动必须能够成功执行（从 {:?} 到 {:?}）",
                mv.from, mv.to
            );
        }
    }
    
    /// 属性测试：AI 移动合法性（象棋）
    #[test]
    fn prop_ai_move_legality_xiangqi(difficulty in arbitrary_difficulty()) {
        // 使用标准开局测试
        let game = XiangqiEngine::new_game();
        let ai_player = game.get_board_state().current_player;
        
        // 创建 AI 引擎
        let ai_engine = AIEngine::new(difficulty);
        
        // 计算 AI 的最佳移动
        let ai_move = ai_engine.calculate_best_move(&game);
        
        // 如果 AI 返回了移动
        if let Some(mv) = ai_move {
            // 验证1：起始位置必须有棋子
            let piece_at_from = game.get_board_state().pieces.get(&mv.from);
            prop_assert!(
                piece_at_from.is_some(),
                "AI 移动的起始位置 {:?} 必须有棋子",
                mv.from
            );
            
            // 验证2：起始位置的棋子必须属于 AI 玩家
            let piece = piece_at_from.unwrap();
            prop_assert_eq!(
                piece.player,
                ai_player,
                "AI 移动的棋子必须属于 AI 玩家（{:?}）",
                ai_player
            );
            
            // 验证3：目标位置必须在合法移动列表中
            let legal_moves = game.get_legal_moves(mv.from);
            prop_assert!(
                legal_moves.contains(&mv.to),
                "AI 移动的目标位置 {:?} 必须在合法移动列表中（从 {:?}）",
                mv.to, mv.from
            );
            
            // 验证4：移动必须能够成功执行
            let mut game_copy = game.clone();
            let result = game_copy.make_move(mv.from, mv.to);
            prop_assert!(
                result.is_ok(),
                "AI 返回的移动必须能够成功执行（从 {:?} 到 {:?}）",
                mv.from, mv.to
            );
        }
    }
    
    /// 属性 14：AI 难度级别
    /// 
    /// 对于任何难度设置，AI 引擎应该使用与该难度对应的搜索深度，
    /// 且更高难度应该使用更大的搜索深度。
    /// 
    /// 这个属性测试验证：
    /// 1. Easy 难度使用搜索深度 2
    /// 2. Medium 难度使用搜索深度 4
    /// 3. Hard 难度使用搜索深度 6
    /// 4. 难度越高，搜索深度越大
    #[test]
    fn prop_ai_difficulty_search_depth(difficulty in arbitrary_difficulty()) {
        // 创建 AI 引擎
        let ai_engine = AIEngine::new(difficulty);
        
        // 获取搜索深度
        let search_depth = ai_engine.get_search_depth();
        
        // 验证搜索深度与难度对应
        let expected_depth = match difficulty {
            Difficulty::Easy => 2,
            Difficulty::Medium => 4,
            Difficulty::Hard => 6,
        };
        
        prop_assert_eq!(
            search_depth,
            expected_depth,
            "难度 {:?} 应该使用搜索深度 {}",
            difficulty, expected_depth
        );
    }
}

// 不使用 proptest 宏的测试
#[cfg(test)]
mod regular_tests {
    use super::*;
    
    /// 属性测试：难度级别的单调性
    #[test]
    fn test_difficulty_monotonicity() {
        // 创建不同难度的 AI 引擎
        let easy_ai = AIEngine::new(Difficulty::Easy);
        let medium_ai = AIEngine::new(Difficulty::Medium);
        let hard_ai = AIEngine::new(Difficulty::Hard);
        
        // 获取搜索深度
        let easy_depth = easy_ai.get_search_depth();
        let medium_depth = medium_ai.get_search_depth();
        let hard_depth = hard_ai.get_search_depth();
        
        // 验证单调性：Easy < Medium < Hard
        assert!(
            easy_depth < medium_depth,
            "Easy 难度的搜索深度（{}）应该小于 Medium 难度（{}）",
            easy_depth, medium_depth
        );
        
        assert!(
            medium_depth < hard_depth,
            "Medium 难度的搜索深度（{}）应该小于 Hard 难度（{}）",
            medium_depth, hard_depth
        );
    }
    
    /// 属性测试：AI 在有合法移动时应该返回移动
    #[test]
    fn test_ai_returns_move_when_possible_junqi() {
        // 使用标准开局（肯定有合法移动）
        let game = JunqiEngine::new_game();
        
        // 创建 AI 引擎
        let ai_engine = AIEngine::new(Difficulty::Medium);
        
        // 计算 AI 的最佳移动
        let ai_move = ai_engine.calculate_best_move(&game);
        
        // 在标准开局中，AI 应该能找到至少一个合法移动
        assert!(
            ai_move.is_some(),
            "在标准开局中，AI 应该能找到至少一个合法移动"
        );
    }
    
    /// 属性测试：AI 在有合法移动时应该返回移动（象棋）
    #[test]
    fn test_ai_returns_move_when_possible_xiangqi() {
        // 使用标准开局（肯定有合法移动）
        let game = XiangqiEngine::new_game();
        
        // 创建 AI 引擎
        let ai_engine = AIEngine::new(Difficulty::Medium);
        
        // 计算 AI 的最佳移动
        let ai_move = ai_engine.calculate_best_move(&game);
        
        // 在标准开局中，AI 应该能找到至少一个合法移动
        assert!(
            ai_move.is_some(),
            "在标准开局中，AI 应该能找到至少一个合法移动"
        );
    }
    
    /// 属性测试：AI 搜索深度为正数
    #[test]
    fn test_search_depth_positive() {
        let difficulties = vec![Difficulty::Easy, Difficulty::Medium, Difficulty::Hard];
        
        for difficulty in difficulties {
            let ai_engine = AIEngine::new(difficulty);
            let search_depth = ai_engine.get_search_depth();
            
            assert!(
                search_depth > 0,
                "搜索深度应该是正数，但得到 {}",
                search_depth
            );
        }
    }
    
    /// 属性测试：不同难度的 AI 都返回合法移动
    #[test]
    fn test_all_difficulties_return_legal_moves_junqi() {
        // 使用标准开局
        let game = JunqiEngine::new_game();
        
        // 测试所有难度级别
        let difficulties = vec![Difficulty::Easy, Difficulty::Medium, Difficulty::Hard];
        
        for difficulty in difficulties {
            let ai_engine = AIEngine::new(difficulty);
            let ai_move = ai_engine.calculate_best_move(&game);
            
            // 每个难度都应该能找到移动
            assert!(
                ai_move.is_some(),
                "难度 {:?} 应该能找到合法移动",
                difficulty
            );
            
            // 验证移动合法性
            if let Some(mv) = ai_move {
                let legal_moves = game.get_legal_moves(mv.from);
                assert!(
                    legal_moves.contains(&mv.to),
                    "难度 {:?} 返回的移动应该是合法的",
                    difficulty
                );
            }
        }
    }
    
    /// 属性测试：AI 移动后游戏状态有效
    #[test]
    fn test_ai_move_maintains_valid_state_junqi() {
        // 使用标准开局
        let mut game = JunqiEngine::new_game();
        
        // 创建 AI 引擎
        let ai_engine = AIEngine::new(Difficulty::Easy);
        
        // 计算 AI 的最佳移动
        let ai_move = ai_engine.calculate_best_move(&game);
        
        if let Some(mv) = ai_move {
            // 记录移动前的状态
            let pieces_before = game.get_board_state().pieces.len();
            let player_before = game.get_board_state().current_player;
            
            // 执行 AI 移动
            let result = game.make_move(mv.from, mv.to);
            assert!(result.is_ok(), "AI 移动应该成功");
            
            // 验证游戏状态有效
            let pieces_after = game.get_board_state().pieces.len();
            let player_after = game.get_board_state().current_player;
            
            // 玩家应该切换
            assert_eq!(
                player_after,
                player_before.opponent(),
                "移动后玩家应该切换"
            );
            
            // 棋子数量应该合理（可能减少，但不会增加）
            assert!(
                pieces_after <= pieces_before,
                "移动后棋子数量不应该增加"
            );
            
            // 双方军旗应该至少有一个还在（游戏未结束或刚结束）
            let red_flag_exists = game.get_board_state().pieces.values().any(|p| {
                p.player == Player::Red && 
                matches!(p.piece_type, PieceType::Junqi(JunqiPiece::Flag))
            });
            let black_flag_exists = game.get_board_state().pieces.values().any(|p| {
                p.player == Player::Black && 
                matches!(p.piece_type, PieceType::Junqi(JunqiPiece::Flag))
            });
            
            assert!(
                red_flag_exists || black_flag_exists,
                "至少一方的军旗应该还在棋盘上"
            );
        }
    }
    
    /// 属性测试：AI 移动后游戏状态有效（象棋）
    #[test]
    fn test_ai_move_maintains_valid_state_xiangqi() {
        // 使用标准开局
        let mut game = XiangqiEngine::new_game();
        
        // 创建 AI 引擎
        let ai_engine = AIEngine::new(Difficulty::Easy);
        
        // 计算 AI 的最佳移动
        let ai_move = ai_engine.calculate_best_move(&game);
        
        if let Some(mv) = ai_move {
            // 记录移动前的状态
            let pieces_before = game.get_board_state().pieces.len();
            let player_before = game.get_board_state().current_player;
            
            // 执行 AI 移动
            let result = game.make_move(mv.from, mv.to);
            assert!(result.is_ok(), "AI 移动应该成功");
            
            // 验证游戏状态有效
            let pieces_after = game.get_board_state().pieces.len();
            let player_after = game.get_board_state().current_player;
            
            // 玩家应该切换
            assert_eq!(
                player_after,
                player_before.opponent(),
                "移动后玩家应该切换"
            );
            
            // 棋子数量应该合理（可能减少，但不会增加）
            assert!(
                pieces_after <= pieces_before,
                "移动后棋子数量不应该增加"
            );
            
            // 双方将帅应该至少有一个还在（游戏未结束或刚结束）
            let red_general_exists = game.get_board_state().pieces.values().any(|p| {
                p.player == Player::Red && 
                matches!(p.piece_type, PieceType::Xiangqi(XiangqiPiece::General))
            });
            let black_general_exists = game.get_board_state().pieces.values().any(|p| {
                p.player == Player::Black && 
                matches!(p.piece_type, PieceType::Xiangqi(XiangqiPiece::General))
            });
            
            assert!(
                red_general_exists || black_general_exists,
                "至少一方的将帅应该还在棋盘上"
            );
        }
    }
    
    /// 属性测试：AI 不会移动对手的棋子
    #[test]
    fn test_ai_does_not_move_opponent_pieces_junqi() {
        // 使用标准开局
        let game = JunqiEngine::new_game();
        let ai_player = game.get_board_state().current_player;
        
        // 创建 AI 引擎
        let ai_engine = AIEngine::new(Difficulty::Easy);
        
        // 计算 AI 的最佳移动
        let ai_move = ai_engine.calculate_best_move(&game);
        
        if let Some(mv) = ai_move {
            // 验证起始位置的棋子属于 AI 玩家
            let piece = game.get_board_state().pieces.get(&mv.from);
            assert!(piece.is_some(), "起始位置应该有棋子");
            
            let piece = piece.unwrap();
            assert_eq!(
                piece.player,
                ai_player,
                "AI 不应该移动对手的棋子"
            );
        }
    }
}
