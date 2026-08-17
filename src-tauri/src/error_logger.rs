// 错误日志记录模块
// 负责记录所有错误和异常到日志文件

use std::fs::{OpenOptions, create_dir_all};
use std::io::Write;
use std::path::PathBuf;
use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};

use crate::game_engine::GameError;
use crate::models::BoardState;

/// 日志条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    /// 时间戳
    pub timestamp: String,
    /// 错误码
    pub error_code: String,
    /// 错误消息
    pub message: String,
    /// 游戏状态快照（可选）
    pub game_state: Option<BoardState>,
}

/// 错误日志记录器
pub struct ErrorLogger {
    log_file_path: PathBuf,
}

impl ErrorLogger {
    /// 创建新的错误日志记录器
    pub fn new() -> Result<Self, String> {
        // 获取日志文件路径
        let log_dir = Self::get_log_directory()?;
        
        // 确保日志目录存在
        create_dir_all(&log_dir)
            .map_err(|e| format!("无法创建日志目录: {}", e))?;
        
        let log_file_path = log_dir.join("game_errors.log");
        
        Ok(ErrorLogger { log_file_path })
    }
    
    /// 获取日志目录路径
    fn get_log_directory() -> Result<PathBuf, String> {
        // 使用应用程序数据目录
        let app_data_dir = dirs::data_local_dir()
            .ok_or_else(|| "无法获取应用程序数据目录".to_string())?;
        
        Ok(app_data_dir.join("chess-game-app").join("logs"))
    }
    
    /// 记录错误
    pub fn log_error(&self, error: &GameError, game_state: Option<&BoardState>) -> Result<(), String> {
        let log_entry = LogEntry {
            timestamp: Self::get_timestamp(),
            error_code: error.error_code().to_string(),
            message: error.user_message(),
            game_state: game_state.cloned(),
        };
        
        self.write_log_entry(&log_entry)
    }
    
    /// 获取当前时间戳
    fn get_timestamp() -> String {
        let now: DateTime<Local> = Local::now();
        now.format("%Y-%m-%d %H:%M:%S%.3f").to_string()
    }
    
    /// 写入日志条目到文件
    fn write_log_entry(&self, entry: &LogEntry) -> Result<(), String> {
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file_path)
            .map_err(|e| format!("无法打开日志文件: {}", e))?;
        
        // 格式化日志条目
        let log_line = format!(
            "[{}] {} - {}\n",
            entry.timestamp,
            entry.error_code,
            entry.message
        );
        
        // 写入日志
        file.write_all(log_line.as_bytes())
            .map_err(|e| format!("无法写入日志文件: {}", e))?;
        
        // 如果有游戏状态，写入详细信息
        if let Some(game_state) = &entry.game_state {
            let state_json = serde_json::to_string_pretty(game_state)
                .map_err(|e| format!("无法序列化游戏状态: {}", e))?;
            
            let state_line = format!("  游戏状态: {}\n", state_json);
            file.write_all(state_line.as_bytes())
                .map_err(|e| format!("无法写入游戏状态: {}", e))?;
        }
        
        Ok(())
    }
    
    /// 获取日志文件路径
    #[allow(dead_code)]
    pub fn get_log_file_path(&self) -> &PathBuf {
        &self.log_file_path
    }
}

impl Default for ErrorLogger {
    fn default() -> Self {
        Self::new().expect("无法创建错误日志记录器")
    }
}

// 全局错误日志记录器实例
lazy_static::lazy_static! {
    pub static ref ERROR_LOGGER: ErrorLogger = ErrorLogger::new()
        .expect("无法初始化全局错误日志记录器");
}

/// 记录错误到日志文件
pub fn log_error(error: &GameError, game_state: Option<&BoardState>) {
    if let Err(e) = ERROR_LOGGER.log_error(error, game_state) {
        eprintln!("日志记录失败: {}", e);
    }
}

// 属性测试已移动到 tests/ 目录
// #[cfg(test)]
// #[path = "error_logger_property_tests.rs"]
// mod error_logger_property_tests;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Position, Player};

    #[test]
    fn test_error_logger_creation() {
        let logger = ErrorLogger::new();
        assert!(logger.is_ok());
    }

    #[test]
    fn test_log_illegal_move_error() {
        let logger = ErrorLogger::new().unwrap();
        
        let error = GameError::IllegalMove {
            from: Position { row: 0, col: 0 },
            to: Position { row: 1, col: 1 },
            reason: "测试错误".to_string(),
        };
        
        let result = logger.log_error(&error, None);
        assert!(result.is_ok());
        
        // 验证日志文件存在
        assert!(logger.get_log_file_path().exists());
    }

    #[test]
    fn test_log_error_with_game_state() {
        let logger = ErrorLogger::new().unwrap();
        
        let error = GameError::InvalidState {
            message: "测试状态错误".to_string(),
        };
        
        let board_state = BoardState {
            pieces: std::collections::HashMap::new(),
            current_player: Player::Red,
            move_history: vec![],
        };
        
        let result = logger.log_error(&error, Some(&board_state));
        assert!(result.is_ok());
    }

    #[test]
    fn test_error_code() {
        let error1 = GameError::IllegalMove {
            from: Position { row: 0, col: 0 },
            to: Position { row: 1, col: 1 },
            reason: "测试".to_string(),
        };
        assert_eq!(error1.error_code(), "ILLEGAL_MOVE");
        
        let error2 = GameError::InvalidInput {
            message: "测试".to_string(),
        };
        assert_eq!(error2.error_code(), "INVALID_INPUT");
        
        let error3 = GameError::InvalidState {
            message: "测试".to_string(),
        };
        assert_eq!(error3.error_code(), "INVALID_STATE");
        
        let error4 = GameError::AIError {
            message: "测试".to_string(),
        };
        assert_eq!(error4.error_code(), "AI_ERROR");
        
        let error5 = GameError::IPCError {
            message: "测试".to_string(),
        };
        assert_eq!(error5.error_code(), "IPC_ERROR");
    }

    #[test]
    fn test_user_message() {
        let error = GameError::IllegalMove {
            from: Position { row: 0, col: 0 },
            to: Position { row: 1, col: 1 },
            reason: "棋子不能这样移动".to_string(),
        };
        
        let message = error.user_message();
        assert!(message.contains("非法移动"));
        assert!(message.contains("0"));
        assert!(message.contains("1"));
        assert!(message.contains("棋子不能这样移动"));
    }

    #[test]
    fn test_global_log_error() {
        let error = GameError::AIError {
            message: "AI 计算超时".to_string(),
        };
        
        // 测试全局日志函数
        log_error(&error, None);
        
        // 验证日志文件存在
        assert!(ERROR_LOGGER.get_log_file_path().exists());
    }

    #[test]
    fn test_timestamp_format() {
        let timestamp = ErrorLogger::get_timestamp();
        
        // 验证时间戳格式（应该包含日期和时间）
        assert!(timestamp.contains("-"));
        assert!(timestamp.contains(":"));
    }
}
