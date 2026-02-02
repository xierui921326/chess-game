// 游戏引擎模块
// 包含 GameEngine trait 和具体的游戏引擎实现

pub mod game_engine_trait;
pub mod xiangqi_engine;
pub mod junqi_engine;

// 测试模块
#[cfg(test)]
mod xiangqi_engine_property_tests;

// 重新导出
pub use game_engine_trait::{GameEngine, GameError, GameResult};
pub use xiangqi_engine::XiangqiEngine;
pub use junqi_engine::JunqiEngine;
