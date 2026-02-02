// AI 引擎模块
// 包含 Minimax 算法和评估函数

pub mod ai_engine;
pub mod difficulty;

#[cfg(test)]
mod ai_engine_tests;

// 重新导出
pub use ai_engine::{AIEngine, Move as AIMove};
pub use difficulty::Difficulty;
