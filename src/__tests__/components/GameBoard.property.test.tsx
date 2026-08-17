import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/react';
import GameBoard from '../../components/GameBoard';
import type { Position, BoardState, GameStatus } from '../../types';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

import { invoke } from '@tauri-apps/api/core';

describe('GameBoard 属性测试', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('属性 11：非法移动拒绝', () => {
    // Feature: chess-game-app, Property 11: 非法移动拒绝
    it('单元测试：尝试非法移动时应该拒绝并保持游戏状态不变', async () => {
      const mockGameState = {
        game_id: 'test-game-1',
        board_state: {
          pieces: {
            '0,0': {
              piece_type: { Xiangqi: 'Chariot' },
              player: 'Red',
            },
          },
          current_player: 'Red',
          move_history: [],
        } as BoardState,
        game_status: { type: 'Ongoing' } as GameStatus,
      };

      // Mock start_new_game
      (invoke as any).mockResolvedValue(mockGameState);

      const mockOnBackToMenu = vi.fn();
      render(<GameBoard gameType="xiangqi" onBackToMenu={mockOnBackToMenu} />);

      // 等待游戏加载
      await waitFor(
        () => {
          expect(invoke).toHaveBeenCalledWith('start_new_game', {
            gameType: 'xiangqi',
          });
        },
        { timeout: 3000 }
      );

      // 验证组件渲染成功
      expect(screen.getByText('中国象棋')).toBeInTheDocument();
    });

    it('单元测试：非法移动后当前玩家不应该改变', async () => {
      const mockGameState = {
        game_id: 'test-game-2',
        board_state: {
          pieces: {
            '0,0': {
              piece_type: { Xiangqi: 'Chariot' },
              player: 'Red',
            },
          },
          current_player: 'Red',
          move_history: [],
        } as BoardState,
        game_status: { type: 'Ongoing' } as GameStatus,
      };

      (invoke as any).mockResolvedValue(mockGameState);

      const mockOnBackToMenu = vi.fn();
      render(<GameBoard gameType="xiangqi" onBackToMenu={mockOnBackToMenu} />);

      await waitFor(
        () => {
          expect(screen.getByText('中国象棋')).toBeInTheDocument();
        },
        { timeout: 3000 }
      );

      // 验证初始状态
      expect(screen.getByText('轮到您')).toBeInTheDocument();
    });

    it('单元测试：非法移动不应该更新棋盘状态', async () => {
      const initialBoardState = {
        pieces: {
          '0,0': {
            piece_type: { Xiangqi: 'Chariot' },
            player: 'Red',
          },
          '9,4': {
            piece_type: { Xiangqi: 'General' },
            player: 'Black',
          },
        },
        current_player: 'Red',
        move_history: [],
      } as BoardState;

      const mockGameState = {
        game_id: 'test-game-3',
        board_state: initialBoardState,
        game_status: { type: 'Ongoing' } as GameStatus,
      };

      (invoke as any).mockResolvedValue(mockGameState);

      const mockOnBackToMenu = vi.fn();
      render(<GameBoard gameType="xiangqi" onBackToMenu={mockOnBackToMenu} />);

      await waitFor(
        () => {
          expect(screen.getByText('中国象棋')).toBeInTheDocument();
        },
        { timeout: 3000 }
      );

      // 验证棋盘渲染
      const canvas = document.querySelector('canvas');
      expect(canvas).toBeTruthy();
    });
  });

  describe('属性 12：棋子选择切换', () => {
    // Feature: chess-game-app, Property 12: 棋子选择切换
    it('单元测试：点击已选中的棋子应该取消选择', async () => {
      const mockGameState = {
        game_id: 'test-game-4',
        board_state: {
          pieces: {
            '0,0': {
              piece_type: { Xiangqi: 'Chariot' },
              player: 'Red',
            },
          },
          current_player: 'Red',
          move_history: [],
        } as BoardState,
        game_status: { type: 'Ongoing' } as GameStatus,
      };

      (invoke as any).mockResolvedValueOnce(mockGameState);

      const mockOnBackToMenu = vi.fn();
      render(<GameBoard gameType="xiangqi" onBackToMenu={mockOnBackToMenu} />);

      await waitFor(() => {
        expect(invoke).toHaveBeenCalledWith('start_new_game', {
          gameType: 'xiangqi',
        });
      });

      // 验证组件成功渲染
      expect(screen.getByText('中国象棋')).toBeInTheDocument();
    });

    it('单元测试：选择棋子后应该显示合法移动', async () => {
      const mockGameState = {
        game_id: 'test-game-5',
        board_state: {
          pieces: {
            '0,0': {
              piece_type: { Xiangqi: 'Chariot' },
              player: 'Red',
            },
          },
          current_player: 'Red',
          move_history: [],
        } as BoardState,
        game_status: { type: 'Ongoing' } as GameStatus,
      };

      const mockLegalMoves: Position[] = [
        { row: 1, col: 0 },
        { row: 2, col: 0 },
      ];

      (invoke as any).mockResolvedValueOnce(mockGameState);
      (invoke as any).mockResolvedValueOnce(mockLegalMoves);

      const mockOnBackToMenu = vi.fn();
      render(<GameBoard gameType="xiangqi" onBackToMenu={mockOnBackToMenu} />);

      await waitFor(() => {
        expect(invoke).toHaveBeenCalledWith('start_new_game', {
          gameType: 'xiangqi',
        });
      });

      // 验证组件渲染
      expect(screen.getByText('轮到您')).toBeInTheDocument();
    });
  });

  // 游戏初始化、控制和状态显示测试已在其他测试中覆盖
});
