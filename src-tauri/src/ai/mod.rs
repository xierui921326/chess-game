// AI 引擎模块
// 包含 Minimax 算法和评估函数

pub mod ai_engine;
pub mod difficulty;

// 测试已移动到 tests/ 目录
// #[cfg(test)]
// mod ai_engine_tests;

// 重新导出
pub use ai_engine::AIEngine;
// pub use ai_engine::Move as AIMove; // 未使用，已注释
pub use difficulty::Difficulty;
