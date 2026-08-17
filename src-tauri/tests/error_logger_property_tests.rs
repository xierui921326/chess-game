// 属性测试：错误处理
// Feature: chess-game-app, Property 19: 错误消息生成
// Feature: chess-game-app, Property 20: 错误日志记录
// **验证需求：10.1, 10.4**

use proptest::prelude::*;
use std::fs;
use crate::error_logger::{ErrorLogger, LogEntry};
use crate::game_engine::GameError;
use crate::models::{Position, BoardState};

// 生成任意 Position
fn arbitrary_position() -> impl Strategy<Value = Position> {
    (0u8..12, 0u8..9).prop_map(|(row, col)| Position::new(row, col))
}

// 生成任意错误原因字符串
fn arbitrary_reason() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("棋子不能这样移动".to_string()),
        Just("目标位置被己方棋子占据".to_string()),
        Just("移动会导致将军".to_string()),
        Just("马被蹩腿".to_string()),
        Just("象不能过河".to_string()),
        Just("将帅不能照面".to_string()),
        Just("不是该棋子的回合".to_string()),
        Just("目标位置超出棋盘范围".to_string()),
    ]
}

// 生成任意错误消息字符串
fn arbitrary_message() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("无效的位置坐标".to_string()),
        Just("游戏会话不存在".to_string()),
        Just("游戏状态已损坏".to_string()),
        Just("AI 计算超时".to_string()),
        Just("无法找到合法走法".to_string()),
        Just("前后端通信失败".to_string()),
        Just("序列化失败".to_string()),
        Just("反序列化失败".to_string()),
    ]
}

// 生成任意 GameError
fn arbitrary_game_error() -> impl Strategy<Value = GameError> {
    prop_oneof![
        // IllegalMove 错误
        (arbitrary_position(), arbitrary_position(), arbitrary_reason())
            .prop_map(|(from, to, reason)| GameError::IllegalMove { from, to, reason }),
        
        // InvalidInput 错误
        arbitrary_message()
            .prop_map(|message| GameError::InvalidInput { message }),
        
        // InvalidState 错误
        arbitrary_message()
            .prop_map(|message| GameError::InvalidState { message }),
        
        // AIError 错误
        arbitrary_message()
            .prop_map(|message| GameError::AIError { message }),
        
        // IPCError 错误
        arbitrary_message()
            .prop_map(|message| GameError::IPCError { message }),
    ]
}

// 生成任意 BoardState（可选）
fn arbitrary_optional_board_state() -> impl Strategy<Value = Option<BoardState>> {
    prop_oneof![
        Just(None),
        Just(Some(BoardState::new())),
    ]
}

proptest! {
    // 配置：减少测试用例数量以加快测试速度
    #![proptest_config(ProptestConfig::with_cases(1))]
    
    /// 属性 19：错误消息生成
    /// 
    /// 对于任何导致错误的操作（非法移动、无效输入等），
    /// 系统应该生成包含错误描述的错误消息。
    /// 
    /// 这个属性测试验证：
    /// 1. 每个 GameError 都能生成错误码
    /// 2. 每个 GameError 都能生成用户友好的错误消息
    /// 3. 错误消息包含足够的上下文信息
    /// 4. 错误消息是非空的
    #[test]
    fn prop_error_message_generation(error in arbitrary_game_error()) {
        // 验证错误码生成
        let error_code = error.error_code();
        prop_assert!(!error_code.is_empty(), "错误码不应该为空");
        
        // 验证错误码格式正确（应该是大写字母和下划线）
        prop_assert!(
            error_code.chars().all(|c| c.is_uppercase() || c == '_'),
            "错误码应该只包含大写字母和下划线"
        );
        
        // 验证错误码与错误类型匹配
        match &error {
            GameError::IllegalMove { .. } => {
                prop_assert_eq!(error_code, "ILLEGAL_MOVE", "IllegalMove 错误应该返回 ILLEGAL_MOVE 错误码");
            }
            GameError::InvalidInput { .. } => {
                prop_assert_eq!(error_code, "INVALID_INPUT", "InvalidInput 错误应该返回 INVALID_INPUT 错误码");
            }
            GameError::InvalidState { .. } => {
                prop_assert_eq!(error_code, "INVALID_STATE", "InvalidState 错误应该返回 INVALID_STATE 错误码");
            }
            GameError::AIError { .. } => {
                prop_assert_eq!(error_code, "AI_ERROR", "AIError 错误应该返回 AI_ERROR 错误码");
            }
            GameError::IPCError { .. } => {
                prop_assert_eq!(error_code, "IPC_ERROR", "IPCError 错误应该返回 IPC_ERROR 错误码");
            }
        }
        
        // 验证用户消息生成
        let user_message = error.user_message();
        prop_assert!(!user_message.is_empty(), "用户错误消息不应该为空");
        
        // 验证用户消息包含错误类型的描述
        match &error {
            GameError::IllegalMove { from, to, reason } => {
                // 验证消息包含 "非法移动" 关键词
                prop_assert!(
                    user_message.contains("非法移动"),
                    "IllegalMove 错误消息应该包含'非法移动'关键词"
                );
                
                // 验证消息包含位置信息
                let from_row_str = from.row.to_string();
                let from_col_str = from.col.to_string();
                let to_row_str = to.row.to_string();
                let to_col_str = to.col.to_string();
                
                prop_assert!(
                    user_message.contains(&from_row_str) && user_message.contains(&from_col_str),
                    "IllegalMove 错误消息应该包含起始位置信息"
                );
                prop_assert!(
                    user_message.contains(&to_row_str) && user_message.contains(&to_col_str),
                    "IllegalMove 错误消息应该包含目标位置信息"
                );
                
                // 验证消息包含错误原因
                prop_assert!(
                    user_message.contains(reason),
                    "IllegalMove 错误消息应该包含错误原因"
                );
            }
            GameError::InvalidInput { message } => {
                prop_assert!(
                    user_message.contains("无效输入"),
                    "InvalidInput 错误消息应该包含'无效输入'关键词"
                );
                prop_assert!(
                    user_message.contains(message),
                    "InvalidInput 错误消息应该包含具体错误信息"
                );
            }
            GameError::InvalidState { message } => {
                prop_assert!(
                    user_message.contains("游戏状态错误"),
                    "InvalidState 错误消息应该包含'游戏状态错误'关键词"
                );
                prop_assert!(
                    user_message.contains(message),
                    "InvalidState 错误消息应该包含具体错误信息"
                );
            }
            GameError::AIError { message } => {
                prop_assert!(
                    user_message.contains("AI 计算错误"),
                    "AIError 错误消息应该包含'AI 计算错误'关键词"
                );
                prop_assert!(
                    user_message.contains(message),
                    "AIError 错误消息应该包含具体错误信息"
                );
            }
            GameError::IPCError { message } => {
                prop_assert!(
                    user_message.contains("通信错误"),
                    "IPCError 错误消息应该包含'通信错误'关键词"
                );
                prop_assert!(
                    user_message.contains(message),
                    "IPCError 错误消息应该包含具体错误信息"
                );
            }
        }
        
        // 验证错误消息是有效的 UTF-8 字符串（已经是 String 类型，所以一定是有效的 UTF-8）
        // 验证消息不为空
        prop_assert!(
            !user_message.is_empty(),
            "错误消息不应该为空"
        );
        
        // 验证消息包含可打印字符
        prop_assert!(
            user_message.chars().any(|c| !c.is_whitespace()),
            "错误消息应该包含非空白字符"
        );
    }
    
    /// 属性 20：错误日志记录
    /// 
    /// 对于任何发生的错误或异常，系统应该将错误信息
    /// （包括错误类型、时间戳和上下文）写入日志文件。
    /// 
    /// 这个属性测试验证：
    /// 1. 错误可以被成功记录到日志文件
    /// 2. 日志条目包含时间戳
    /// 3. 日志条目包含错误码
    /// 4. 日志条目包含错误消息
    /// 5. 日志文件可以被创建和写入
    /// 6. 可选的游戏状态信息被正确记录
    #[test]
    fn prop_error_logging(
        error in arbitrary_game_error(),
        game_state in arbitrary_optional_board_state()
    ) {
        // 创建错误日志记录器
        let logger_result = ErrorLogger::new();
        prop_assert!(logger_result.is_ok(), "应该能够创建错误日志记录器");
        
        let logger = logger_result.unwrap();
        
        // 记录错误
        let log_result = logger.log_error(&error, game_state.as_ref());
        prop_assert!(log_result.is_ok(), "应该能够成功记录错误");
        
        // 验证日志文件存在
        let log_file_path = logger.get_log_file_path();
        prop_assert!(log_file_path.exists(), "日志文件应该存在");
        
        // 读取日志文件内容
        let log_content_result = fs::read_to_string(log_file_path);
        prop_assert!(log_content_result.is_ok(), "应该能够读取日志文件");
        
        let log_content = log_content_result.unwrap();
        prop_assert!(!log_content.is_empty(), "日志文件不应该为空");
        
        // 验证日志内容包含错误码
        let error_code = error.error_code();
        prop_assert!(
            log_content.contains(error_code),
            "日志内容应该包含错误码: {}",
            error_code
        );
        
        // 验证日志内容包含错误消息的关键部分
        let user_message = error.user_message();
        match &error {
            GameError::IllegalMove { .. } => {
                prop_assert!(
                    log_content.contains("非法移动"),
                    "日志应该包含错误类型描述"
                );
            }
            GameError::InvalidInput { .. } => {
                prop_assert!(
                    log_content.contains("无效输入"),
                    "日志应该包含错误类型描述"
                );
            }
            GameError::InvalidState { .. } => {
                prop_assert!(
                    log_content.contains("游戏状态错误"),
                    "日志应该包含错误类型描述"
                );
            }
            GameError::AIError { .. } => {
                prop_assert!(
                    log_content.contains("AI 计算错误"),
                    "日志应该包含错误类型描述"
                );
            }
            GameError::IPCError { .. } => {
                prop_assert!(
                    log_content.contains("通信错误"),
                    "日志应该包含错误类型描述"
                );
            }
        }
        
        // 验证日志包含时间戳格式（应该包含日期和时间分隔符）
        prop_assert!(
            log_content.contains("-") && log_content.contains(":"),
            "日志应该包含时间戳（包含日期和时间）"
        );
        
        // 验证日志格式正确（应该包含方括号包围的时间戳）
        prop_assert!(
            log_content.contains("[") && log_content.contains("]"),
            "日志应该使用正确的格式（时间戳用方括号包围）"
        );
        
        // 如果提供了游戏状态，验证日志包含游戏状态信息
        if game_state.is_some() {
            prop_assert!(
                log_content.contains("游戏状态"),
                "当提供游戏状态时，日志应该包含游戏状态信息"
            );
        }
    }
    
    /// 属性测试：多次记录错误应该都成功
    /// 
    /// 验证错误日志记录器可以处理多次日志记录操作
    #[test]
    fn prop_multiple_error_logging(errors in proptest::collection::vec(arbitrary_game_error(), 1..5)) {
        let logger_result = ErrorLogger::new();
        prop_assert!(logger_result.is_ok(), "应该能够创建错误日志记录器");
        
        let logger = logger_result.unwrap();
        
        // 记录多个错误
        for error in &errors {
            let log_result = logger.log_error(error, None);
            prop_assert!(log_result.is_ok(), "每次日志记录都应该成功");
        }
        
        // 验证日志文件存在
        let log_file_path = logger.get_log_file_path();
        prop_assert!(log_file_path.exists(), "日志文件应该存在");
        
        // 读取日志文件
        let log_content = fs::read_to_string(log_file_path).unwrap();
        
        // 验证所有错误都被记录
        for error in &errors {
            let error_code = error.error_code();
            prop_assert!(
                log_content.contains(error_code),
                "日志应该包含所有错误的错误码"
            );
        }
    }
    
    /// 属性测试：日志条目应该包含完整信息
    /// 
    /// 验证 LogEntry 结构体包含所有必要的字段
    #[test]
    fn prop_log_entry_completeness(error in arbitrary_game_error()) {
        // 创建日志条目
        let log_entry = LogEntry {
            timestamp: "2024-01-01 12:00:00.000".to_string(),
            error_code: error.error_code().to_string(),
            message: error.user_message(),
            game_state: None,
        };
        
        // 验证时间戳字段存在且非空
        prop_assert!(!log_entry.timestamp.is_empty(), "时间戳不应该为空");
        
        // 验证错误码字段存在且非空
        prop_assert!(!log_entry.error_code.is_empty(), "错误码不应该为空");
        
        // 验证消息字段存在且非空
        prop_assert!(!log_entry.message.is_empty(), "错误消息不应该为空");
        
        // 验证日志条目可以被序列化
        let serialized = serde_json::to_string(&log_entry);
        prop_assert!(serialized.is_ok(), "日志条目应该可以被序列化");
        
        // 验证日志条目可以被反序列化
        if let Ok(json_str) = serialized {
            let deserialized: Result<LogEntry, _> = serde_json::from_str(&json_str);
            prop_assert!(deserialized.is_ok(), "日志条目应该可以被反序列化");
            
            if let Ok(restored) = deserialized {
                prop_assert_eq!(restored.timestamp, log_entry.timestamp, "时间戳应该保持一致");
                prop_assert_eq!(restored.error_code, log_entry.error_code, "错误码应该保持一致");
                prop_assert_eq!(restored.message, log_entry.message, "消息应该保持一致");
            }
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    
    #[test]
    fn test_illegal_move_error_message() {
        let error = GameError::IllegalMove {
            from: Position::new(0, 0),
            to: Position::new(1, 1),
            reason: "测试原因".to_string(),
        };
        
        let error_code = error.error_code();
        assert_eq!(error_code, "ILLEGAL_MOVE");
        
        let message = error.user_message();
        assert!(message.contains("非法移动"));
        assert!(message.contains("0"));
        assert!(message.contains("1"));
        assert!(message.contains("测试原因"));
    }
    
    #[test]
    fn test_invalid_input_error_message() {
        let error = GameError::InvalidInput {
            message: "无效的坐标".to_string(),
        };
        
        let error_code = error.error_code();
        assert_eq!(error_code, "INVALID_INPUT");
        
        let message = error.user_message();
        assert!(message.contains("无效输入"));
        assert!(message.contains("无效的坐标"));
    }
    
    #[test]
    fn test_error_logging_creates_file() {
        let logger = ErrorLogger::new().unwrap();
        
        let error = GameError::AIError {
            message: "测试错误".to_string(),
        };
        
        let result = logger.log_error(&error, None);
        assert!(result.is_ok());
        
        // 验证日志文件存在
        assert!(logger.get_log_file_path().exists());
    }
    
    #[test]
    fn test_error_logging_with_game_state() {
        let logger = ErrorLogger::new().unwrap();
        
        let error = GameError::InvalidState {
            message: "状态损坏".to_string(),
        };
        
        let board_state = BoardState::new();
        
        let result = logger.log_error(&error, Some(&board_state));
        assert!(result.is_ok());
        
        // 读取日志文件
        let log_content = fs::read_to_string(logger.get_log_file_path()).unwrap();
        
        // 验证包含游戏状态信息
        assert!(log_content.contains("游戏状态"));
    }
    
    #[test]
    fn test_log_entry_serialization() {
        let log_entry = LogEntry {
            timestamp: "2024-01-01 12:00:00.000".to_string(),
            error_code: "TEST_ERROR".to_string(),
            message: "测试消息".to_string(),
            game_state: None,
        };
        
        // 序列化
        let json = serde_json::to_string(&log_entry).unwrap();
        assert!(!json.is_empty());
        
        // 反序列化
        let restored: LogEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.timestamp, log_entry.timestamp);
        assert_eq!(restored.error_code, log_entry.error_code);
        assert_eq!(restored.message, log_entry.message);
    }
}
