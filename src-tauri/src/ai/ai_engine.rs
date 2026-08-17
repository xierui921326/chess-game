// AIEngine - AI 对手引擎
use super::difficulty::Difficulty;
use crate::game_engine::GameEngine;
use crate::models::{Position, Player, PieceType, XiangqiPiece, JunqiPiece};

/// AI 引擎，使用 Minimax 算法和 Alpha-Beta 剪枝计算最优走法
pub struct AIEngine {
    search_depth: u8,
}

/// 表示一个走法（从某个位置移动到另一个位置）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub from: Position,
    pub to: Position,
}

impl AIEngine {
    /// 创建新的 AI 引擎，根据难度设置搜索深度
    pub fn new(difficulty: Difficulty) -> Self {
        Self {
            search_depth: difficulty.search_depth(),
        }
    }

    /// 获取搜索深度
    #[allow(dead_code)]
    pub fn get_search_depth(&self) -> u8 {
        self.search_depth
    }

    /// 计算最优走法
    /// 
    /// 使用 Minimax 算法和 Alpha-Beta 剪枝搜索最优走法。
    /// 返回 None 如果没有合法走法可用。
    pub fn calculate_best_move<T: GameEngine + Clone>(&self, game: &T) -> Option<Move> {
        let current_player = game.get_board_state().current_player;
        let mut best_move: Option<Move> = None;
        let mut best_score = i32::MIN;
        let mut alpha = i32::MIN;
        let beta = i32::MAX;

        // 获取所有可能的走法
        let all_moves = self.get_all_possible_moves(game);

        // 如果没有合法走法，返回 None
        if all_moves.is_empty() {
            return None;
        }

        // 对每个可能的走法进行评估
        for mv in all_moves {
            let mut game_copy = game.clone();
            
            // 尝试执行走法
            if game_copy.make_move(mv.from, mv.to).is_ok() {
                // 使用 minimax 评估这个走法
                let score = self.minimax(
                    &game_copy,
                    self.search_depth - 1,
                    alpha,
                    beta,
                    false, // 下一层是对手的回合，所以是最小化
                    current_player,
                );

                // 更新最佳走法
                if score > best_score {
                    best_score = score;
                    best_move = Some(mv);
                }

                // 更新 alpha 值
                alpha = alpha.max(score);
            }
        }

        best_move
    }

    /// Minimax 算法实现（带 Alpha-Beta 剪枝）
    /// 
    /// # 参数
    /// - `game`: 当前游戏状态
    /// - `depth`: 剩余搜索深度
    /// - `alpha`: Alpha 值（最大化玩家的最佳选择）
    /// - `beta`: Beta 值（最小化玩家的最佳选择）
    /// - `maximizing`: 是否是最大化玩家的回合
    /// - `ai_player`: AI 玩家（用于评估函数）
    /// 
    /// # 返回
    /// 当前位置的评估分数
    fn minimax<T: GameEngine + Clone>(
        &self,
        game: &T,
        depth: u8,
        mut alpha: i32,
        mut beta: i32,
        maximizing: bool,
        ai_player: Player,
    ) -> i32 {
        // 终止条件：达到搜索深度或游戏结束
        if depth == 0 || game.is_game_over() {
            return self.evaluate_position(game, ai_player);
        }

        if maximizing {
            // 最大化玩家的回合
            let mut max_eval = i32::MIN;
            let all_moves = self.get_all_possible_moves(game);

            for mv in all_moves {
                let mut game_copy = game.clone();
                
                if game_copy.make_move(mv.from, mv.to).is_ok() {
                    let eval = self.minimax(
                        &game_copy,
                        depth - 1,
                        alpha,
                        beta,
                        false,
                        ai_player,
                    );
                    max_eval = max_eval.max(eval);
                    alpha = alpha.max(eval);

                    // Beta 剪枝
                    if beta <= alpha {
                        break;
                    }
                }
            }

            max_eval
        } else {
            // 最小化玩家的回合
            let mut min_eval = i32::MAX;
            let all_moves = self.get_all_possible_moves(game);

            for mv in all_moves {
                let mut game_copy = game.clone();
                
                if game_copy.make_move(mv.from, mv.to).is_ok() {
                    let eval = self.minimax(
                        &game_copy,
                        depth - 1,
                        alpha,
                        beta,
                        true,
                        ai_player,
                    );
                    min_eval = min_eval.min(eval);
                    beta = beta.min(eval);

                    // Alpha 剪枝
                    if beta <= alpha {
                        break;
                    }
                }
            }

            min_eval
        }
    }

    /// 评估当前位置的分数
    /// 
    /// 实现复杂的位置评估函数，考虑：
    /// - 棋子材料价值
    /// - 棋子位置优势
    /// - 控制力和威胁
    /// - 游戏结束状态
    /// 
    /// # 评估策略
    /// - 如果 AI 获胜：返回非常高的分数
    /// - 如果 AI 失败：返回非常低的分数
    /// - 否则：计算材料价值 + 位置价值 + 移动性
    fn evaluate_position<T: GameEngine>(&self, game: &T, ai_player: Player) -> i32 {
        // 首先检查游戏是否结束
        if game.is_game_over() {
            if let Some(winner) = game.get_winner() {
                if winner == ai_player {
                    // AI 获胜
                    return 10000;
                } else {
                    // AI 失败
                    return -10000;
                }
            }
            // 平局
            return 0;
        }

        let board_state = game.get_board_state();
        let mut score = 0;

        // 遍历所有棋子，计算材料价值和位置价值
        for (position, piece) in &board_state.pieces {
            let piece_value = self.get_piece_value(&piece.piece_type, *position);
            
            if piece.player == ai_player {
                // AI 的棋子，增加分数
                score += piece_value;
            } else {
                // 对手的棋子，减少分数
                score -= piece_value;
            }
        }

        // 计算移动性（可移动的合法走法数量）
        let ai_mobility = self.calculate_mobility(game, ai_player);
        let opponent_mobility = self.calculate_mobility(game, ai_player.opponent());
        score += (ai_mobility - opponent_mobility) * 10;

        score
    }

    /// 获取棋子的价值（材料价值 + 位置价值）
    fn get_piece_value(&self, piece_type: &PieceType, position: Position) -> i32 {
        match piece_type {
            PieceType::Xiangqi(xiangqi_piece) => {
                self.get_xiangqi_piece_value(*xiangqi_piece, position)
            }
            PieceType::Junqi(junqi_piece) => {
                self.get_junqi_piece_value(*junqi_piece, position)
            }
        }
    }

    /// 获取象棋棋子的价值
    /// 
    /// 材料价值基于传统象棋棋子价值：
    /// - 将/帅：无价（游戏结束条件）
    /// - 车：1000
    /// - 马：450
    /// - 炮：450
    /// - 象/相：200
    /// - 士：200
    /// - 兵/卒：100（过河后增加）
    /// 
    /// 位置价值：鼓励棋子占据中心和关键位置
    fn get_xiangqi_piece_value(&self, piece: XiangqiPiece, position: Position) -> i32 {
        // 基础材料价值
        let material_value = match piece {
            XiangqiPiece::General => 10000,  // 将/帅是最重要的
            XiangqiPiece::Chariot => 1000,   // 车
            XiangqiPiece::Horse => 450,      // 马
            XiangqiPiece::Cannon => 450,     // 炮
            XiangqiPiece::Elephant => 200,   // 象/相
            XiangqiPiece::Advisor => 200,    // 士
            XiangqiPiece::Soldier => 100,    // 兵/卒
        };

        // 位置价值
        let position_value = match piece {
            XiangqiPiece::Soldier => {
                // 兵/卒：过河后价值增加，越靠近对方底线价值越高
                if position.row >= 5 {
                    // 红方兵过河
                    let advance = position.row - 5;
                    20 + (advance as i32 * 10)
                } else if position.row <= 4 {
                    // 黑方卒过河
                    let advance = 4 - position.row;
                    20 + (advance as i32 * 10)
                } else {
                    0
                }
            }
            XiangqiPiece::Horse | XiangqiPiece::Cannon => {
                // 马和炮：中心位置价值更高
                let center_distance = ((position.row as i32 - 5).abs() + (position.col as i32 - 4).abs()) as i32;
                20 - center_distance * 2
            }
            XiangqiPiece::Chariot => {
                // 车：控制开放线路价值更高
                10
            }
            _ => 0,
        };

        material_value + position_value
    }

    /// 获取军棋棋子的价值
    /// 
    /// 材料价值基于军棋棋子等级：
    /// - 军旗：无价（游戏结束条件）
    /// - 司令：900
    /// - 军长：800
    /// - 师长：700
    /// - 旅长：600
    /// - 团长：500
    /// - 营长：400
    /// - 连长：300
    /// - 排长：200
    /// - 工兵：150（可以挖地雷）
    /// - 炸弹：300（可以同归于尽）
    /// - 地雷：250（防御价值）
    fn get_junqi_piece_value(&self, piece: JunqiPiece, _position: Position) -> i32 {
        match piece {
            JunqiPiece::Flag => 10000,      // 军旗是最重要的
            JunqiPiece::Commander => 900,   // 司令
            JunqiPiece::General => 800,     // 军长
            JunqiPiece::Major => 700,       // 师长
            JunqiPiece::Colonel => 600,     // 旅长
            JunqiPiece::Captain => 500,     // 团长
            JunqiPiece::Battalion => 400,   // 营长
            JunqiPiece::Company => 300,     // 连长
            JunqiPiece::Platoon => 200,     // 排长
            JunqiPiece::Engineer => 150,    // 工兵（可以挖地雷）
            JunqiPiece::Bomb => 300,        // 炸弹（可以同归于尽）
            JunqiPiece::Landmine => 250,    // 地雷（防御价值）
        }
    }

    /// 计算移动性（可移动的合法走法数量）
    /// 
    /// 移动性是一个重要的评估因素，表示玩家的灵活性和控制力。
    /// 更多的合法走法意味着更多的选择和更好的位置。
    fn calculate_mobility<T: GameEngine>(&self, game: &T, player: Player) -> i32 {
        let board_state = game.get_board_state();
        let mut mobility = 0;

        // 遍历所有属于该玩家的棋子
        for (position, piece) in &board_state.pieces {
            if piece.player == player {
                // 计算该棋子的合法走法数量
                let legal_moves = game.get_legal_moves(*position);
                mobility += legal_moves.len() as i32;
            }
        }

        mobility
    }

    /// 获取所有可能的走法
    /// 
    /// 遍历棋盘上所有属于当前玩家的棋子，
    /// 并获取每个棋子的所有合法走法。
    fn get_all_possible_moves<T: GameEngine>(&self, game: &T) -> Vec<Move> {
        let board_state = game.get_board_state();
        let current_player = board_state.current_player;
        let mut moves = Vec::new();

        // 遍历棋盘上的所有棋子
        for (position, piece) in &board_state.pieces {
            // 只考虑当前玩家的棋子
            if piece.player == current_player {
                // 获取该棋子的所有合法走法
                let legal_moves = game.get_legal_moves(*position);
                
                // 将所有合法走法添加到列表中
                for to in legal_moves {
                    moves.push(Move {
                        from: *position,
                        to,
                    });
                }
            }
        }

        moves
    }
}

// 属性测试已移动到 tests/ 目录
// #[cfg(test)]
// #[path = "ai_engine_property_tests.rs"]
// mod ai_engine_property_tests;

