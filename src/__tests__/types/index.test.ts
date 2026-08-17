// TypeScript 类型定义测试
import { describe, it, expect } from 'vitest';
import type {
  Position,
  Player,
  Piece,
  XiangqiPieceType,
  JunqiPieceType,
  Move,
  BoardState,
  GameStatus,
  MoveResult,
} from '../../types/index';

describe('数据类型定义测试', () => {
  describe('Position', () => {
    it('应该创建有效的位置对象', () => {
      const pos: Position = { row: 0, col: 0 };
      expect(pos.row).toBe(0);
      expect(pos.col).toBe(0);
    });

    it('应该支持不同的位置值', () => {
      const pos1: Position = { row: 9, col: 8 };
      const pos2: Position = { row: 11, col: 4 };
      
      expect(pos1.row).toBe(9);
      expect(pos1.col).toBe(8);
      expect(pos2.row).toBe(11);
      expect(pos2.col).toBe(4);
    });
  });

  describe('Player', () => {
    it('应该支持 Red 和 Black 玩家', () => {
      const red: Player = 'Red';
      const black: Player = 'Black';
      
      expect(red).toBe('Red');
      expect(black).toBe('Black');
    });
  });

  describe('Piece', () => {
    it('应该创建象棋棋子', () => {
      const piece: Piece = {
        piece_type: { Xiangqi: 'General' },
        player: 'Red',
      };
      
      expect(piece.player).toBe('Red');
      expect('Xiangqi' in piece.piece_type).toBe(true);
    });

    it('应该创建军棋棋子', () => {
      const piece: Piece = {
        piece_type: { Junqi: 'Commander' },
        player: 'Black',
      };
      
      expect(piece.player).toBe('Black');
      expect('Junqi' in piece.piece_type).toBe(true);
    });

    it('应该支持所有象棋棋子类型', () => {
      const xiangqiTypes: XiangqiPieceType[] = [
        'General',
        'Advisor',
        'Elephant',
        'Horse',
        'Chariot',
        'Cannon',
        'Soldier',
      ];
      
      xiangqiTypes.forEach((type) => {
        const piece: Piece = {
          piece_type: { Xiangqi: type },
          player: 'Red',
        };
        expect(piece).toBeDefined();
      });
    });

    it('应该支持所有军棋棋子类型', () => {
      const junqiTypes: JunqiPieceType[] = [
        'Flag',
        'Landmine',
        'Bomb',
        'Commander',
        'General',
        'Major',
        'Colonel',
        'Captain',
        'Battalion',
        'Company',
        'Platoon',
        'Engineer',
      ];
      
      junqiTypes.forEach((type) => {
        const piece: Piece = {
          piece_type: { Junqi: type },
          player: 'Black',
        };
        expect(piece).toBeDefined();
      });
    });
  });

  describe('Move', () => {
    it('应该创建移动对象', () => {
      const move: Move = {
        from: { row: 0, col: 0 },
        to: { row: 1, col: 0 },
        piece: {
          piece_type: { Xiangqi: 'Soldier' },
          player: 'Red',
        },
        timestamp: Date.now(),
      };
      
      expect(move.from.row).toBe(0);
      expect(move.to.row).toBe(1);
      expect(move.piece.player).toBe('Red');
    });

    it('应该支持可选的 captured_piece', () => {
      const moveWithCapture: Move = {
        from: { row: 0, col: 0 },
        to: { row: 1, col: 0 },
        piece: {
          piece_type: { Xiangqi: 'Chariot' },
          player: 'Red',
        },
        captured_piece: {
          piece_type: { Xiangqi: 'Soldier' },
          player: 'Black',
        },
        timestamp: Date.now(),
      };
      
      expect(moveWithCapture.captured_piece).toBeDefined();
      expect(moveWithCapture.captured_piece?.player).toBe('Black');
    });
  });

  describe('BoardState', () => {
    it('应该创建空棋盘状态', () => {
      const board: BoardState = {
        pieces: {},
        current_player: 'Red',
        move_history: [],
      };
      
      expect(Object.keys(board.pieces).length).toBe(0);
      expect(board.current_player).toBe('Red');
      expect(board.move_history.length).toBe(0);
    });

    it('应该支持添加棋子', () => {
      const board: BoardState = {
        pieces: {
          '0,0': {
            piece_type: { Xiangqi: 'General' },
            player: 'Red',
          },
          '9,4': {
            piece_type: { Xiangqi: 'General' },
            player: 'Black',
          },
        },
        current_player: 'Red',
        move_history: [],
      };
      
      expect(Object.keys(board.pieces).length).toBe(2);
      expect(board.pieces['0,0'].player).toBe('Red');
      expect(board.pieces['9,4'].player).toBe('Black');
    });
  });

  describe('GameStatus', () => {
    it('应该支持 Ongoing 状态', () => {
      const status: GameStatus = { type: 'Ongoing' };
      expect(status.type).toBe('Ongoing');
    });

    it('应该支持 Check 状态', () => {
      const status: GameStatus = { type: 'Check', player: 'Red' };
      expect(status.type).toBe('Check');
      if (status.type === 'Check') {
        expect(status.player).toBe('Red');
      }
    });

    it('应该支持 Checkmate 状态', () => {
      const status: GameStatus = { type: 'Checkmate', winner: 'Black' };
      expect(status.type).toBe('Checkmate');
      if (status.type === 'Checkmate') {
        expect(status.winner).toBe('Black');
      }
    });

    it('应该支持 Stalemate 状态', () => {
      const status: GameStatus = { type: 'Stalemate' };
      expect(status.type).toBe('Stalemate');
    });

    it('应该支持 Victory 状态', () => {
      const status: GameStatus = { type: 'Victory', winner: 'Red' };
      expect(status.type).toBe('Victory');
      if (status.type === 'Victory') {
        expect(status.winner).toBe('Red');
      }
    });
  });

  describe('MoveResult', () => {
    it('应该创建成功的移动结果', () => {
      const result: MoveResult = {
        success: true,
        new_board_state: {
          pieces: {},
          current_player: 'Black',
          move_history: [],
        },
        game_status: { type: 'Ongoing' },
      };
      
      expect(result.success).toBe(true);
      expect(result.new_board_state.current_player).toBe('Black');
      expect(result.game_status.type).toBe('Ongoing');
    });

    it('应该支持带有吃子的移动结果', () => {
      const result: MoveResult = {
        success: true,
        new_board_state: {
          pieces: {},
          current_player: 'Black',
          move_history: [],
        },
        game_status: { type: 'Ongoing' },
        captured_piece: {
          piece_type: { Xiangqi: 'Soldier' },
          player: 'Black',
        },
      };
      
      expect(result.captured_piece).toBeDefined();
      expect(result.captured_piece?.player).toBe('Black');
    });
  });
});
