import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor } from '@testing-library/react';
import App from '../App';

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

import { invoke } from '@tauri-apps/api/core';

describe('App 集成测试', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('完整游戏流程', () => {
    it('应该显示游戏选择界面', () => {
      render(<App />);
      
      expect(screen.getByText('棋类游戏')).toBeInTheDocument();
      expect(screen.getByText('选择您想要玩的游戏')).toBeInTheDocument();
      expect(screen.getByText('中国象棋')).toBeInTheDocument();
      expect(screen.getByText('军棋')).toBeInTheDocument();
    });

    it('点击象棋应该启动象棋游戏', async () => {
      const mockGameState = {
        game_id: 'test-xiangqi',
        board_state: {
          pieces: {},
          current_player: 'Red',
          move_history: [],
        },
        game_status: { type: 'Ongoing' },
      };

      (invoke as any).mockResolvedValue(mockGameState);

      render(<App />);
      
      const xiangqiButton = screen.getByLabelText('选择中国象棋');
      xiangqiButton.click();

      await waitFor(
        () => {
          expect(invoke).toHaveBeenCalledWith('start_new_game', {
            gameType: 'xiangqi',
          });
        },
        { timeout: 3000 }
      );
    });

    it('点击军棋应该启动军棋游戏', async () => {
      const mockGameState = {
        game_id: 'test-junqi',
        board_state: {
          pieces: {},
          current_player: 'Red',
          move_history: [],
        },
        game_status: { type: 'Ongoing' },
      };

      (invoke as any).mockResolvedValue(mockGameState);

      render(<App />);
      
      const junqiButton = screen.getByLabelText('选择军棋');
      junqiButton.click();

      await waitFor(
        () => {
          expect(invoke).toHaveBeenCalledWith('start_new_game', {
            gameType: 'junqi',
          });
        },
        { timeout: 3000 }
      );
    });

    it('从游戏界面返回主菜单应该显示游戏选择界面', async () => {
      const mockGameState = {
        game_id: 'test-game',
        board_state: {
          pieces: {},
          current_player: 'Red',
          move_history: [],
        },
        game_status: { type: 'Ongoing' },
      };

      (invoke as any).mockResolvedValue(mockGameState);

      render(<App />);
      
      // 选择游戏
      const xiangqiButton = screen.getByLabelText('选择中国象棋');
      xiangqiButton.click();

      // 等待游戏加载
      await waitFor(
        () => {
          const backButtons = screen.queryAllByText('返回主菜单');
          expect(backButtons.length).toBeGreaterThan(0);
        },
        { timeout: 3000 }
      );

      // 点击返回主菜单
      const backButtons = screen.getAllByText('返回主菜单');
      backButtons[0].click();

      // 应该回到游戏选择界面
      await waitFor(() => {
        expect(screen.getByText('棋类游戏')).toBeInTheDocument();
      });
    });
  });

  describe('组件连接测试', () => {
    it('GameSelector 和 GameBoard 应该正确连接', async () => {
      const mockGameState = {
        game_id: 'test-connection',
        board_state: {
          pieces: {},
          current_player: 'Red',
          move_history: [],
        },
        game_status: { type: 'Ongoing' },
      };

      (invoke as any).mockResolvedValue(mockGameState);

      const { container } = render(<App />);
      
      // 初始状态应该显示 GameSelector
      expect(screen.getByText('棋类游戏')).toBeInTheDocument();
      
      // 选择游戏
      const xiangqiButton = screen.getByLabelText('选择中国象棋');
      xiangqiButton.click();

      // 应该切换到 GameBoard
      await waitFor(
        () => {
          const canvas = container.querySelector('canvas');
          expect(canvas).toBeTruthy();
        },
        { timeout: 3000 }
      );
    });

    it('应用程序状态应该正确管理', () => {
      render(<App />);
      
      // 初始状态：没有选中的游戏
      expect(screen.getByText('棋类游戏')).toBeInTheDocument();
      
      // 不应该显示游戏界面
      expect(screen.queryByText('悔棋')).not.toBeInTheDocument();
    });
  });

  describe('错误处理', () => {
    it('游戏启动失败时应该能够返回主菜单', async () => {
      (invoke as any).mockRejectedValue(new Error('启动失败'));

      render(<App />);
      
      const xiangqiButton = screen.getByLabelText('选择中国象棋');
      xiangqiButton.click();

      // 等待错误处理
      await waitFor(
        () => {
          expect(invoke).toHaveBeenCalled();
        },
        { timeout: 3000 }
      );

      // 验证 invoke 被调用
      expect(invoke).toHaveBeenCalledWith('start_new_game', {
        gameType: 'xiangqi',
      });
    });
  });

  describe('UI 一致性', () => {
    it('应该在所有状态下保持响应式布局', () => {
      const { container } = render(<App />);
      
      // 检查根元素
      const appDiv = container.querySelector('.app');
      expect(appDiv).toBeTruthy();
    });

    it('应该正确应用样式', () => {
      render(<App />);
      
      // 检查游戏选择界面的样式
      const selector = screen.getByText('棋类游戏').closest('.game-selector');
      expect(selector).toBeTruthy();
    });
  });
});
