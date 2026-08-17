// 游戏会话管理模块
// 负责管理活动的游戏会话，包括创建、存储和检索游戏实例

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::game_engine::{GameEngine, XiangqiEngine, JunqiEngine};

/// 游戏类型枚举
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameType {
    Xiangqi,
    Junqi,
}

/// 游戏会话，包装不同类型的游戏引擎
pub enum GameSession {
    Xiangqi(XiangqiEngine),
    Junqi(JunqiEngine),
}

impl GameSession {
    /// 创建新的游戏会话
    pub fn new(game_type: GameType) -> Self {
        match game_type {
            GameType::Xiangqi => GameSession::Xiangqi(XiangqiEngine::new_game()),
            GameType::Junqi => GameSession::Junqi(JunqiEngine::new_game()),
        }
    }

    /// 获取游戏类型
    #[allow(dead_code)]
    pub fn game_type(&self) -> GameType {
        match self {
            GameSession::Xiangqi(_) => GameType::Xiangqi,
            GameSession::Junqi(_) => GameType::Junqi,
        }
    }

    /// 获取象棋引擎的可变引用（如果是象棋游戏）
    #[allow(dead_code)]
    pub fn as_xiangqi_mut(&mut self) -> Option<&mut XiangqiEngine> {
        match self {
            GameSession::Xiangqi(engine) => Some(engine),
            _ => None,
        }
    }

    /// 获取象棋引擎的引用（如果是象棋游戏）
    #[allow(dead_code)]
    pub fn as_xiangqi(&self) -> Option<&XiangqiEngine> {
        match self {
            GameSession::Xiangqi(engine) => Some(engine),
            _ => None,
        }
    }

    /// 获取军棋引擎的可变引用（如果是军棋游戏）
    #[allow(dead_code)]
    pub fn as_junqi_mut(&mut self) -> Option<&mut JunqiEngine> {
        match self {
            GameSession::Junqi(engine) => Some(engine),
            _ => None,
        }
    }

    /// 获取军棋引擎的引用（如果是军棋游戏）
    #[allow(dead_code)]
    pub fn as_junqi(&self) -> Option<&JunqiEngine> {
        match self {
            GameSession::Junqi(engine) => Some(engine),
            _ => None,
        }
    }
}

/// 游戏会话管理器
/// 使用线程安全的方式管理所有活动的游戏会话
pub struct GameSessionManager {
    sessions: Arc<Mutex<HashMap<String, GameSession>>>,
}

impl GameSessionManager {
    /// 创建新的游戏会话管理器
    pub fn new() -> Self {
        GameSessionManager {
            sessions: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 生成唯一的游戏 ID
    fn generate_game_id() -> String {
        Uuid::new_v4().to_string()
    }

    /// 创建新的游戏会话并返回游戏 ID
    pub fn create_session(&self, game_type: GameType) -> Result<String, String> {
        let game_id = Self::generate_game_id();
        let session = GameSession::new(game_type);

        let mut sessions = self.sessions.lock()
            .map_err(|e| format!("无法获取会话锁: {}", e))?;

        sessions.insert(game_id.clone(), session);
        Ok(game_id)
    }

    /// 获取游戏会话的引用
    /// 获取游戏会话的引用
    #[allow(dead_code)]
    pub fn get_session(&self, game_id: &str) -> Result<Arc<Mutex<HashMap<String, GameSession>>>, String> {
        let sessions = self.sessions.lock()
            .map_err(|e| format!("无法获取会话锁: {}", e))?;

        if sessions.contains_key(game_id) {
            Ok(Arc::clone(&self.sessions))
        } else {
            Err(format!("游戏会话不存在: {}", game_id))
        }
    }

    /// 执行对游戏会话的操作
    pub fn with_session<F, R>(&self, game_id: &str, f: F) -> Result<R, String>
    where
        F: FnOnce(&GameSession) -> Result<R, String>,
    {
        let sessions = self.sessions.lock()
            .map_err(|e| format!("无法获取会话锁: {}", e))?;

        let session = sessions.get(game_id)
            .ok_or_else(|| format!("游戏会话不存在: {}", game_id))?;

        f(session)
    }

    /// 执行对游戏会话的可变操作
    pub fn with_session_mut<F, R>(&self, game_id: &str, f: F) -> Result<R, String>
    where
        F: FnOnce(&mut GameSession) -> Result<R, String>,
    {
        let mut sessions = self.sessions.lock()
            .map_err(|e| format!("无法获取会话锁: {}", e))?;

        let session = sessions.get_mut(game_id)
            .ok_or_else(|| format!("游戏会话不存在: {}", game_id))?;

        f(session)
    }

    /// 删除游戏会话
    #[allow(dead_code)]
    pub fn remove_session(&self, game_id: &str) -> Result<(), String> {
        let mut sessions = self.sessions.lock()
            .map_err(|e| format!("无法获取会话锁: {}", e))?;

        sessions.remove(game_id)
            .ok_or_else(|| format!("游戏会话不存在: {}", game_id))?;

        Ok(())
    }

    /// 获取所有活动会话的数量
    #[allow(dead_code)]
    pub fn session_count(&self) -> Result<usize, String> {
        let sessions = self.sessions.lock()
            .map_err(|e| format!("无法获取会话锁: {}", e))?;

        Ok(sessions.len())
    }

    /// 清除所有会话
    #[allow(dead_code)]
    pub fn clear_all_sessions(&self) -> Result<(), String> {
        let mut sessions = self.sessions.lock()
            .map_err(|e| format!("无法获取会话锁: {}", e))?;

        sessions.clear();
        Ok(())
    }
}

impl Default for GameSessionManager {
    fn default() -> Self {
        Self::new()
    }
}

// 全局游戏会话管理器实例
lazy_static::lazy_static! {
    pub static ref GAME_SESSION_MANAGER: GameSessionManager = GameSessionManager::new();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_xiangqi_session() {
        let manager = GameSessionManager::new();
        let game_id = manager.create_session(GameType::Xiangqi).unwrap();
        
        assert!(!game_id.is_empty());
        
        // 验证会话存在
        let result = manager.with_session(&game_id, |session| {
            assert_eq!(session.game_type(), GameType::Xiangqi);
            Ok(())
        });
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_junqi_session() {
        let manager = GameSessionManager::new();
        let game_id = manager.create_session(GameType::Junqi).unwrap();
        
        assert!(!game_id.is_empty());
        
        // 验证会话存在
        let result = manager.with_session(&game_id, |session| {
            assert_eq!(session.game_type(), GameType::Junqi);
            Ok(())
        });
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_unique_game_ids() {
        let manager = GameSessionManager::new();
        let game_id1 = manager.create_session(GameType::Xiangqi).unwrap();
        let game_id2 = manager.create_session(GameType::Xiangqi).unwrap();
        
        assert_ne!(game_id1, game_id2);
    }

    #[test]
    fn test_get_nonexistent_session() {
        let manager = GameSessionManager::new();
        let result = manager.with_session("nonexistent-id", |_| Ok(()));
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("游戏会话不存在"));
    }

    #[test]
    fn test_remove_session() {
        let manager = GameSessionManager::new();
        let game_id = manager.create_session(GameType::Xiangqi).unwrap();
        
        // 验证会话存在
        assert!(manager.with_session(&game_id, |_| Ok(())).is_ok());
        
        // 删除会话
        assert!(manager.remove_session(&game_id).is_ok());
        
        // 验证会话已被删除
        assert!(manager.with_session(&game_id, |_| Ok(())).is_err());
    }

    #[test]
    fn test_session_count() {
        let manager = GameSessionManager::new();
        
        assert_eq!(manager.session_count().unwrap(), 0);
        
        let _game_id1 = manager.create_session(GameType::Xiangqi).unwrap();
        assert_eq!(manager.session_count().unwrap(), 1);
        
        let _game_id2 = manager.create_session(GameType::Junqi).unwrap();
        assert_eq!(manager.session_count().unwrap(), 2);
    }

    #[test]
    fn test_clear_all_sessions() {
        let manager = GameSessionManager::new();
        
        let _game_id1 = manager.create_session(GameType::Xiangqi).unwrap();
        let _game_id2 = manager.create_session(GameType::Junqi).unwrap();
        
        assert_eq!(manager.session_count().unwrap(), 2);
        
        manager.clear_all_sessions().unwrap();
        
        assert_eq!(manager.session_count().unwrap(), 0);
    }

    #[test]
    fn test_with_session_mut() {
        let manager = GameSessionManager::new();
        let game_id = manager.create_session(GameType::Xiangqi).unwrap();
        
        // 测试可变访问
        let result = manager.with_session_mut(&game_id, |session| {
            let engine = session.as_xiangqi_mut().unwrap();
            // 验证可以获取可变引用
            let _ = engine.get_board_state();
            Ok(())
        });
        
        assert!(result.is_ok());
    }
}
