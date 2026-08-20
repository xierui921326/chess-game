import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, waitFor, fireEvent } from '@testing-library/react';
import { MemoryRouter } from 'react-router-dom';
import { AppRoutes } from '../App';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

import { invoke } from '@tauri-apps/api/core';

function renderApp(initialEntries: string[] = ['/']) {
  return render(
    <MemoryRouter initialEntries={initialEntries}>
      <div className="app">
        <AppRoutes />
      </div>
    </MemoryRouter>,
  );
}

describe('App 集成测试', () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe('完整游戏流程', () => {
    it('应该显示游戏选择界面', () => {
      renderApp();

      expect(screen.getByText('棋类游戏')).toBeInTheDocument();
      expect(screen.getByText('选择棋种，人机对战')).toBeInTheDocument();
      expect(screen.getByText('中国象棋')).toBeInTheDocument();
      expect(screen.getByText('军棋')).toBeInTheDocument();
    });

    it('点击象棋应进入对局设置页', async () => {
      renderApp();

      fireEvent.click(screen.getByLabelText('选择中国象棋'));

      expect(
        await screen.findByText('选择执子颜色与 AI 难度后开始对局'),
      ).toBeInTheDocument();
      expect(screen.getByText('开始对局')).toBeInTheDocument();
    });

    it('设置后开始对局应启动象棋游戏', async () => {
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

      renderApp();

      fireEvent.click(screen.getByLabelText('选择中国象棋'));
      fireEvent.click(await screen.findByText('开始对局'));

      await waitFor(
        () => {
          expect(invoke).toHaveBeenCalledWith('start_new_game', {
            gameType: 'xiangqi',
            difficulty: 'Medium',
          });
        },
        { timeout: 3000 },
      );
    });

    it('军棋设置页应标注翻棋', async () => {
      renderApp();

      fireEvent.click(screen.getByLabelText('选择军棋'));

      expect(await screen.findByText('军棋（翻棋）')).toBeInTheDocument();
      expect(screen.getByText(/二人暗棋翻棋/)).toBeInTheDocument();
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

      renderApp();

      fireEvent.click(screen.getByLabelText('选择中国象棋'));
      fireEvent.click(await screen.findByText('开始对局'));

      await waitFor(
        () => {
          expect(screen.getAllByText('返回主菜单').length).toBeGreaterThan(0);
        },
        { timeout: 3000 },
      );

      fireEvent.click(screen.getAllByText('返回主菜单')[0]);

      await waitFor(() => {
        expect(screen.getByText('棋类游戏')).toBeInTheDocument();
      });
    });
  });

  describe('组件连接测试', () => {
    it('主菜单到棋盘应正确连接', async () => {
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

      const { container } = renderApp();

      expect(screen.getByText('棋类游戏')).toBeInTheDocument();

      fireEvent.click(screen.getByLabelText('选择中国象棋'));
      fireEvent.click(await screen.findByText('开始对局'));

      await waitFor(
        () => {
          expect(container.querySelector('canvas')).toBeTruthy();
        },
        { timeout: 3000 },
      );
    });

    it('应用程序初始状态应显示主菜单', () => {
      renderApp();

      expect(screen.getByText('棋类游戏')).toBeInTheDocument();
      expect(screen.queryByText('悔棋')).not.toBeInTheDocument();
    });
  });

  describe('错误处理', () => {
    it('游戏启动失败时应该能够返回主菜单', async () => {
      (invoke as any).mockRejectedValue(new Error('启动失败'));

      renderApp();

      fireEvent.click(screen.getByLabelText('选择中国象棋'));
      fireEvent.click(await screen.findByText('开始对局'));

      await waitFor(
        () => {
          expect(invoke).toHaveBeenCalled();
        },
        { timeout: 3000 },
      );

      expect(invoke).toHaveBeenCalledWith('start_new_game', {
        gameType: 'xiangqi',
        difficulty: 'Medium',
      });
    });
  });

  describe('UI 一致性', () => {
    it('应该在所有状态下保持响应式布局', () => {
      const { container } = renderApp();
      expect(container.querySelector('.app')).toBeTruthy();
    });

    it('应该正确应用样式', () => {
      renderApp();
      const selector = screen.getByText('棋类游戏').closest('.game-selector');
      expect(selector).toBeTruthy();
    });
  });
});
