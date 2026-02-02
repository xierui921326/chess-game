// Tauri 命令模块
// 包含所有前端可调用的 Rust 命令

use crate::models::*;
use crate::game_session::{GAME_SESSION_MANAGER, GameType};
use crate::game_engine::{GameEngine, GameError};
use crate::ai::{AIEngine, Difficulty};
use crate::error_logger::log_error;

/// 开始新游戏
/// 
/// # 参数
/// - `game_type`: 游戏类型（"xiangqi"/"象棋" 或 "junqi"/"军棋"）
/// 
/// # 返回
/// - 成功：返回游戏 ID
/// - 失败：返回错误消息
#[tauri::command]
pub fn start_new_game(game_type: String) -> Result<String, String> {
    // 解析游戏类型
    let game_type_enum = match game_type.to_lowercase().as_str() {
        "xiangqi" | "象棋" => GameType::Xiangqi,
        "junqi" | "军棋" => GameType::Junqi,
        _ => {
            let error = GameError::InvalidInput {
                message: format!("不支持的游戏类型: {}", game_type),
            };
            log_error(&error, None);
            return Err(error.user_message());
        }
    };

    // 创建新的游戏会话
    GAME_SESSION_MANAGER.create_session(game_type_enum)
        .map_err(|e| {
            let error = GameError::IPCError { message: e };
            log_error(&error, None);
            error.user_message()
        })
}

/// 获取指定位置棋子的合法走法
/// 
/// # 参数
/// - `game_id`: 游戏 ID
/// - `position`: 棋子位置
/// 
/// # 返回
/// - 成功：返回合法走法位置列表
/// - 失败：返回错误消息
#[tauri::command]
pub fn get_legal_moves(game_id: String, position: Position) -> Result<Vec<Position>, String> {
    GAME_SESSION_MANAGER.with_session(&game_id, |session| {
        match session {
            crate::game_session::GameSession::Xiangqi(engine) => {
                Ok(engine.get_legal_moves(position))
            }
            crate::game_session::GameSession::Junqi(engine) => {
                Ok(engine.get_legal_moves(position))
            }
        }
    }).map_err(|e| {
        let error = GameError::IPCError { message: e };
        log_error(&error, None);
        error.user_message()
    })
}

/// 执行玩家走法
/// 
/// # 参数
/// - `game_id`: 游戏 ID
/// - `from`: 起始位置
/// - `to`: 目标位置
/// 
/// # 返回
/// - 成功：返回更新后的棋盘状态
/// - 失败：返回错误消息
#[tauri::command]
pub fn make_player_move(game_id: String, from: Position, to: Position) -> Result<BoardState, String> {
    GAME_SESSION_MANAGER.with_session_mut(&game_id, |session| {
        match session {
            crate::game_session::GameSession::Xiangqi(engine) => {
                let board_state = engine.get_board_state().clone();
                engine.make_move(from, to)
                    .map_err(|e| {
                        log_error(&e, Some(&board_state));
                        e.user_message()
                    })?;
                Ok(engine.get_board_state().clone())
            }
            crate::game_session::GameSession::Junqi(engine) => {
                let board_state = engine.get_board_state().clone();
                engine.make_move(from, to)
                    .map_err(|e| {
                        log_error(&e, Some(&board_state));
                        e.user_message()
                    })?;
                Ok(engine.get_board_state().clone())
            }
        }
    }).map_err(|e| {
        let error = GameError::IPCError { message: e };
        log_error(&error, None);
        error.user_message()
    })
}

/// 执行 AI 走法
/// 
/// # 参数
/// - `game_id`: 游戏 ID
/// 
/// # 返回
/// - 成功：返回 AI 的走法和更新后的棋盘状态
/// - 失败：返回错误消息
#[tauri::command]
pub fn make_ai_move(game_id: String) -> Result<(Position, Position, BoardState), String> {
    GAME_SESSION_MANAGER.with_session_mut(&game_id, |session| {
        match session {
            crate::game_session::GameSession::Xiangqi(engine) => {
                // 创建 AI 引擎（中等难度）
                let ai = AIEngine::new(Difficulty::Medium);
                
                // 计算最优走法
                let best_move = ai.calculate_best_move(engine)
                    .ok_or_else(|| {
                        let error = GameError::AIError {
                            message: "AI 无法找到合法走法".to_string(),
                        };
                        log_error(&error, Some(engine.get_board_state()));
                        error.user_message()
                    })?;
                
                // 执行走法
                let board_state = engine.get_board_state().clone();
                engine.make_move(best_move.from, best_move.to)
                    .map_err(|e| {
                        log_error(&e, Some(&board_state));
                        e.user_message()
                    })?;
                
                Ok((best_move.from, best_move.to, engine.get_board_state().clone()))
            }
            crate::game_session::GameSession::Junqi(engine) => {
                // 创建 AI 引擎（中等难度）
                let ai = AIEngine::new(Difficulty::Medium);
                
                // 计算最优走法
                let best_move = ai.calculate_best_move(engine)
                    .ok_or_else(|| {
                        let error = GameError::AIError {
                            message: "AI 无法找到合法走法".to_string(),
                        };
                        log_error(&error, Some(engine.get_board_state()));
                        error.user_message()
                    })?;
                
                // 执行走法
                let board_state = engine.get_board_state().clone();
                engine.make_move(best_move.from, best_move.to)
                    .map_err(|e| {
                        log_error(&e, Some(&board_state));
                        e.user_message()
                    })?;
                
                Ok((best_move.from, best_move.to, engine.get_board_state().clone()))
            }
        }
    }).map_err(|e| {
        let error = GameError::IPCError { message: e };
        log_error(&error, None);
        error.user_message()
    })
}

/// 悔棋
/// 
/// # 参数
/// - `game_id`: 游戏 ID
/// 
/// # 返回
/// - 成功：返回悔棋后的棋盘状态
/// - 失败：返回错误消息
#[tauri::command]
pub fn undo_move(game_id: String) -> Result<BoardState, String> {
    GAME_SESSION_MANAGER.with_session_mut(&game_id, |session| {
        match session {
            crate::game_session::GameSession::Xiangqi(engine) => {
                let board_state = engine.get_board_state().clone();
                engine.undo_move()
                    .map_err(|e| {
                        log_error(&e, Some(&board_state));
                        e.user_message()
                    })?;
                Ok(engine.get_board_state().clone())
            }
            crate::game_session::GameSession::Junqi(engine) => {
                let board_state = engine.get_board_state().clone();
                engine.undo_move()
                    .map_err(|e| {
                        log_error(&e, Some(&board_state));
                        e.user_message()
                    })?;
                Ok(engine.get_board_state().clone())
            }
        }
    }).map_err(|e| {
        let error = GameError::IPCError { message: e };
        log_error(&error, None);
        error.user_message()
    })
}

/// 重新开始游戏
/// 
/// # 参数
/// - `game_id`: 游戏 ID
/// 
/// # 返回
/// - 成功：返回新游戏的棋盘状态
/// - 失败：返回错误消息
#[tauri::command]
pub fn restart_game(game_id: String) -> Result<BoardState, String> {
    GAME_SESSION_MANAGER.with_session_mut(&game_id, |session| {
        match session {
            crate::game_session::GameSession::Xiangqi(engine) => {
                // 创建新的象棋引擎
                *engine = crate::game_engine::XiangqiEngine::new_game();
                Ok(engine.get_board_state().clone())
            }
            crate::game_session::GameSession::Junqi(engine) => {
                // 创建新的军棋引擎
                *engine = crate::game_engine::JunqiEngine::new_game();
                Ok(engine.get_board_state().clone())
            }
        }
    }).map_err(|e| {
        let error = GameError::IPCError { message: e };
        log_error(&error, None);
        error.user_message()
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_new_xiangqi_game() {
        let result = start_new_game("xiangqi".to_string());
        assert!(result.is_ok());
        
        let game_id = result.unwrap();
        assert!(!game_id.is_empty());
        
        // 验证游戏会话已创建
        let session_result = GAME_SESSION_MANAGER.with_session(&game_id, |session| {
            assert_eq!(session.game_type(), GameType::Xiangqi);
            Ok(())
        });
        assert!(session_result.is_ok());
    }

    #[test]
    fn test_start_new_junqi_game() {
        let result = start_new_game("junqi".to_string());
        assert!(result.is_ok());
        
        let game_id = result.unwrap();
        assert!(!game_id.is_empty());
        
        // 验证游戏会话已创建
        let session_result = GAME_SESSION_MANAGER.with_session(&game_id, |session| {
            assert_eq!(session.game_type(), GameType::Junqi);
            Ok(())
        });
        assert!(session_result.is_ok());
    }

    #[test]
    fn test_start_new_game_chinese_name() {
        // 测试中文游戏名称
        let result1 = start_new_game("象棋".to_string());
        assert!(result1.is_ok());
        
        let result2 = start_new_game("军棋".to_string());
        assert!(result2.is_ok());
    }

    #[test]
    fn test_start_new_game_case_insensitive() {
        // 测试大小写不敏感
        let result1 = start_new_game("XIANGQI".to_string());
        assert!(result1.is_ok());
        
        let result2 = start_new_game("Junqi".to_string());
        assert!(result2.is_ok());
    }

    #[test]
    fn test_start_new_game_invalid_type() {
        let result = start_new_game("invalid_game".to_string());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("不支持的游戏类型"));
    }

    #[test]
    fn test_multiple_game_sessions() {
        // 测试可以创建多个游戏会话
        let game_id1 = start_new_game("xiangqi".to_string()).unwrap();
        let game_id2 = start_new_game("junqi".to_string()).unwrap();
        let game_id3 = start_new_game("xiangqi".to_string()).unwrap();
        
        // 验证所有游戏 ID 都不同
        assert_ne!(game_id1, game_id2);
        assert_ne!(game_id1, game_id3);
        assert_ne!(game_id2, game_id3);
        
        // 验证所有会话都存在
        assert!(GAME_SESSION_MANAGER.with_session(&game_id1, |_| Ok(())).is_ok());
        assert!(GAME_SESSION_MANAGER.with_session(&game_id2, |_| Ok(())).is_ok());
        assert!(GAME_SESSION_MANAGER.with_session(&game_id3, |_| Ok(())).is_ok());
    }

    #[test]
    fn test_get_legal_moves_xiangqi() {
        // 创建象棋游戏
        let game_id = start_new_game("xiangqi".to_string()).unwrap();
        
        // 获取红方马的合法走法（位置 (9, 1)）
        let result = get_legal_moves(game_id.clone(), Position { row: 9, col: 1 });
        assert!(result.is_ok());
        
        let moves = result.unwrap();
        // 马在初始位置应该有 2 个合法走法
        assert_eq!(moves.len(), 2);
    }

    #[test]
    fn test_get_legal_moves_invalid_game_id() {
        let result = get_legal_moves("invalid-id".to_string(), Position { row: 0, col: 0 });
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("游戏会话不存在"));
    }

    #[test]
    fn test_make_player_move_xiangqi() {
        // 创建象棋游戏
        let game_id = start_new_game("xiangqi".to_string()).unwrap();
        
        // 移动红方马：(9, 1) -> (7, 2)
        let result = make_player_move(
            game_id.clone(),
            Position { row: 9, col: 1 },
            Position { row: 7, col: 2 }
        );
        
        assert!(result.is_ok());
        let board_state = result.unwrap();
        
        // 验证马已经移动到新位置
        assert!(board_state.pieces.contains_key(&Position { row: 7, col: 2 }));
        assert!(!board_state.pieces.contains_key(&Position { row: 9, col: 1 }));
        
        // 验证轮到黑方
        assert_eq!(board_state.current_player, Player::Black);
    }

    #[test]
    fn test_make_player_move_illegal() {
        // 创建象棋游戏
        let game_id = start_new_game("xiangqi".to_string()).unwrap();
        
        // 尝试非法移动：将马移动到非法位置
        let result = make_player_move(
            game_id.clone(),
            Position { row: 9, col: 1 },
            Position { row: 5, col: 5 }
        );
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("非法移动"));
    }

    #[test]
    fn test_undo_move_xiangqi() {
        // 创建象棋游戏
        let game_id = start_new_game("xiangqi".to_string()).unwrap();
        
        // 执行一个走法
        let _ = make_player_move(
            game_id.clone(),
            Position { row: 9, col: 1 },
            Position { row: 7, col: 2 }
        );
        
        // 悔棋
        let result = undo_move(game_id.clone());
        assert!(result.is_ok());
        
        let board_state = result.unwrap();
        
        // 验证马回到原位置
        assert!(board_state.pieces.contains_key(&Position { row: 9, col: 1 }));
        assert!(!board_state.pieces.contains_key(&Position { row: 7, col: 2 }));
        
        // 验证轮到红方
        assert_eq!(board_state.current_player, Player::Red);
    }

    #[test]
    fn test_undo_move_no_history() {
        // 创建象棋游戏
        let game_id = start_new_game("xiangqi".to_string()).unwrap();
        
        // 尝试在没有历史记录时悔棋
        let result = undo_move(game_id.clone());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("游戏状态错误"));
    }

    #[test]
    fn test_restart_game_xiangqi() {
        // 创建象棋游戏
        let game_id = start_new_game("xiangqi".to_string()).unwrap();
        
        // 执行一些走法
        let _ = make_player_move(
            game_id.clone(),
            Position { row: 9, col: 1 },
            Position { row: 7, col: 2 }
        );
        
        // 重新开始游戏
        let result = restart_game(game_id.clone());
        assert!(result.is_ok());
        
        let board_state = result.unwrap();
        
        // 验证棋盘恢复到初始状态
        assert!(board_state.pieces.contains_key(&Position { row: 9, col: 1 }));
        assert!(!board_state.pieces.contains_key(&Position { row: 7, col: 2 }));
        
        // 验证轮到红方
        assert_eq!(board_state.current_player, Player::Red);
        
        // 验证移动历史为空
        assert_eq!(board_state.move_history.len(), 0);
    }

    #[test]
    fn test_make_ai_move_xiangqi() {
        // 创建象棋游戏
        let game_id = start_new_game("xiangqi".to_string()).unwrap();
        
        // 先让玩家走一步（红方）
        let _ = make_player_move(
            game_id.clone(),
            Position { row: 9, col: 1 },
            Position { row: 7, col: 2 }
        );
        
        // 让 AI 走一步（黑方）
        let result = make_ai_move(game_id.clone());
        assert!(result.is_ok());
        
        let (from, to, board_state) = result.unwrap();
        
        // 验证 AI 返回了有效的走法
        assert_ne!(from, to);
        
        // 验证轮到红方
        assert_eq!(board_state.current_player, Player::Red);
        
        // 验证移动历史增加了
        assert_eq!(board_state.move_history.len(), 2);
    }

    #[test]
    fn test_get_legal_moves_junqi() {
        // 创建军棋游戏
        let game_id = start_new_game("junqi".to_string()).unwrap();
        
        // 获取红方某个棋子的合法走法（位置 (7, 0) 是红方连长）
        let result = get_legal_moves(game_id.clone(), Position { row: 7, col: 0 });
        assert!(result.is_ok());
        
        // 军棋初始位置的棋子应该有合法走法
        let moves = result.unwrap();
        assert!(!moves.is_empty());
    }

    #[test]
    fn test_make_player_move_junqi() {
        // 创建军棋游戏
        let game_id = start_new_game("junqi".to_string()).unwrap();
        
        // 获取合法走法（位置 (7, 0) 是红方连长）
        let legal_moves = get_legal_moves(game_id.clone(), Position { row: 7, col: 0 }).unwrap();
        
        if !legal_moves.is_empty() {
            // 执行第一个合法走法
            let result = make_player_move(
                game_id.clone(),
                Position { row: 7, col: 0 },
                legal_moves[0]
            );
            
            assert!(result.is_ok());
            let board_state = result.unwrap();
            
            // 验证轮到黑方
            assert_eq!(board_state.current_player, Player::Black);
        }
    }

    #[test]
    fn test_restart_game_junqi() {
        // 创建军棋游戏
        let game_id = start_new_game("junqi".to_string()).unwrap();
        
        // 获取初始棋子数量
        let initial_result = GAME_SESSION_MANAGER.with_session(&game_id, |session| {
            match session {
                crate::game_session::GameSession::Junqi(engine) => {
                    Ok(engine.get_board_state().pieces.len())
                }
                _ => Err("错误的游戏类型".to_string())
            }
        });
        let initial_count = initial_result.unwrap();
        
        // 重新开始游戏
        let result = restart_game(game_id.clone());
        assert!(result.is_ok());
        
        let board_state = result.unwrap();
        
        // 验证棋子数量相同
        assert_eq!(board_state.pieces.len(), initial_count);
        
        // 验证轮到红方
        assert_eq!(board_state.current_player, Player::Red);
        
        // 验证移动历史为空
        assert_eq!(board_state.move_history.len(), 0);
    }
}
