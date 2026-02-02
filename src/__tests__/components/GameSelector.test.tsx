import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/react';
import GameSelector from '../../components/GameSelector';

describe('GameSelector 组件', () => {
  describe('渲染测试', () => {
    it('应该渲染游戏选择界面', () => {
      const mockOnGameSelect = vi.fn();
      render(<GameSelector onGameSelect={mockOnGameSelect} />);
      
      // 检查标题是否存在
      expect(screen.getByText('棋类游戏')).toBeInTheDocument();
      expect(screen.getByText('选择您想要玩的游戏')).toBeInTheDocument();
    });

    it('应该显示中国象棋选项', () => {
      const mockOnGameSelect = vi.fn();
      render(<GameSelector onGameSelect={mockOnGameSelect} />);
      
      expect(screen.getByText('中国象棋')).toBeInTheDocument();
      expect(screen.getByText('经典的中国象棋游戏')).toBeInTheDocument();
    });

    it('应该显示军棋选项', () => {
      const mockOnGameSelect = vi.fn();
      render(<GameSelector onGameSelect={mockOnGameSelect} />);
      
      expect(screen.getByText('军棋')).toBeInTheDocument();
      expect(screen.getByText('策略性的军棋对战')).toBeInTheDocument();
    });

    it('应该渲染两个游戏选项按钮', () => {
      const mockOnGameSelect = vi.fn();
      render(<GameSelector onGameSelect={mockOnGameSelect} />);
      
      const buttons = screen.getAllByRole('button');
      expect(buttons).toHaveLength(2);
    });
  });

  describe('交互测试', () => {
    it('点击中国象棋按钮应该调用 onGameSelect 并传入 xiangqi', () => {
      const mockOnGameSelect = vi.fn();
      render(<GameSelector onGameSelect={mockOnGameSelect} />);
      
      const xiangqiButton = screen.getByLabelText('选择中国象棋');
      fireEvent.click(xiangqiButton);
      
      expect(mockOnGameSelect).toHaveBeenCalledTimes(1);
      expect(mockOnGameSelect).toHaveBeenCalledWith('xiangqi');
    });

    it('点击军棋按钮应该调用 onGameSelect 并传入 junqi', () => {
      const mockOnGameSelect = vi.fn();
      render(<GameSelector onGameSelect={mockOnGameSelect} />);
      
      const junqiButton = screen.getByLabelText('选择军棋');
      fireEvent.click(junqiButton);
      
      expect(mockOnGameSelect).toHaveBeenCalledTimes(1);
      expect(mockOnGameSelect).toHaveBeenCalledWith('junqi');
    });

    it('多次点击应该多次调用 onGameSelect', () => {
      const mockOnGameSelect = vi.fn();
      render(<GameSelector onGameSelect={mockOnGameSelect} />);
      
      const xiangqiButton = screen.getByLabelText('选择中国象棋');
      const junqiButton = screen.getByLabelText('选择军棋');
      
      fireEvent.click(xiangqiButton);
      fireEvent.click(junqiButton);
      fireEvent.click(xiangqiButton);
      
      expect(mockOnGameSelect).toHaveBeenCalledTimes(3);
      expect(mockOnGameSelect).toHaveBeenNthCalledWith(1, 'xiangqi');
      expect(mockOnGameSelect).toHaveBeenNthCalledWith(2, 'junqi');
      expect(mockOnGameSelect).toHaveBeenNthCalledWith(3, 'xiangqi');
    });
  });

  describe('可访问性测试', () => {
    it('按钮应该有正确的 aria-label', () => {
      const mockOnGameSelect = vi.fn();
      render(<GameSelector onGameSelect={mockOnGameSelect} />);
      
      expect(screen.getByLabelText('选择中国象棋')).toBeInTheDocument();
      expect(screen.getByLabelText('选择军棋')).toBeInTheDocument();
    });

    it('按钮应该是可点击的', () => {
      const mockOnGameSelect = vi.fn();
      render(<GameSelector onGameSelect={mockOnGameSelect} />);
      
      const xiangqiButton = screen.getByLabelText('选择中国象棋');
      const junqiButton = screen.getByLabelText('选择军棋');
      
      expect(xiangqiButton).toBeEnabled();
      expect(junqiButton).toBeEnabled();
    });
  });

  describe('样式测试', () => {
    it('中国象棋按钮应该有正确的 CSS 类', () => {
      const mockOnGameSelect = vi.fn();
      render(<GameSelector onGameSelect={mockOnGameSelect} />);
      
      const xiangqiButton = screen.getByLabelText('选择中国象棋');
      expect(xiangqiButton).toHaveClass('game-option', 'xiangqi');
    });

    it('军棋按钮应该有正确的 CSS 类', () => {
      const mockOnGameSelect = vi.fn();
      render(<GameSelector onGameSelect={mockOnGameSelect} />);
      
      const junqiButton = screen.getByLabelText('选择军棋');
      expect(junqiButton).toHaveClass('game-option', 'junqi');
    });
  });
});
