import React, { useCallback, useEffect, useRef, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import BoardRenderer from './BoardRenderer';
import type {
  BoardState,
  Difficulty,
  GameStatus,
  GameType,
  MoveResult,
  Player,
  Position,
} from '../types';
import { DIFFICULTY_LABELS, parseGameStatus } from '../types';
import './GameBoard.css';

interface GameBoardProps {
  gameType: GameType;
  playerSide: Player;
  difficulty: Difficulty;
  onBackToMenu: () => void;
  onBackToSetup?: () => void;
}

interface GameState {
  game_id: string;
  board_state: BoardState;
  game_status: GameStatus;
}

function normalizeGameState(raw: GameState): GameState {
  return {
    ...raw,
    game_status: parseGameStatus(raw.game_status),
  };
}

function normalizeMoveResult(raw: MoveResult): MoveResult {
  return {
    ...raw,
    game_status: parseGameStatus(raw.game_status),
  };
}

const GameBoard: React.FC<GameBoardProps> = ({
  gameType,
  playerSide,
  difficulty,
  onBackToMenu,
  onBackToSetup,
}) => {
  const [gameState, setGameState] = useState<GameState | null>(null);
  const [selectedPiece, setSelectedPiece] = useState<Position | null>(null);
  const [legalMoves, setLegalMoves] = useState<Position[]>([]);
  const [isPlayerTurn, setIsPlayerTurn] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [resigned, setResigned] = useState(false);
  const gameIdRef = useRef<string | null>(null);

  const isGameOver = useCallback((status: GameStatus): boolean => {
    return (
      status.type === 'Checkmate' ||
      status.type === 'Stalemate' ||
      status.type === 'Victory'
    );
  }, []);

  const makeAIMove = useCallback(
    async (current: GameState) => {
      try {
        await new Promise((resolve) => setTimeout(resolve, 400));

        const result = normalizeMoveResult(
          await invoke<MoveResult>('make_ai_move', {
            gameId: current.game_id,
          }),
        );

        if (result.success) {
          setGameState({
            game_id: current.game_id,
            board_state: result.new_board_state,
            game_status: result.game_status,
          });

          if (!isGameOver(result.game_status)) {
            setIsPlayerTurn(true);
          }
        }
      } catch (err) {
        setError(`AI 移动失败: ${err}`);
        console.error('Failed to make AI move:', err);
        setIsPlayerTurn(true);
      }
    },
    [isGameOver],
  );

  const startNewGame = useCallback(async () => {
    try {
      setIsLoading(true);
      setError(null);
      setResigned(false);
      setSelectedPiece(null);
      setLegalMoves([]);

      const result = normalizeGameState(
        await invoke<GameState>('start_new_game', {
          gameType,
          difficulty,
        }),
      );

      gameIdRef.current = result.game_id;
      setGameState(result);

      const humanToMove = result.board_state.current_player === playerSide;
      setIsPlayerTurn(humanToMove);

      if (!humanToMove && !isGameOver(result.game_status)) {
        setIsPlayerTurn(false);
        await makeAIMove(result);
      }
    } catch (err) {
      setError(`启动游戏失败: ${err}`);
      console.error('Failed to start game:', err);
      setGameState(null);
    } finally {
      setIsLoading(false);
    }
  }, [gameType, difficulty, playerSide, isGameOver, makeAIMove]);

  useEffect(() => {
    startNewGame();
  }, [startNewGame]);

  const fetchLegalMoves = async (position: Position, gameId: string) => {
    try {
      const moves = await invoke<Position[]>('get_legal_moves', {
        gameId,
        position,
      });
      setLegalMoves(moves);
    } catch (err) {
      console.error('Failed to get legal moves:', err);
      setLegalMoves([]);
    }
  };

  const attemptMove = async (from: Position, to: Position, current: GameState) => {
    try {
      setError(null);
      const result = normalizeMoveResult(
        await invoke<MoveResult>('make_player_move', {
          gameId: current.game_id,
          from,
          to,
        }),
      );

      if (result.success) {
        const next: GameState = {
          game_id: current.game_id,
          board_state: result.new_board_state,
          game_status: result.game_status,
        };
        setGameState(next);
        setSelectedPiece(null);
        setLegalMoves([]);

        if (isGameOver(result.game_status)) {
          setIsPlayerTurn(false);
          return;
        }

        setIsPlayerTurn(false);
        await makeAIMove(next);
      }
    } catch (err) {
      setError(`移动失败: ${err}`);
      console.error('Failed to make move:', err);
    }
  };

  const handleCellClick = async (position: Position) => {
    if (!gameState || !isPlayerTurn || resigned) return;

    // 仅允许操作己方棋子
    const posKey = `${position.row},${position.col}`;
    const clickedPiece = gameState.board_state.pieces[posKey];

    if (clickedPiece && clickedPiece.player === playerSide) {
      if (
        selectedPiece &&
        selectedPiece.row === position.row &&
        selectedPiece.col === position.col
      ) {
        setSelectedPiece(null);
        setLegalMoves([]);
      } else {
        setSelectedPiece(position);
        await fetchLegalMoves(position, gameState.game_id);
      }
    } else if (selectedPiece) {
      await attemptMove(selectedPiece, position, gameState);
    }
  };

  const handleUndo = async () => {
    if (!gameState) return;

    try {
      setError(null);
      const result = normalizeGameState(
        await invoke<GameState>('undo_move', {
          gameId: gameState.game_id,
        }),
      );
      setGameState(result);
      setSelectedPiece(null);
      setLegalMoves([]);
      setResigned(false);
      setIsPlayerTurn(result.board_state.current_player === playerSide);
    } catch (err) {
      setError(`悔棋失败: ${err}`);
      console.error('Failed to undo move:', err);
    }
  };

  const handleResign = () => {
    setResigned(true);
    setIsPlayerTurn(false);
    setSelectedPiece(null);
    setLegalMoves([]);
  };

  const getGameStatusText = (): string => {
    if (!gameState) return '';

    if (resigned) {
      const winner = playerSide === 'Red' ? '黑方' : '红方';
      return `您已认输，${winner}获胜`;
    }

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

  if (isLoading) {
    return (
      <div className="game-board-container">
        <div className="loading">加载中...</div>
      </div>
    );
  }

  if (!gameState) {
    return (
      <div className="game-board-container">
        <div className="error">
          <p>{error || '无法加载游戏'}</p>
          <button type="button" onClick={onBackToMenu}>
            返回主菜单
          </button>
          {onBackToSetup && (
            <button type="button" onClick={onBackToSetup}>
              返回设置
            </button>
          )}
        </div>
      </div>
    );
  }

  const canUndo =
    !!gameState.board_state.move_history &&
    gameState.board_state.move_history.length > 0;
  const gameOver = resigned || isGameOver(gameState.game_status);
  const gameTitle = gameType === 'xiangqi' ? '中国象棋' : '军棋（翻棋）';
  const statusText = getGameStatusText();

  return (
    <div className={`game-board-container layout-${gameType}`}>
      <div className="game-header">
        <div className="game-header-text">
          <h1>{gameTitle}</h1>
          <p className="game-meta">
            执{playerSide === 'Red' ? '红' : '黑'} · {DIFFICULTY_LABELS[difficulty]}
          </p>
        </div>
        <div className="header-actions">
          {onBackToSetup && (
            <button type="button" className="back-button secondary" onClick={onBackToSetup}>
              对局设置
            </button>
          )}
          <button type="button" className="back-button" onClick={onBackToMenu}>
            返回主菜单
          </button>
        </div>
      </div>

      {statusText ? (
        <div className="game-status">
          <p className={gameOver ? 'game-over' : ''}>{statusText}</p>
        </div>
      ) : null}

      {error ? (
        <div className="game-error-banner" role="alert">
          {error}
        </div>
      ) : null}

      <div className="board-area">
        <BoardRenderer
          boardState={gameState.board_state}
          selectedPiece={selectedPiece}
          legalMoves={legalMoves}
          onCellClick={handleCellClick}
          gameType={gameType}
        />
      </div>

      <div className="game-controls">
        <button
          type="button"
          onClick={handleUndo}
          disabled={!canUndo || !isPlayerTurn || gameOver}
        >
          悔棋
        </button>
        <button type="button" onClick={handleResign} disabled={gameOver || !isPlayerTurn}>
          认输
        </button>
        <button type="button" onClick={startNewGame}>
          重新开始
        </button>
      </div>

      {gameOver && (
        <div className="game-over-overlay">
          <div className="game-over-dialog">
            <h2>游戏结束</h2>
            <p>{statusText}</p>
            <div className="game-over-buttons">
              <button type="button" onClick={startNewGame}>
                再来一局
              </button>
              {onBackToSetup && (
                <button type="button" onClick={onBackToSetup}>
                  调整设置
                </button>
              )}
              <button type="button" onClick={onBackToMenu}>
                返回主菜单
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default GameBoard;
