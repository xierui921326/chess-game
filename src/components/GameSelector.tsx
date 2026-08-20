import React from 'react';
import './GameSelector.css';

export type GameType = 'xiangqi' | 'junqi';

interface GameSelectorProps {
  onGameSelect: (gameType: GameType) => void;
}

const GameSelector: React.FC<GameSelectorProps> = ({ onGameSelect }) => {
  return (
    <div className="game-selector">
      <h1 className="game-selector-title">棋类游戏</h1>
      <p className="game-selector-subtitle">选择棋种，人机对战</p>

      <div className="game-options">
        <button
          className="game-option xiangqi"
          onClick={() => onGameSelect('xiangqi')}
          aria-label="选择中国象棋"
        >
          <div className="game-icon">♟</div>
          <h2>中国象棋</h2>
          <p>标准规则 · 人机对战</p>
        </button>

        <button
          className="game-option junqi"
          onClick={() => onGameSelect('junqi')}
          aria-label="选择军棋"
        >
          <div className="game-icon">⚔</div>
          <h2>军棋</h2>
          <p>二人翻棋 MVP · 暗棋对战</p>
        </button>
      </div>
    </div>
  );
};

export default GameSelector;
