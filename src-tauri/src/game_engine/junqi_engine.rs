// JunqiEngine - 军棋游戏引擎
use crate::models::*;
use super::game_engine_trait::{GameEngine, GameResult, GameError};
use std::collections::HashSet;

// 战斗结果
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleResult {
    AttackerWins,  // 攻击方获胜
    DefenderWins,  // 防守方获胜
    BothDie,       // 同归于尽
}

// 军棋棋盘尺寸
pub const JUNQI_ROWS: u8 = 12;
pub const JUNQI_COLS: u8 = 5;

// 营地位置（每方5个营地）
// 红方营地（底部）
pub const RED_CAMP_POSITIONS: [(u8, u8); 5] = [
    (10, 1), (10, 3),  // 第10行的两个营地
    (11, 0), (11, 2), (11, 4),  // 第11行的三个营地
];

// 黑方营地（顶部）
pub const BLACK_CAMP_POSITIONS: [(u8, u8); 5] = [
    (0, 0), (0, 2), (0, 4),  // 第0行的三个营地
    (1, 1), (1, 3),  // 第1行的两个营地
];

// 铁路线位置（军棋中铁路线连接各个位置，工兵可以沿铁路线快速移动）
// 铁路线包括所有非营地的可移动位置
lazy_static::lazy_static! {
    pub static ref RAILWAY_POSITIONS: HashSet<(u8, u8)> = {
        let mut set = HashSet::new();
        
        // 铁路线包括棋盘上除了营地和行营之外的所有位置
        // 简化实现：铁路线包括所有位置，但营地不在铁路线上
        for row in 0..JUNQI_ROWS {
            for col in 0..JUNQI_COLS {
                let pos = (row, col);
                // 检查是否是营地
                if !RED_CAMP_POSITIONS.contains(&pos) && !BLACK_CAMP_POSITIONS.contains(&pos) {
                    set.insert(pos);
                }
            }
        }
        
        set
    };
}

#[derive(Clone)]
pub struct JunqiEngine {
    board_state: BoardState,
    game_status: GameStatus,
}

impl GameEngine for JunqiEngine {
    fn new_game() -> Self {
        let mut board_state = BoardState::new();
        
        // 初始化军棋开局
        // 军棋标准布局：每方25个棋子
        // 黑方（上方，第0-1行）
        // 第0行（从左到右）：军旗、地雷、炸弹、地雷、军旗位置通常在后排
        // 标准开局布局（这是一个示例布局，实际游戏中玩家可以自由布局）
        
        // 黑方布局（第0-1行）
        // 第0行：后排，包含军旗和地雷
        board_state.pieces.insert(Position::new(0, 0), Piece::new(PieceType::Junqi(JunqiPiece::Landmine), Player::Black));
        board_state.pieces.insert(Position::new(0, 1), Piece::new(PieceType::Junqi(JunqiPiece::Flag), Player::Black));
        board_state.pieces.insert(Position::new(0, 2), Piece::new(PieceType::Junqi(JunqiPiece::Landmine), Player::Black));
        board_state.pieces.insert(Position::new(0, 3), Piece::new(PieceType::Junqi(JunqiPiece::Bomb), Player::Black));
        board_state.pieces.insert(Position::new(0, 4), Piece::new(PieceType::Junqi(JunqiPiece::Landmine), Player::Black));
        
        // 第1行：前排战斗棋子
        board_state.pieces.insert(Position::new(1, 0), Piece::new(PieceType::Junqi(JunqiPiece::Commander), Player::Black));
        board_state.pieces.insert(Position::new(1, 1), Piece::new(PieceType::Junqi(JunqiPiece::General), Player::Black));
        board_state.pieces.insert(Position::new(1, 2), Piece::new(PieceType::Junqi(JunqiPiece::Major), Player::Black));
        board_state.pieces.insert(Position::new(1, 3), Piece::new(PieceType::Junqi(JunqiPiece::Colonel), Player::Black));
        board_state.pieces.insert(Position::new(1, 4), Piece::new(PieceType::Junqi(JunqiPiece::Captain), Player::Black));
        
        // 第2行：中排战斗棋子
        board_state.pieces.insert(Position::new(2, 0), Piece::new(PieceType::Junqi(JunqiPiece::Battalion), Player::Black));
        board_state.pieces.insert(Position::new(2, 1), Piece::new(PieceType::Junqi(JunqiPiece::Company), Player::Black));
        board_state.pieces.insert(Position::new(2, 2), Piece::new(PieceType::Junqi(JunqiPiece::Platoon), Player::Black));
        board_state.pieces.insert(Position::new(2, 3), Piece::new(PieceType::Junqi(JunqiPiece::Engineer), Player::Black));
        board_state.pieces.insert(Position::new(2, 4), Piece::new(PieceType::Junqi(JunqiPiece::Engineer), Player::Black));
        
        // 第3行：补充棋子
        board_state.pieces.insert(Position::new(3, 0), Piece::new(PieceType::Junqi(JunqiPiece::Engineer), Player::Black));
        board_state.pieces.insert(Position::new(3, 1), Piece::new(PieceType::Junqi(JunqiPiece::Bomb), Player::Black));
        board_state.pieces.insert(Position::new(3, 2), Piece::new(PieceType::Junqi(JunqiPiece::Colonel), Player::Black));
        board_state.pieces.insert(Position::new(3, 3), Piece::new(PieceType::Junqi(JunqiPiece::Captain), Player::Black));
        board_state.pieces.insert(Position::new(3, 4), Piece::new(PieceType::Junqi(JunqiPiece::Battalion), Player::Black));
        
        // 第4行：最前排
        board_state.pieces.insert(Position::new(4, 0), Piece::new(PieceType::Junqi(JunqiPiece::Company), Player::Black));
        board_state.pieces.insert(Position::new(4, 1), Piece::new(PieceType::Junqi(JunqiPiece::Platoon), Player::Black));
        board_state.pieces.insert(Position::new(4, 2), Piece::new(PieceType::Junqi(JunqiPiece::Major), Player::Black));
        board_state.pieces.insert(Position::new(4, 3), Piece::new(PieceType::Junqi(JunqiPiece::General), Player::Black));
        board_state.pieces.insert(Position::new(4, 4), Piece::new(PieceType::Junqi(JunqiPiece::Company), Player::Black));
        
        // 红方布局（第7-11行，镜像布局）
        // 第7行：最前排
        board_state.pieces.insert(Position::new(7, 0), Piece::new(PieceType::Junqi(JunqiPiece::Company), Player::Red));
        board_state.pieces.insert(Position::new(7, 1), Piece::new(PieceType::Junqi(JunqiPiece::Platoon), Player::Red));
        board_state.pieces.insert(Position::new(7, 2), Piece::new(PieceType::Junqi(JunqiPiece::Major), Player::Red));
        board_state.pieces.insert(Position::new(7, 3), Piece::new(PieceType::Junqi(JunqiPiece::General), Player::Red));
        board_state.pieces.insert(Position::new(7, 4), Piece::new(PieceType::Junqi(JunqiPiece::Company), Player::Red));
        
        // 第8行：补充棋子
        board_state.pieces.insert(Position::new(8, 0), Piece::new(PieceType::Junqi(JunqiPiece::Battalion), Player::Red));
        board_state.pieces.insert(Position::new(8, 1), Piece::new(PieceType::Junqi(JunqiPiece::Captain), Player::Red));
        board_state.pieces.insert(Position::new(8, 2), Piece::new(PieceType::Junqi(JunqiPiece::Colonel), Player::Red));
        board_state.pieces.insert(Position::new(8, 3), Piece::new(PieceType::Junqi(JunqiPiece::Bomb), Player::Red));
        board_state.pieces.insert(Position::new(8, 4), Piece::new(PieceType::Junqi(JunqiPiece::Engineer), Player::Red));
        
        // 第9行：中排战斗棋子
        board_state.pieces.insert(Position::new(9, 0), Piece::new(PieceType::Junqi(JunqiPiece::Engineer), Player::Red));
        board_state.pieces.insert(Position::new(9, 1), Piece::new(PieceType::Junqi(JunqiPiece::Engineer), Player::Red));
        board_state.pieces.insert(Position::new(9, 2), Piece::new(PieceType::Junqi(JunqiPiece::Platoon), Player::Red));
        board_state.pieces.insert(Position::new(9, 3), Piece::new(PieceType::Junqi(JunqiPiece::Company), Player::Red));
        board_state.pieces.insert(Position::new(9, 4), Piece::new(PieceType::Junqi(JunqiPiece::Battalion), Player::Red));
        
        // 第10行：前排战斗棋子
        board_state.pieces.insert(Position::new(10, 0), Piece::new(PieceType::Junqi(JunqiPiece::Captain), Player::Red));
        board_state.pieces.insert(Position::new(10, 1), Piece::new(PieceType::Junqi(JunqiPiece::Colonel), Player::Red));
        board_state.pieces.insert(Position::new(10, 2), Piece::new(PieceType::Junqi(JunqiPiece::Major), Player::Red));
        board_state.pieces.insert(Position::new(10, 3), Piece::new(PieceType::Junqi(JunqiPiece::General), Player::Red));
        board_state.pieces.insert(Position::new(10, 4), Piece::new(PieceType::Junqi(JunqiPiece::Commander), Player::Red));
        
        // 第11行：后排，包含军旗和地雷
        board_state.pieces.insert(Position::new(11, 0), Piece::new(PieceType::Junqi(JunqiPiece::Landmine), Player::Red));
        board_state.pieces.insert(Position::new(11, 1), Piece::new(PieceType::Junqi(JunqiPiece::Bomb), Player::Red));
        board_state.pieces.insert(Position::new(11, 2), Piece::new(PieceType::Junqi(JunqiPiece::Flag), Player::Red));
        board_state.pieces.insert(Position::new(11, 3), Piece::new(PieceType::Junqi(JunqiPiece::Landmine), Player::Red));
        board_state.pieces.insert(Position::new(11, 4), Piece::new(PieceType::Junqi(JunqiPiece::Landmine), Player::Red));
        
        // 红方先行
        board_state.current_player = Player::Red;
        
        Self {
            board_state,
            game_status: GameStatus::Ongoing,
        }
    }

    fn get_board_state(&self) -> &BoardState {
        &self.board_state
    }

    fn get_legal_moves(&self, position: Position) -> Vec<Position> {
        // 获取指定位置的棋子
        let piece = match self.board_state.pieces.get(&position) {
            Some(p) => p,
            None => return vec![], // 位置上没有棋子
        };

        // 只能移动当前玩家的棋子
        if piece.player != self.board_state.current_player {
            return vec![];
        }

        // 检查棋子类型，某些棋子不能移动
        if let PieceType::Junqi(junqi_piece) = piece.piece_type {
            match junqi_piece {
                JunqiPiece::Flag | JunqiPiece::Landmine => {
                    // 军旗和地雷不能移动
                    return vec![];
                }
                _ => {}
            }
        }

        let mut legal_moves = Vec::new();

        // 根据棋子类型生成所有可能的移动
        if let PieceType::Junqi(junqi_piece) = piece.piece_type {
            match junqi_piece {
                JunqiPiece::Engineer => {
                    // 工兵在铁路线上可以沿铁路线任意移动
                    if RAILWAY_POSITIONS.contains(&(position.row, position.col)) {
                        self.get_engineer_railway_moves(position, &mut legal_moves);
                    } else {
                        // 不在铁路线上，只能移动一格
                        self.get_basic_moves(position, piece.player, &mut legal_moves);
                    }
                }
                _ => {
                    // 其他棋子只能移动一格（上下左右）
                    self.get_basic_moves(position, piece.player, &mut legal_moves);
                }
            }
        }

        legal_moves
    }

    fn make_move(&mut self, from: Position, to: Position) -> GameResult<()> {
        // 验证起始位置有棋子
        let piece = self.board_state.pieces.get(&from)
            .ok_or_else(|| GameError::InvalidInput {
                message: format!("起始位置 ({}, {}) 没有棋子", from.row, from.col)
            })?
            .clone();

        // 验证是当前玩家的棋子
        if piece.player != self.board_state.current_player {
            return Err(GameError::IllegalMove {
                from,
                to,
                reason: "不能移动对手的棋子".to_string()
            });
        }

        // 验证移动是否合法
        let legal_moves = self.get_legal_moves(from);
        if !legal_moves.contains(&to) {
            return Err(GameError::IllegalMove {
                from,
                to,
                reason: format!("该移动不在合法移动列表中")
            });
        }

        // 检查目标位置是否有棋子（战斗）
        let captured_piece = if let Some(target_piece) = self.board_state.pieces.get(&to).cloned() {
            // 发生战斗
            let battle_result = self.resolve_battle(&piece, &target_piece);
            
            match battle_result {
                BattleResult::AttackerWins => {
                    // 攻击方获胜，移除防守方棋子，攻击方移动到目标位置
                    self.board_state.pieces.remove(&to);
                    self.board_state.pieces.remove(&from);
                    self.board_state.pieces.insert(to, piece);
                    Some(target_piece)
                }
                BattleResult::DefenderWins => {
                    // 防守方获胜，移除攻击方棋子
                    self.board_state.pieces.remove(&from);
                    Some(piece)
                }
                BattleResult::BothDie => {
                    // 同归于尽，移除双方棋子
                    self.board_state.pieces.remove(&from);
                    self.board_state.pieces.remove(&to);
                    Some(target_piece)
                }
            }
        } else {
            // 目标位置为空，直接移动
            self.board_state.pieces.remove(&from);
            self.board_state.pieces.insert(to, piece);
            None
        };

        // 记录移动历史
        let move_record = Move {
            from,
            to,
            piece,
            captured_piece,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        self.board_state.move_history.push(move_record);

        // 切换玩家
        self.board_state.current_player = self.board_state.current_player.opponent();

        // 更新游戏状态
        if self.is_game_over() {
            if let Some(winner) = self.get_winner() {
                self.game_status = GameStatus::Victory { winner };
            }
        }

        Ok(())
    }

    fn is_game_over(&self) -> bool {
        // 军棋游戏结束条件：一方军旗被夺取
        self.is_flag_captured(Player::Red) || self.is_flag_captured(Player::Black)
    }

    fn get_winner(&self) -> Option<Player> {
        // 如果红方军旗被夺取，黑方获胜
        if self.is_flag_captured(Player::Red) {
            return Some(Player::Black);
        }
        
        // 如果黑方军旗被夺取，红方获胜
        if self.is_flag_captured(Player::Black) {
            return Some(Player::Red);
        }
        
        // 游戏未结束
        None
    }

    fn undo_move(&mut self) -> GameResult<()> {
        // 检查是否有移动历史
        if self.board_state.move_history.is_empty() {
            return Err(GameError::InvalidState {
                message: "没有可以悔棋的移动".to_string(),
            });
        }
        
        // 获取最后一次移动
        let last_move = self.board_state.move_history.pop().unwrap();
        
        // 恢复棋子到原位置
        // 注意：在军棋中，战斗可能导致三种结果：
        // 1. 攻击方获胜：攻击方移动到目标位置，防守方被移除
        //    - 目标位置有攻击方棋子，需要移除并恢复到起始位置
        //    - captured_piece 是防守方棋子，需要恢复到目标位置
        // 2. 防守方获胜：攻击方被移除，防守方留在原位
        //    - 目标位置有防守方棋子（captured_piece），不需要移除
        //    - 攻击方棋子（last_move.piece）需要恢复到起始位置
        // 3. 同归于尽：双方都被移除
        //    - 目标位置为空
        //    - 攻击方棋子需要恢复到起始位置
        //    - 防守方棋子（captured_piece）需要恢复到目标位置
        
        // 检查目标位置是否有棋子
        let piece_at_target = self.board_state.pieces.get(&last_move.to).cloned();
        
        // 判断战斗结果并恢复状态
        if let Some(captured_piece) = last_move.captured_piece.clone() {
            // 发生了战斗
            if let Some(current_piece) = piece_at_target {
                // 目标位置有棋子
                if current_piece == last_move.piece {
                    // 攻击方获胜的情况：目标位置是攻击方棋子
                    self.board_state.pieces.remove(&last_move.to);
                    self.board_state.pieces.insert(last_move.from, last_move.piece);
                    self.board_state.pieces.insert(last_move.to, captured_piece);
                } else {
                    // 防守方获胜的情况：目标位置是防守方棋子（captured_piece）
                    // 目标位置的棋子不变，只需恢复攻击方棋子到起始位置
                    self.board_state.pieces.insert(last_move.from, last_move.piece);
                }
            } else {
                // 同归于尽的情况：目标位置为空
                self.board_state.pieces.insert(last_move.from, last_move.piece);
                self.board_state.pieces.insert(last_move.to, captured_piece);
            }
        } else {
            // 没有战斗，简单移动
            self.board_state.pieces.remove(&last_move.to);
            self.board_state.pieces.insert(last_move.from, last_move.piece);
        }
        
        // 切换回上一个玩家
        self.board_state.current_player = self.board_state.current_player.opponent();
        
        // 更新游戏状态
        if self.is_game_over() {
            if let Some(winner) = self.get_winner() {
                self.game_status = GameStatus::Victory { winner };
            }
        } else {
            self.game_status = GameStatus::Ongoing;
        }
        
        Ok(())
    }

    fn get_game_status(&self) -> GameStatus {
        self.game_status.clone()
    }
}

impl JunqiEngine {
    /// 从给定的棋盘状态创建引擎（仅用于测试）
    #[cfg(test)]
    pub fn from_state(board_state: BoardState) -> Self {
        Self {
            board_state,
            game_status: GameStatus::Ongoing,
        }
    }
    
    /// 获取棋子等级（用于战斗判定）
    /// 返回值：等级数字，0表示特殊棋子（军旗、地雷、炸弹）
    pub fn get_piece_rank(piece: &JunqiPiece) -> u8 {
        match piece {
            JunqiPiece::Commander => 9,  // 司令
            JunqiPiece::General => 8,    // 军长
            JunqiPiece::Major => 7,      // 师长
            JunqiPiece::Colonel => 6,    // 旅长
            JunqiPiece::Captain => 5,    // 团长
            JunqiPiece::Battalion => 4,  // 营长
            JunqiPiece::Company => 3,    // 连长
            JunqiPiece::Platoon => 2,    // 排长
            JunqiPiece::Engineer => 1,   // 工兵
            JunqiPiece::Flag => 0,       // 军旗
            JunqiPiece::Landmine => 0,   // 地雷
            JunqiPiece::Bomb => 0,       // 炸弹
        }
    }
    
    /// 处理战斗逻辑
    /// 根据军棋规则判定战斗结果：
    /// 1. 炸弹与任何棋子相遇都同归于尽
    /// 2. 地雷只能被工兵排除，其他棋子碰到地雷都会被炸死
    /// 3. 工兵可以排除地雷
    /// 4. 任何棋子（除工兵外）碰到军旗都获胜
    /// 5. 普通战斗：等级高的获胜，等级相同则同归于尽
    pub fn resolve_battle(&self, attacker: &Piece, defender: &Piece) -> BattleResult {
        // 提取军棋棋子类型
        let attacker_piece = match attacker.piece_type {
            PieceType::Junqi(p) => p,
            _ => panic!("攻击方不是军棋棋子"),
        };
        
        let defender_piece = match defender.piece_type {
            PieceType::Junqi(p) => p,
            _ => panic!("防守方不是军棋棋子"),
        };
        
        // 规则1：炸弹与任何棋子相遇都同归于尽
        if matches!(attacker_piece, JunqiPiece::Bomb) || matches!(defender_piece, JunqiPiece::Bomb) {
            return BattleResult::BothDie;
        }
        
        // 规则2：地雷的特殊处理
        if matches!(defender_piece, JunqiPiece::Landmine) {
            // 工兵可以排除地雷
            if matches!(attacker_piece, JunqiPiece::Engineer) {
                return BattleResult::AttackerWins;
            }
            // 其他棋子碰到地雷都会被炸死
            return BattleResult::DefenderWins;
        }
        
        // 规则3：攻击军旗
        if matches!(defender_piece, JunqiPiece::Flag) {
            // 任何棋子攻击军旗都获胜
            return BattleResult::AttackerWins;
        }
        
        // 规则4：普通战斗，比较等级
        let attacker_rank = Self::get_piece_rank(&attacker_piece);
        let defender_rank = Self::get_piece_rank(&defender_piece);
        
        if attacker_rank > defender_rank {
            BattleResult::AttackerWins
        } else if attacker_rank < defender_rank {
            BattleResult::DefenderWins
        } else {
            // 等级相同，同归于尽
            BattleResult::BothDie
        }
    }
    
    /// 获取基础移动（一格，上下左右）
    fn get_basic_moves(&self, from: Position, player: Player, legal_moves: &mut Vec<Position>) {
        // 军棋中，棋子可以向上下左右移动一格
        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        for (row_delta, col_delta) in directions.iter() {
            let new_row = from.row as i8 + row_delta;
            let new_col = from.col as i8 + col_delta;

            // 检查是否在棋盘范围内
            if new_row >= 0 && new_row < JUNQI_ROWS as i8 && new_col >= 0 && new_col < JUNQI_COLS as i8 {
                let to = Position::new(new_row as u8, new_col as u8);
                
                // 检查目标位置是否可以移动（空位或敌方棋子）
                if self.can_move_to(to, player) {
                    legal_moves.push(to);
                }
            }
        }
    }

    /// 获取工兵在铁路线上的移动
    fn get_engineer_railway_moves(&self, from: Position, legal_moves: &mut Vec<Position>) {
        let piece = self.board_state.pieces.get(&from).unwrap();
        let player = piece.player;

        // 工兵在铁路线上可以沿铁路线任意移动（无棋子阻挡）
        // 使用 BFS 或 DFS 找到所有可达的铁路位置
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        
        visited.insert((from.row, from.col));
        queue.push_back(from);

        while let Some(current) = queue.pop_front() {
            // 检查四个方向
            let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
            
            for (row_delta, col_delta) in directions.iter() {
                let new_row = current.row as i8 + row_delta;
                let new_col = current.col as i8 + col_delta;

                // 检查是否在棋盘范围内
                if new_row >= 0 && new_row < JUNQI_ROWS as i8 && new_col >= 0 && new_col < JUNQI_COLS as i8 {
                    let next_pos = (new_row as u8, new_col as u8);
                    
                    // 检查是否已访问
                    if visited.contains(&next_pos) {
                        continue;
                    }

                    // 检查是否在铁路线上
                    if !RAILWAY_POSITIONS.contains(&next_pos) {
                        continue;
                    }

                    let next_position = Position::new(next_pos.0, next_pos.1);
                    
                    // 检查该位置是否有棋子
                    if let Some(piece_at_pos) = self.board_state.pieces.get(&next_position) {
                        // 如果有敌方棋子，可以攻击但不能继续前进
                        if piece_at_pos.player != player {
                            legal_moves.push(next_position);
                        }
                        // 有棋子阻挡，不能继续前进
                        continue;
                    }

                    // 空位，可以移动并继续搜索
                    legal_moves.push(next_position);
                    visited.insert(next_pos);
                    queue.push_back(next_position);
                }
            }
        }
    }

    /// 检查目标位置是否可以移动（空位或敌方棋子）
    fn can_move_to(&self, position: Position, player: Player) -> bool {
        match self.board_state.pieces.get(&position) {
            None => true, // 空位
            Some(piece) => piece.player != player, // 敌方棋子
        }
    }

    /// 检查指定玩家的军旗是否被夺取（不在棋盘上）
    pub fn is_flag_captured(&self, player: Player) -> bool {
        // 遍历棋盘上的所有棋子，查找指定玩家的军旗
        for piece in self.board_state.pieces.values() {
            if piece.player == player {
                if let PieceType::Junqi(JunqiPiece::Flag) = piece.piece_type {
                    // 找到军旗，说明未被夺取
                    return false;
                }
            }
        }
        
        // 未找到军旗，说明已被夺取
        true
    }

    /// 检查工兵是否可以在铁路线上移动到目标位置
    pub fn can_move_on_railway(&self, from: Position, to: Position) -> bool {
        // 检查起始位置是否有工兵
        let piece = match self.board_state.pieces.get(&from) {
            Some(p) => p,
            None => return false,
        };

        // 检查是否是工兵
        if !matches!(piece.piece_type, PieceType::Junqi(JunqiPiece::Engineer)) {
            return false;
        }

        // 检查起始位置是否在铁路线上
        if !RAILWAY_POSITIONS.contains(&(from.row, from.col)) {
            return false;
        }

        // 检查目标位置是否在铁路线上
        if !RAILWAY_POSITIONS.contains(&(to.row, to.col)) {
            return false;
        }

        // 使用 BFS 检查是否可以从 from 到达 to（沿铁路线且无阻挡）
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        
        visited.insert((from.row, from.col));
        queue.push_back(from);

        while let Some(current) = queue.pop_front() {
            // 如果到达目标位置
            if current == to {
                return true;
            }

            // 检查四个方向
            let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
            
            for (row_delta, col_delta) in directions.iter() {
                let new_row = current.row as i8 + row_delta;
                let new_col = current.col as i8 + col_delta;

                // 检查是否在棋盘范围内
                if new_row >= 0 && new_row < JUNQI_ROWS as i8 && new_col >= 0 && new_col < JUNQI_COLS as i8 {
                    let next_pos = (new_row as u8, new_col as u8);
                    
                    // 检查是否已访问
                    if visited.contains(&next_pos) {
                        continue;
                    }

                    // 检查是否在铁路线上
                    if !RAILWAY_POSITIONS.contains(&next_pos) {
                        continue;
                    }

                    let next_position = Position::new(next_pos.0, next_pos.1);
                    
                    // 如果是目标位置，检查是否可以移动到该位置
                    if next_position == to {
                        return self.can_move_to(to, piece.player);
                    }

                    // 检查该位置是否有棋子（阻挡）
                    if self.board_state.pieces.contains_key(&next_position) {
                        continue;
                    }

                    // 空位，继续搜索
                    visited.insert(next_pos);
                    queue.push_back(next_position);
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_junqi_new_game_initialization() {
        let game = JunqiEngine::new_game();
        let board_state = game.get_board_state();
        
        // 验证每方有25个棋子，总共50个
        assert_eq!(board_state.pieces.len(), 50, "军棋开局应该有50个棋子");
        
        // 验证红方先行
        assert_eq!(board_state.current_player, Player::Red, "红方应该先行");
        
        // 验证每方都有军旗
        let mut red_flag_count = 0;
        let mut black_flag_count = 0;
        
        for piece in board_state.pieces.values() {
            if let PieceType::Junqi(JunqiPiece::Flag) = piece.piece_type {
                match piece.player {
                    Player::Red => red_flag_count += 1,
                    Player::Black => black_flag_count += 1,
                }
            }
        }
        
        assert_eq!(red_flag_count, 1, "红方应该有1个军旗");
        assert_eq!(black_flag_count, 1, "黑方应该有1个军旗");
    }

    #[test]
    fn test_junqi_board_dimensions() {
        // 验证棋盘尺寸常量
        assert_eq!(JUNQI_ROWS, 12, "军棋棋盘应该有12行");
        assert_eq!(JUNQI_COLS, 5, "军棋棋盘应该有5列");
    }

    #[test]
    fn test_camp_positions() {
        // 验证营地位置数量
        assert_eq!(RED_CAMP_POSITIONS.len(), 5, "红方应该有5个营地");
        assert_eq!(BLACK_CAMP_POSITIONS.len(), 5, "黑方应该有5个营地");
        
        // 验证营地位置在棋盘范围内
        for (row, col) in RED_CAMP_POSITIONS.iter() {
            assert!(*row < JUNQI_ROWS && *col < JUNQI_COLS, "红方营地位置应该在棋盘范围内");
        }
        
        for (row, col) in BLACK_CAMP_POSITIONS.iter() {
            assert!(*row < JUNQI_ROWS && *col < JUNQI_COLS, "黑方营地位置应该在棋盘范围内");
        }
    }

    #[test]
    fn test_railway_positions() {
        // 验证铁路线位置不包含营地
        for (row, col) in RED_CAMP_POSITIONS.iter() {
            assert!(!RAILWAY_POSITIONS.contains(&(*row, *col)), "铁路线不应该包含红方营地");
        }
        
        for (row, col) in BLACK_CAMP_POSITIONS.iter() {
            assert!(!RAILWAY_POSITIONS.contains(&(*row, *col)), "铁路线不应该包含黑方营地");
        }
        
        // 验证铁路线位置数量（总位置60个，减去10个营地）
        assert_eq!(RAILWAY_POSITIONS.len(), 50, "铁路线应该有50个位置");
    }

    #[test]
    fn test_special_pieces_placement() {
        let game = JunqiEngine::new_game();
        let board_state = game.get_board_state();
        
        // 验证军旗、地雷、炸弹等特殊棋子的数量
        let mut red_landmines = 0;
        let mut black_landmines = 0;
        let mut red_bombs = 0;
        let mut black_bombs = 0;
        
        for piece in board_state.pieces.values() {
            match piece.piece_type {
                PieceType::Junqi(JunqiPiece::Landmine) => {
                    match piece.player {
                        Player::Red => red_landmines += 1,
                        Player::Black => black_landmines += 1,
                    }
                }
                PieceType::Junqi(JunqiPiece::Bomb) => {
                    match piece.player {
                        Player::Red => red_bombs += 1,
                        Player::Black => black_bombs += 1,
                    }
                }
                _ => {}
            }
        }
        
        assert_eq!(red_landmines, 3, "红方应该有3个地雷");
        assert_eq!(black_landmines, 3, "黑方应该有3个地雷");
        assert_eq!(red_bombs, 2, "红方应该有2个炸弹");
        assert_eq!(black_bombs, 2, "黑方应该有2个炸弹");
    }

    #[test]
    fn test_flag_and_landmine_cannot_move() {
        let game = JunqiEngine::new_game();
        
        // 测试军旗不能移动
        let red_flag_pos = Position::new(11, 2);
        let legal_moves = game.get_legal_moves(red_flag_pos);
        assert_eq!(legal_moves.len(), 0, "军旗不能移动");
        
        // 测试地雷不能移动
        let red_landmine_pos = Position::new(11, 0);
        let legal_moves = game.get_legal_moves(red_landmine_pos);
        assert_eq!(legal_moves.len(), 0, "地雷不能移动");
    }

    #[test]
    fn test_basic_piece_movement() {
        let mut game = JunqiEngine::new_game();
        
        // 测试红方连长（第7行第0列）的基础移动
        let company_pos = Position::new(7, 0);
        let legal_moves = game.get_legal_moves(company_pos);
        
        // 连长应该可以向上、向右移动（向下和向左超出边界或有己方棋子）
        assert!(legal_moves.len() > 0, "连长应该有合法移动");
        
        // 验证移动方向正确（只能移动一格）
        for move_pos in legal_moves.iter() {
            let row_diff = (move_pos.row as i8 - company_pos.row as i8).abs();
            let col_diff = (move_pos.col as i8 - company_pos.col as i8).abs();
            assert_eq!(row_diff + col_diff, 1, "基础移动只能移动一格");
        }
    }

    #[test]
    fn test_engineer_basic_movement() {
        let mut game = JunqiEngine::new_game();
        
        // 清空一些位置以便工兵可以移动
        game.board_state.pieces.remove(&Position::new(8, 0));
        
        // 测试红方工兵（第9行第0列）的移动
        let engineer_pos = Position::new(9, 0);
        let legal_moves = game.get_legal_moves(engineer_pos);
        
        println!("工兵的合法移动数量: {}", legal_moves.len());
        for move_pos in legal_moves.iter() {
            println!("  可移动到: ({}, {})", move_pos.row, move_pos.col);
        }
        
        // 工兵在铁路线上应该有多个合法移动
        assert!(legal_moves.len() > 0, "工兵应该有合法移动");
    }

    #[test]
    fn test_engineer_railway_movement() {
        let mut game = JunqiEngine::new_game();
        
        // 创建一个简化的测试场景：清空部分棋盘，测试工兵铁路移动
        // 移除一些棋子以创建空间
        game.board_state.pieces.remove(&Position::new(6, 0));
        game.board_state.pieces.remove(&Position::new(7, 0));
        game.board_state.pieces.remove(&Position::new(8, 0));
        
        // 测试工兵在铁路线上的移动
        let engineer_pos = Position::new(9, 0);
        let legal_moves = game.get_legal_moves(engineer_pos);
        
        println!("工兵在 ({}, {}) 的合法移动数量: {}", engineer_pos.row, engineer_pos.col, legal_moves.len());
        for move_pos in legal_moves.iter() {
            println!("  可移动到: ({}, {})", move_pos.row, move_pos.col);
        }
        
        // 工兵在铁路线上应该能移动到多个位置（至少可以向上移动到清空的位置）
        assert!(legal_moves.len() >= 1, "工兵在铁路线上应该能移动到至少一个位置");
        
        // 验证工兵可以移动到清空的位置
        assert!(legal_moves.contains(&Position::new(8, 0)) || 
                legal_moves.contains(&Position::new(6, 0)) ||
                legal_moves.contains(&Position::new(7, 0)),
                "工兵应该能移动到清空的铁路位置");
    }

    #[test]
    fn test_can_move_on_railway() {
        let mut game = JunqiEngine::new_game();
        
        // 清空一些位置以测试铁路移动
        game.board_state.pieces.remove(&Position::new(6, 0));
        game.board_state.pieces.remove(&Position::new(7, 0));
        game.board_state.pieces.remove(&Position::new(8, 0));
        
        let engineer_pos = Position::new(9, 0);
        let target_pos = Position::new(6, 0);
        
        // 测试工兵是否可以沿铁路线移动
        let can_move = game.can_move_on_railway(engineer_pos, target_pos);
        assert!(can_move, "工兵应该能沿铁路线移动到目标位置");
    }

    #[test]
    fn test_cannot_move_opponent_pieces() {
        let game = JunqiEngine::new_game();
        
        // 当前是红方回合，尝试移动黑方棋子
        let black_piece_pos = Position::new(1, 0); // 黑方司令
        let legal_moves = game.get_legal_moves(black_piece_pos);
        
        assert_eq!(legal_moves.len(), 0, "不能移动对手的棋子");
    }

    #[test]
    fn test_get_piece_rank() {
        // 测试所有棋子的等级
        assert_eq!(JunqiEngine::get_piece_rank(&JunqiPiece::Commander), 9);
        assert_eq!(JunqiEngine::get_piece_rank(&JunqiPiece::General), 8);
        assert_eq!(JunqiEngine::get_piece_rank(&JunqiPiece::Major), 7);
        assert_eq!(JunqiEngine::get_piece_rank(&JunqiPiece::Colonel), 6);
        assert_eq!(JunqiEngine::get_piece_rank(&JunqiPiece::Captain), 5);
        assert_eq!(JunqiEngine::get_piece_rank(&JunqiPiece::Battalion), 4);
        assert_eq!(JunqiEngine::get_piece_rank(&JunqiPiece::Company), 3);
        assert_eq!(JunqiEngine::get_piece_rank(&JunqiPiece::Platoon), 2);
        assert_eq!(JunqiEngine::get_piece_rank(&JunqiPiece::Engineer), 1);
        assert_eq!(JunqiEngine::get_piece_rank(&JunqiPiece::Flag), 0);
        assert_eq!(JunqiEngine::get_piece_rank(&JunqiPiece::Landmine), 0);
        assert_eq!(JunqiEngine::get_piece_rank(&JunqiPiece::Bomb), 0);
    }

    #[test]
    fn test_battle_higher_rank_wins() {
        let game = JunqiEngine::new_game();
        
        // 司令（等级9）攻击军长（等级8）
        let attacker = Piece::new(PieceType::Junqi(JunqiPiece::Commander), Player::Red);
        let defender = Piece::new(PieceType::Junqi(JunqiPiece::General), Player::Black);
        
        let result = game.resolve_battle(&attacker, &defender);
        assert_eq!(result, BattleResult::AttackerWins, "等级高的应该获胜");
    }

    #[test]
    fn test_battle_lower_rank_loses() {
        let game = JunqiEngine::new_game();
        
        // 排长（等级2）攻击连长（等级3）
        let attacker = Piece::new(PieceType::Junqi(JunqiPiece::Platoon), Player::Red);
        let defender = Piece::new(PieceType::Junqi(JunqiPiece::Company), Player::Black);
        
        let result = game.resolve_battle(&attacker, &defender);
        assert_eq!(result, BattleResult::DefenderWins, "等级低的应该失败");
    }

    #[test]
    fn test_battle_same_rank_both_die() {
        let game = JunqiEngine::new_game();
        
        // 连长（等级3）攻击连长（等级3）
        let attacker = Piece::new(PieceType::Junqi(JunqiPiece::Company), Player::Red);
        let defender = Piece::new(PieceType::Junqi(JunqiPiece::Company), Player::Black);
        
        let result = game.resolve_battle(&attacker, &defender);
        assert_eq!(result, BattleResult::BothDie, "等级相同应该同归于尽");
    }

    #[test]
    fn test_bomb_kills_all() {
        let game = JunqiEngine::new_game();
        
        // 炸弹攻击司令
        let attacker = Piece::new(PieceType::Junqi(JunqiPiece::Bomb), Player::Red);
        let defender = Piece::new(PieceType::Junqi(JunqiPiece::Commander), Player::Black);
        
        let result = game.resolve_battle(&attacker, &defender);
        assert_eq!(result, BattleResult::BothDie, "炸弹应该与任何棋子同归于尽");
        
        // 司令攻击炸弹
        let attacker = Piece::new(PieceType::Junqi(JunqiPiece::Commander), Player::Red);
        let defender = Piece::new(PieceType::Junqi(JunqiPiece::Bomb), Player::Black);
        
        let result = game.resolve_battle(&attacker, &defender);
        assert_eq!(result, BattleResult::BothDie, "任何棋子碰到炸弹都同归于尽");
    }

    #[test]
    fn test_engineer_can_remove_landmine() {
        let game = JunqiEngine::new_game();
        
        // 工兵攻击地雷
        let attacker = Piece::new(PieceType::Junqi(JunqiPiece::Engineer), Player::Red);
        let defender = Piece::new(PieceType::Junqi(JunqiPiece::Landmine), Player::Black);
        
        let result = game.resolve_battle(&attacker, &defender);
        assert_eq!(result, BattleResult::AttackerWins, "工兵应该能排除地雷");
    }

    #[test]
    fn test_landmine_kills_non_engineer() {
        let game = JunqiEngine::new_game();
        
        // 司令攻击地雷
        let attacker = Piece::new(PieceType::Junqi(JunqiPiece::Commander), Player::Red);
        let defender = Piece::new(PieceType::Junqi(JunqiPiece::Landmine), Player::Black);
        
        let result = game.resolve_battle(&attacker, &defender);
        assert_eq!(result, BattleResult::DefenderWins, "非工兵碰到地雷应该被炸死");
        
        // 排长攻击地雷
        let attacker = Piece::new(PieceType::Junqi(JunqiPiece::Platoon), Player::Red);
        let defender = Piece::new(PieceType::Junqi(JunqiPiece::Landmine), Player::Black);
        
        let result = game.resolve_battle(&attacker, &defender);
        assert_eq!(result, BattleResult::DefenderWins, "非工兵碰到地雷应该被炸死");
    }

    #[test]
    fn test_capture_flag_wins() {
        let game = JunqiEngine::new_game();
        
        // 任何棋子攻击军旗都获胜
        let pieces = vec![
            JunqiPiece::Commander,
            JunqiPiece::General,
            JunqiPiece::Engineer,
            JunqiPiece::Platoon,
        ];
        
        for piece_type in pieces {
            let attacker = Piece::new(PieceType::Junqi(piece_type), Player::Red);
            let defender = Piece::new(PieceType::Junqi(JunqiPiece::Flag), Player::Black);
            
            let result = game.resolve_battle(&attacker, &defender);
            assert_eq!(result, BattleResult::AttackerWins, "攻击军旗应该获胜");
        }
    }

    #[test]
    fn test_make_move_empty_square() {
        let mut game = JunqiEngine::new_game();
        
        // 清空一些位置以便测试移动
        game.board_state.pieces.remove(&Position::new(6, 0));
        
        // 移动红方连长从 (7, 0) 到 (6, 0)
        let from = Position::new(7, 0);
        let to = Position::new(6, 0);
        
        let result = game.make_move(from, to);
        assert!(result.is_ok(), "移动到空位应该成功");
        
        // 验证棋子已移动
        assert!(game.board_state.pieces.get(&from).is_none(), "起始位置应该为空");
        assert!(game.board_state.pieces.get(&to).is_some(), "目标位置应该有棋子");
        
        // 验证玩家已切换
        assert_eq!(game.board_state.current_player, Player::Black, "应该切换到黑方");
    }

    #[test]
    fn test_make_move_capture_piece() {
        let mut game = JunqiEngine::new_game();
        
        // 设置一个简单的战斗场景
        // 将红方司令移到 (6, 0)，黑方排长在 (5, 0)
        game.board_state.pieces.remove(&Position::new(10, 4)); // 移除原位置的红方司令
        game.board_state.pieces.insert(
            Position::new(6, 0),
            Piece::new(PieceType::Junqi(JunqiPiece::Commander), Player::Red)
        );
        game.board_state.pieces.remove(&Position::new(2, 2)); // 移除原位置的黑方排长
        game.board_state.pieces.insert(
            Position::new(5, 0),
            Piece::new(PieceType::Junqi(JunqiPiece::Platoon), Player::Black)
        );
        
        // 红方司令攻击黑方排长
        let from = Position::new(6, 0);
        let to = Position::new(5, 0);
        
        let result = game.make_move(from, to);
        assert!(result.is_ok(), "战斗移动应该成功");
        
        // 验证司令获胜并占据目标位置
        assert!(game.board_state.pieces.get(&from).is_none(), "起始位置应该为空");
        let piece_at_to = game.board_state.pieces.get(&to);
        assert!(piece_at_to.is_some(), "目标位置应该有棋子");
        assert_eq!(piece_at_to.unwrap().piece_type, PieceType::Junqi(JunqiPiece::Commander), "应该是司令");
        assert_eq!(piece_at_to.unwrap().player, Player::Red, "应该是红方");
    }

    #[test]
    fn test_make_move_both_die() {
        let mut game = JunqiEngine::new_game();
        
        // 设置同等级战斗场景
        // 将红方连长移到 (6, 0)，黑方连长在 (5, 0)
        game.board_state.pieces.remove(&Position::new(7, 0)); // 移除原位置的红方连长
        game.board_state.pieces.insert(
            Position::new(6, 0),
            Piece::new(PieceType::Junqi(JunqiPiece::Company), Player::Red)
        );
        game.board_state.pieces.remove(&Position::new(2, 1)); // 移除原位置的黑方连长
        game.board_state.pieces.insert(
            Position::new(5, 0),
            Piece::new(PieceType::Junqi(JunqiPiece::Company), Player::Black)
        );
        
        // 红方连长攻击黑方连长
        let from = Position::new(6, 0);
        let to = Position::new(5, 0);
        
        let result = game.make_move(from, to);
        assert!(result.is_ok(), "战斗移动应该成功");
        
        // 验证双方都被移除
        assert!(game.board_state.pieces.get(&from).is_none(), "起始位置应该为空");
        assert!(game.board_state.pieces.get(&to).is_none(), "目标位置应该为空（同归于尽）");
    }

    #[test]
    fn test_is_flag_captured_initial_state() {
        let game = JunqiEngine::new_game();
        
        // 初始状态下，双方军旗都应该在棋盘上
        assert!(!game.is_flag_captured(Player::Red), "红方军旗应该在棋盘上");
        assert!(!game.is_flag_captured(Player::Black), "黑方军旗应该在棋盘上");
    }

    #[test]
    fn test_is_flag_captured_after_removal() {
        let mut game = JunqiEngine::new_game();
        
        // 移除红方军旗
        game.board_state.pieces.remove(&Position::new(11, 2));
        
        // 红方军旗应该被夺取
        assert!(game.is_flag_captured(Player::Red), "红方军旗应该被夺取");
        assert!(!game.is_flag_captured(Player::Black), "黑方军旗应该还在棋盘上");
    }

    #[test]
    fn test_is_game_over_initial_state() {
        let game = JunqiEngine::new_game();
        
        // 初始状态下游戏未结束
        assert!(!game.is_game_over(), "初始状态下游戏应该未结束");
    }

    #[test]
    fn test_is_game_over_after_flag_captured() {
        let mut game = JunqiEngine::new_game();
        
        // 移除红方军旗
        game.board_state.pieces.remove(&Position::new(11, 2));
        
        // 游戏应该结束
        assert!(game.is_game_over(), "军旗被夺取后游戏应该结束");
    }

    #[test]
    fn test_get_winner_no_winner() {
        let game = JunqiEngine::new_game();
        
        // 初始状态下没有获胜方
        assert_eq!(game.get_winner(), None, "初始状态下应该没有获胜方");
    }

    #[test]
    fn test_get_winner_red_flag_captured() {
        let mut game = JunqiEngine::new_game();
        
        // 移除红方军旗（黑方获胜）
        game.board_state.pieces.remove(&Position::new(11, 2));
        
        // 黑方应该获胜
        assert_eq!(game.get_winner(), Some(Player::Black), "红方军旗被夺取，黑方应该获胜");
    }

    #[test]
    fn test_get_winner_black_flag_captured() {
        let mut game = JunqiEngine::new_game();
        
        // 移除黑方军旗（红方获胜）
        game.board_state.pieces.remove(&Position::new(0, 1));
        
        // 红方应该获胜
        assert_eq!(game.get_winner(), Some(Player::Red), "黑方军旗被夺取，红方应该获胜");
    }

    #[test]
    fn test_game_ends_when_flag_captured_in_battle() {
        let mut game = JunqiEngine::new_game();
        
        // 设置场景：红方工兵攻击黑方军旗
        // 将红方工兵移到军旗旁边
        game.board_state.pieces.remove(&Position::new(9, 0)); // 移除原位置的工兵
        game.board_state.pieces.insert(
            Position::new(0, 0),
            Piece::new(PieceType::Junqi(JunqiPiece::Engineer), Player::Red)
        );
        
        // 工兵攻击军旗
        let from = Position::new(0, 0);
        let to = Position::new(0, 1); // 黑方军旗位置
        
        let result = game.make_move(from, to);
        assert!(result.is_ok(), "攻击军旗应该成功");
        
        // 游戏应该结束，红方获胜
        assert!(game.is_game_over(), "夺取军旗后游戏应该结束");
        assert_eq!(game.get_winner(), Some(Player::Red), "红方应该获胜");
        assert_eq!(game.game_status, GameStatus::Victory { winner: Player::Red }, "游戏状态应该是红方获胜");
    }

    #[test]
    fn test_undo_move_simple_move() {
        let mut game = JunqiEngine::new_game();
        
        // 清空一些位置以便测试移动
        game.board_state.pieces.remove(&Position::new(6, 0));
        
        // 保存初始状态
        let initial_pieces_count = game.board_state.pieces.len();
        let initial_player = game.board_state.current_player;
        
        // 移动红方连长从 (7, 0) 到 (6, 0)
        let from = Position::new(7, 0);
        let to = Position::new(6, 0);
        let piece_at_from = game.board_state.pieces.get(&from).unwrap().clone();
        
        let result = game.make_move(from, to);
        assert!(result.is_ok(), "移动应该成功");
        
        // 悔棋
        let undo_result = game.undo_move();
        assert!(undo_result.is_ok(), "悔棋应该成功");
        
        // 验证状态恢复
        assert_eq!(game.board_state.pieces.len(), initial_pieces_count, "棋子数量应该恢复");
        assert_eq!(game.board_state.current_player, initial_player, "当前玩家应该恢复");
        assert!(game.board_state.pieces.get(&from).is_some(), "起始位置应该有棋子");
        assert_eq!(game.board_state.pieces.get(&from).unwrap(), &piece_at_from, "棋子应该恢复到原位置");
        assert!(game.board_state.pieces.get(&to).is_none(), "目标位置应该为空");
    }

    #[test]
    fn test_undo_move_with_capture() {
        let mut game = JunqiEngine::new_game();
        
        // 设置一个简单的战斗场景
        // 将红方司令移到 (6, 0)，黑方排长在 (5, 0)
        game.board_state.pieces.remove(&Position::new(10, 4)); // 移除原位置的红方司令
        game.board_state.pieces.insert(
            Position::new(6, 0),
            Piece::new(PieceType::Junqi(JunqiPiece::Commander), Player::Red)
        );
        game.board_state.pieces.remove(&Position::new(2, 2)); // 移除原位置的黑方排长
        let defender = Piece::new(PieceType::Junqi(JunqiPiece::Platoon), Player::Black);
        game.board_state.pieces.insert(Position::new(5, 0), defender.clone());
        
        // 保存初始状态
        let initial_pieces_count = game.board_state.pieces.len();
        
        // 红方司令攻击黑方排长
        let from = Position::new(6, 0);
        let to = Position::new(5, 0);
        
        let result = game.make_move(from, to);
        assert!(result.is_ok(), "战斗移动应该成功");
        
        // 悔棋
        let undo_result = game.undo_move();
        assert!(undo_result.is_ok(), "悔棋应该成功");
        
        // 验证状态恢复
        assert_eq!(game.board_state.pieces.len(), initial_pieces_count, "棋子数量应该恢复");
        assert_eq!(game.board_state.current_player, Player::Red, "当前玩家应该恢复为红方");
        
        // 验证司令回到原位置
        let piece_at_from = game.board_state.pieces.get(&from);
        assert!(piece_at_from.is_some(), "起始位置应该有棋子");
        assert_eq!(piece_at_from.unwrap().piece_type, PieceType::Junqi(JunqiPiece::Commander), "应该是司令");
        
        // 验证排长恢复到目标位置
        let piece_at_to = game.board_state.pieces.get(&to);
        assert!(piece_at_to.is_some(), "目标位置应该有棋子");
        assert_eq!(piece_at_to.unwrap().piece_type, PieceType::Junqi(JunqiPiece::Platoon), "应该是排长");
        assert_eq!(piece_at_to.unwrap().player, Player::Black, "应该是黑方");
    }

    #[test]
    fn test_undo_move_both_die() {
        let mut game = JunqiEngine::new_game();
        
        // 设置同等级战斗场景
        // 将红方连长移到 (6, 0)，黑方连长在 (5, 0)
        game.board_state.pieces.remove(&Position::new(7, 0)); // 移除原位置的红方连长
        let attacker = Piece::new(PieceType::Junqi(JunqiPiece::Company), Player::Red);
        game.board_state.pieces.insert(Position::new(6, 0), attacker.clone());
        
        game.board_state.pieces.remove(&Position::new(2, 1)); // 移除原位置的黑方连长
        let defender = Piece::new(PieceType::Junqi(JunqiPiece::Company), Player::Black);
        game.board_state.pieces.insert(Position::new(5, 0), defender.clone());
        
        // 保存初始状态
        let initial_pieces_count = game.board_state.pieces.len();
        
        // 红方连长攻击黑方连长
        let from = Position::new(6, 0);
        let to = Position::new(5, 0);
        
        let result = game.make_move(from, to);
        assert!(result.is_ok(), "战斗移动应该成功");
        
        // 悔棋
        let undo_result = game.undo_move();
        assert!(undo_result.is_ok(), "悔棋应该成功");
        
        // 验证状态恢复
        assert_eq!(game.board_state.pieces.len(), initial_pieces_count, "棋子数量应该恢复");
        assert_eq!(game.board_state.current_player, Player::Red, "当前玩家应该恢复为红方");
        
        // 验证双方棋子都恢复
        let piece_at_from = game.board_state.pieces.get(&from);
        assert!(piece_at_from.is_some(), "起始位置应该有棋子");
        assert_eq!(piece_at_from.unwrap().piece_type, PieceType::Junqi(JunqiPiece::Company), "应该是连长");
        assert_eq!(piece_at_from.unwrap().player, Player::Red, "应该是红方");
        
        let piece_at_to = game.board_state.pieces.get(&to);
        assert!(piece_at_to.is_some(), "目标位置应该有棋子");
        assert_eq!(piece_at_to.unwrap().piece_type, PieceType::Junqi(JunqiPiece::Company), "应该是连长");
        assert_eq!(piece_at_to.unwrap().player, Player::Black, "应该是黑方");
    }

    #[test]
    fn test_undo_move_empty_history() {
        let mut game = JunqiEngine::new_game();
        
        // 尝试在没有移动历史的情况下悔棋
        let result = game.undo_move();
        assert!(result.is_err(), "没有移动历史时悔棋应该失败");
        
        if let Err(GameError::InvalidState { message }) = result {
            assert!(message.contains("没有可以悔棋的移动"), "错误消息应该说明没有可悔棋的移动");
        } else {
            panic!("应该返回 InvalidState 错误");
        }
    }

    #[test]
    fn test_undo_move_multiple_times() {
        let mut game = JunqiEngine::new_game();
        
        // 清空一些位置
        game.board_state.pieces.remove(&Position::new(6, 0));
        game.board_state.pieces.remove(&Position::new(5, 0));
        
        // 执行第一次移动
        let move1_from = Position::new(7, 0);
        let move1_to = Position::new(6, 0);
        game.make_move(move1_from, move1_to).unwrap();
        
        // 执行第二次移动（黑方）
        let move2_from = Position::new(4, 0);
        let move2_to = Position::new(5, 0);
        game.make_move(move2_from, move2_to).unwrap();
        
        // 悔棋第二次移动
        game.undo_move().unwrap();
        assert_eq!(game.board_state.current_player, Player::Black, "应该是黑方回合");
        assert!(game.board_state.pieces.get(&move2_from).is_some(), "黑方棋子应该回到原位置");
        assert!(game.board_state.pieces.get(&move2_to).is_none(), "目标位置应该为空");
        
        // 悔棋第一次移动
        game.undo_move().unwrap();
        assert_eq!(game.board_state.current_player, Player::Red, "应该是红方回合");
        assert!(game.board_state.pieces.get(&move1_from).is_some(), "红方棋子应该回到原位置");
        assert!(game.board_state.pieces.get(&move1_to).is_none(), "目标位置应该为空");
    }

    #[test]
    fn test_move_history_recorded() {
        let mut game = JunqiEngine::new_game();
        
        // 清空一些位置
        game.board_state.pieces.remove(&Position::new(6, 0));
        
        // 执行移动
        let from = Position::new(7, 0);
        let to = Position::new(6, 0);
        let piece = game.board_state.pieces.get(&from).unwrap().clone();
        
        game.make_move(from, to).unwrap();
        
        // 验证移动历史
        assert_eq!(game.board_state.move_history.len(), 1, "应该有一条移动历史");
        
        let last_move = &game.board_state.move_history[0];
        assert_eq!(last_move.from, from, "移动历史应该记录起始位置");
        assert_eq!(last_move.to, to, "移动历史应该记录目标位置");
        assert_eq!(last_move.piece, piece, "移动历史应该记录移动的棋子");
        assert!(last_move.captured_piece.is_none(), "移动历史应该记录没有被吃掉的棋子");
    }
}


// 属性测试已移动到 tests/ 目录
// #[cfg(test)]
// #[path = "junqi_engine_property_tests.rs"]
// mod junqi_engine_property_tests;

