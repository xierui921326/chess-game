import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import BoardRenderer from './BoardRenderer';
import type { BoardState, Position, GameStatus, MoveResult } from '../types';
import './GameBoard.css';

export type GameType = 'xiangqi' | 'junqi';

interface GameBoardProps {
  gameType: GameType;
  onBackToMenu: () => void;
}

interface GameState {
  game_id: string;
  board_state: BoardState;
  game_status: GameStatus;
}

const GameBoard: React.FC<GameBoardProps> = ({ gameType, onBackToMenu }) => {
  const [gameState, setGameState] = useState<GameState | null>(null);
  const [selectedPiece, setSelectedPiece] = useState<Position | null>(null);
  const [legalMoves, setLegalMoves] = useState<Position[]>([]);
  const [isPlayerTurn, setIsPlayerTurn] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  // 初始化游戏
  useEffect(() => {
    startNewGame();
  }, [gameType]);

  const startNewGame = async () => {
    try {
      setIsLoading(true);
      setError(null);
      const result = await invoke<GameState>('start_new_game', {
        gameType: gameType,
      });
      setGameState(result);
      setSelectedPiece(null);
      setLegalMoves([]);
      setIsPlayerTurn(true);
    } catch (err) {
      setError(`启动游戏失败: ${err}`);
      console.error('Failed to start game:', err);
    } finally {
      setIsLoading(false);
    }
  };

  // 处理棋盘点击
  const handleCellClick = async (position: Position) => {
    if (!gameState || !isPlayerTurn) return;

    const posKey = `${position.row},${position.col}`;
    const clickedPiece = gameState.board_state.pieces[posKey];

    // 如果点击的是自己的棋子，选中它
    if (clickedPiece && clickedPiece.player === gameState.board_state.current_player) {
      // 如果点击已选中的棋子，取消选择
      if (
        selectedPiece &&
        selectedPiece.row === position.row &&
        selectedPiece.col === position.col
      ) {
        setSelectedPiece(null);
        setLegalMoves([]);
      } else {
        // 选中新棋子
        setSelectedPiece(position);
        await fetchLegalMoves(position);
      }
    }
    // 如果已经选中了棋子，尝试移动到点击的位置
    else if (selectedPiece) {
      await attemptMove(selectedPiece, position);
    }
  };

  // 获取合法移动
  const fetchLegalMoves = async (position: Position) => {
    if (!gameState) return;

    try {
      const moves = await invoke<Position[]>('get_legal_moves', {
        gameId: gameState.game_id,
        position: position,
      });
      setLegalMoves(moves);
    } catch (err) {
      console.error('Failed to get legal moves:', err);
      setLegalMoves([]);
    }
  };

  // 尝试移动棋子
  const attemptMove = async (from: Position, to: Position) => {
    if (!gameState) return;

    try {
      setError(null);
      const result = await invoke<MoveResult>('make_player_move', {
        gameId: gameState.game_id,
        from: from,
        to: to,
      });

      if (result.success) {
        // 更新游戏状态
        setGameState({
          ...gameState,
          board_state: result.new_board_state,
          game_status: result.game_status,
        });

        // 清除选择
        setSelectedPiece(null);
        setLegalMoves([]);

        // 检查游戏是否结束
        if (isGameOver(result.game_status)) {
          setIsPlayerTurn(false);
          return;
        }

        // 轮到 AI
        setIsPlayerTurn(false);
        await makeAIMove();
      }
    } catch (err) {
      setError(`移动失败: ${err}`);
      console.error('Failed to make move:', err);
    }
  };

  // AI 移动
  const makeAIMove = async () => {
    if (!gameState) return;

    try {
      // 添加短暂延迟，让玩家看到自己的移动
      await new Promise((resolve) => setTimeout(resolve, 500));

      const result = await invoke<MoveResult>('make_ai_move', {
        gameId: gameState.game_id,
      });

      if (result.success) {
        setGameState({
          ...gameState,
          board_state: result.new_board_state,
          game_status: result.game_status,
        });

        // 检查游戏是否结束
        if (!isGameOver(result.game_status)) {
          setIsPlayerTurn(true);
        }
      }
    } catch (err) {
      setError(`AI 移动失败: ${err}`);
      console.error('Failed to make AI move:', err);
      setIsPlayerTurn(true);
    }
  };

  // 检查游戏是否结束
  const isGameOver = (status: GameStatus): boolean => {
    return (
      status.type === 'Checkmate' ||
      status.type === 'Stalemate' ||
      status.type === 'Victory'
    );
  };

  // 悔棋
  const handleUndo = async () => {
    if (!gameState) return;

    try {
      setError(null);
      const result = await invoke<GameState>('undo_move', {
        gameId: gameState.game_id,
      });
      setGameState(result);
      setSelectedPiece(null);
      setLegalMoves([]);
      setIsPlayerTurn(true);
    } catch (err) {
      setError(`悔棋失败: ${err}`);
      console.error('Failed to undo move:', err);
    }
  };

  // 重新开始
  const handleRestart = async () => {
    await startNewGame();
  };

  // 获取游戏状态文本
  const getGameStatusText = (): string => {
    if (!gameState) return '';

    const status = gameState.game_status;
    switch (status.type) {
      case 'Ongoing':
        return isPlayerTurn ? '轮到您' : 'AI 思考中...';
      case 'Check':
        return `${status.player === 'Red' ? '红方' : '黑方'}被将军！`;
      case 'Checkmate':
        return `${status.winner === 'Red' ? '红方' : '黑方'}获胜！（将死）`;
      case 'Stalemate':
        return '和棋（困毙）';
      case 'Victory':
        return `${status.winner === 'Red' ? '红方' : '黑方'}获胜！`;
      default:
        return '';
    }
  };

  // 渲染加载状态
  if (isLoading) {
    return (
      <div className="game-board-container">
        <div className="loading">加载中...</div>
      </div>
    );
  }

  // 渲染错误状态
  if (!gameState) {
    return (
      <div className="game-board-container">
        <div className="error">
          <p>无法加载游戏</p>
          <button onClick={onBackToMenu}>返回主菜单</button>
        </div>
      </div>
    );
  }

  const canUndo = gameState.board_state.move_history.length > 0;
  const gameOver = isGameOver(gameState.game_status);

  return (
    <div className="game-board-container">
      <div className="game-header">
        <h1>{gameType === 'xiangqi' ? '中国象棋' : '军棋'}</h1>
        <button className="back-button" onClick={onBackToMenu}>
          返回主菜单
        </button>
      </div>

      <div className="game-status">
        <p className={gameOver ? 'game-over' : ''}>{getGameStatusText()}</p>
        {error && <p className="error-message">{error}</p>}
      </div>

      <BoardRenderer
        boardState={gameState.board_state}
        selectedPiece={selectedPiece}
        legalMoves={legalMoves}
        onCellClick={handleCellClick}
        gameType={gameType}
      />

      <div className="game-controls">
        <button onClick={handleUndo} disabled={!canUndo || !isPlayerTurn || gameOver}>
          悔棋
        </button>
        <button onClick={handleRestart}>重新开始</button>
      </div>

      {gameOver && (
        <div className="game-over-overlay">
          <div className="game-over-dialog">
            <h2>游戏结束</h2>
            <p>{getGameStatusText()}</p>
            <div className="game-over-buttons">
              <button onClick={handleRestart}>再来一局</button>
              <button onClick={onBackToMenu}>返回主菜单</button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default GameBoard;
