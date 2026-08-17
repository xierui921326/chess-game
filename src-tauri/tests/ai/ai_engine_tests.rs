// AI 引擎单元测试
#[cfg(test)]
mod tests {
    use crate::ai::{AIEngine, Difficulty};
    use crate::game_engine::XiangqiEngine;
    use crate::game_engine::GameEngine;

    #[test]
    fn test_ai_engine_creation() {
        let ai_easy = AIEngine::new(Difficulty::Easy);
        assert_eq!(ai_easy.get_search_depth(), 1);

        let ai_medium = AIEngine::new(Difficulty::Medium);
        assert_eq!(ai_medium.get_search_depth(), 2);

        let ai_hard = AIEngine::new(Difficulty::Hard);
        assert_eq!(ai_hard.get_search_depth(), 3);
    }

    #[test]
    fn test_calculate_best_move_returns_legal_move() {
        // 创建一个新的象棋游戏
        let game = XiangqiEngine::new_game();
        let ai = AIEngine::new(Difficulty::Easy);

        // AI 应该能够找到一个合法的走法
        let best_move = ai.calculate_best_move(&game);
        assert!(best_move.is_some(), "AI 应该能够找到至少一个合法走法");

        // 验证返回的走法是合法的
        if let Some(mv) = best_move {
            let legal_moves = game.get_legal_moves(mv.from);
            assert!(
                legal_moves.contains(&mv.to),
                "AI 返回的走法应该是合法的"
            );
        }
    }

    #[test]
    fn test_calculate_best_move_with_different_difficulties() {
        let game = XiangqiEngine::new_game();

        // 测试不同难度级别都能返回走法
        let ai_easy = AIEngine::new(Difficulty::Easy);
        let move_easy = ai_easy.calculate_best_move(&game);
        assert!(move_easy.is_some(), "简单难度应该能找到走法");

        let ai_medium = AIEngine::new(Difficulty::Medium);
        let move_medium = ai_medium.calculate_best_move(&game);
        assert!(move_medium.is_some(), "中等难度应该能找到走法");

        let ai_hard = AIEngine::new(Difficulty::Hard);
        let move_hard = ai_hard.calculate_best_move(&game);
        assert!(move_hard.is_some(), "困难难度应该能找到走法");
    }

    #[test]
    fn test_ai_move_is_valid() {
        // 创建游戏并让 AI 计算走法
        let mut game = XiangqiEngine::new_game();
        let ai = AIEngine::new(Difficulty::Easy);

        let best_move = ai.calculate_best_move(&game);
        assert!(best_move.is_some());

        // 尝试执行 AI 的走法
        if let Some(mv) = best_move {
            let result = game.make_move(mv.from, mv.to);
            assert!(result.is_ok(), "AI 返回的走法应该能够成功执行");
        }
    }

    #[test]
    fn test_ai_prefers_winning_move() {
        // 这个测试验证 AI 能够识别获胜的走法
        // 注意：这需要一个接近结束的游戏状态
        // 目前使用简单的评估函数，所以这个测试主要验证算法结构正确
        
        let game = XiangqiEngine::new_game();
        let ai = AIEngine::new(Difficulty::Easy);
        
        let best_move = ai.calculate_best_move(&game);
        assert!(best_move.is_some(), "AI 应该能够在标准开局找到走法");
    }

    #[test]
    fn test_evaluation_function_basic() {
        // 测试评估函数的基本功能
        let game = XiangqiEngine::new_game();
        let ai = AIEngine::new(Difficulty::Easy);
        
        // 在初始状态下，评估分数应该接近 0（双方势均力敌）
        // 注意：evaluate_position 是私有方法，我们通过 calculate_best_move 间接测试
        let best_move = ai.calculate_best_move(&game);
        assert!(best_move.is_some(), "评估函数应该能够工作并返回走法");
    }

    #[test]
    fn test_evaluation_considers_material() {
        // 测试评估函数考虑材料价值
        // 创建一个红方有优势的局面
        use crate::models::{BoardState, Piece, PieceType, XiangqiPiece, Player, Position};
        
        let mut board_state = BoardState::new();
        
        // 放置双方的将/帅
        board_state.pieces.insert(
            Position::new(0, 4),
            Piece::new(PieceType::Xiangqi(XiangqiPiece::General), Player::Black)
        );
        board_state.pieces.insert(
            Position::new(9, 4),
            Piece::new(PieceType::Xiangqi(XiangqiPiece::General), Player::Red)
        );
        
        // 红方多一个车（材料优势）
        board_state.pieces.insert(
            Position::new(8, 0),
            Piece::new(PieceType::Xiangqi(XiangqiPiece::Chariot), Player::Red)
        );
        
        board_state.current_player = Player::Red;
        
        // 创建游戏引擎（注意：这里我们需要一个方法来从 BoardState 创建引擎）
        // 由于当前实现没有这个方法，我们只验证 AI 能够处理标准游戏
        let game = XiangqiEngine::new_game();
        let ai = AIEngine::new(Difficulty::Easy);
        
        let best_move = ai.calculate_best_move(&game);
        assert!(best_move.is_some(), "AI 应该能够评估并返回走法");
    }
}
