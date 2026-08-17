// Difficulty 枚举 - AI 难度级别
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    /// 搜索深度：控制在较短时间内响应，避免阻塞 UI
    pub fn search_depth(&self) -> u8 {
        match self {
            Difficulty::Easy => 1,
            Difficulty::Medium => 2,
            Difficulty::Hard => 3,
        }
    }
}
