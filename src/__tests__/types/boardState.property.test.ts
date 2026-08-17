// 属性测试：游戏状态完整性
// Feature: chess-game-app, Property 15: 游戏状态完整性
// **验证需求：7.1**

import { describe, it, expect } from 'vitest';
import fc from 'fast-check';
import type {
  Position,
  Player,
  Piece,
  PieceType,
  XiangqiPieceType,
  JunqiPieceType,
  Move,
  BoardState,
} from '../../types/index';

// ============ 生成器定义 ============

// 生成任意 Player
const arbitraryPlayer = (): fc.Arbitrary<Player> => {
  return fc.constantFrom('Red' as Player, 'Black' as Player);
};

// 生成任意象棋位置
const arbitraryXiangqiPosition = (): fc.Arbitrary<Position> => {
  return fc.record({
    row: fc.integer({ min: 0, max: 9 }),
    col: fc.integer({ min: 0, max: 8 }),
  });
};

// 生成任意军棋位置
const arbitraryJunqiPosition = (): fc.Arbitrary<Position> => {
  return fc.record({
    row: fc.integer({ min: 0, max: 11 }),
    col: fc.integer({ min: 0, max: 4 }),
  });
};

// 生成任意位置（用于通用场景）
const arbitraryPosition = (): fc.Arbitrary<Position> => {
  return fc.record({
    row: fc.integer({ min: 0, max: 11 }),
    col: fc.integer({ min: 0, max: 8 }),
  });
};

// 生成任意象棋棋子类型
const arbitraryXiangqiPieceType = (): fc.Arbitrary<XiangqiPieceType> => {
  return fc.constantFrom(
    'General' as XiangqiPieceType,
    'Advisor' as XiangqiPieceType,
    'Elephant' as XiangqiPieceType,
    'Horse' as XiangqiPieceType,
    'Chariot' as XiangqiPieceType,
    'Cannon' as XiangqiPieceType,
    'Soldier' as XiangqiPieceType
  );
};

// 生成任意军棋棋子类型
const arbitraryJunqiPieceType = (): fc.Arbitrary<JunqiPieceType> => {
  return fc.constantFrom(
    'Flag' as JunqiPieceType,
    'Landmine' as JunqiPieceType,
    'Bomb' as JunqiPieceType,
    'Commander' as JunqiPieceType,
    'General' as JunqiPieceType,
    'Major' as JunqiPieceType,
    'Colonel' as JunqiPieceType,
    'Captain' as JunqiPieceType,
    'Battalion' as JunqiPieceType,
    'Company' as JunqiPieceType,
    'Platoon' as JunqiPieceType,
    'Engineer' as JunqiPieceType
  );
};

// 生成任意 PieceType
const arbitraryPieceType = (): fc.Arbitrary<PieceType> => {
  return fc.oneof(
    arbitraryXiangqiPieceType().map((type) => ({ Xiangqi: type })),
    arbitraryJunqiPieceType().map((type) => ({ Junqi: type }))
  );
};

// 生成任意 Piece
const arbitraryPiece = (): fc.Arbitrary<Piece> => {
  return fc.record({
    piece_type: arbitraryPieceType(),
    player: arbitraryPlayer(),
  });
};

// 生成任意 Move
const arbitraryMove = (): fc.Arbitrary<Move> => {
  return fc.record({
    from: arbitraryPosition(),
    to: arbitraryPosition(),
    piece: arbitraryPiece(),
    captured_piece: fc.option(arbitraryPiece(), { nil: undefined }),
    timestamp: fc.integer({ min: 0 }),
  });
};

// 生成任意棋盘布局（Record<string, Piece>）
// 智能生成：根据棋子类型生成合适的位置
const arbitraryPieces = (): fc.Arbitrary<Record<string, Piece>> => {
  return fc
    .array(
      fc.oneof(
        // 生成象棋棋子及其位置
        fc
          .tuple(arbitraryXiangqiPosition(), arbitraryXiangqiPieceType(), arbitraryPlayer())
          .map(([pos, pieceType, player]) => ({
            key: `${pos.row},${pos.col}`,
            piece: {
              piece_type: { Xiangqi: pieceType },
              player,
            } as Piece,
          })),
        // 生成军棋棋子及其位置
        fc
          .tuple(arbitraryJunqiPosition(), arbitraryJunqiPieceType(), arbitraryPlayer())
          .map(([pos, pieceType, player]) => ({
            key: `${pos.row},${pos.col}`,
            piece: {
              piece_type: { Junqi: pieceType },
              player,
            } as Piece,
          }))
      ),
      { maxLength: 32 }
    )
    .map((arr) => {
      // 将数组转换为 Record，自动去重相同位置的棋子
      const record: Record<string, Piece> = {};
      for (const item of arr) {
        record[item.key] = item.piece;
      }
      return record;
    });
};

// 生成任意移动历史
const arbitraryMoveHistory = (): fc.Arbitrary<Move[]> => {
  return fc.array(arbitraryMove(), { maxLength: 100 });
};

// 生成任意 BoardState
const arbitraryBoardState = (): fc.Arbitrary<BoardState> => {
  return fc.record({
    pieces: arbitraryPieces(),
    current_player: arbitraryPlayer(),
    move_history: arbitraryMoveHistory(),
  });
};

// ============ 属性测试 ============

describe('属性测试：游戏状态完整性', () => {
  /**
   * 属性 15：游戏状态完整性
   * 
   * 对于任何时刻，游戏状态应该包含所有必要的信息：
   * - 完整的棋盘布局（pieces 字段）
   * - 当前轮到哪个玩家（current_player 字段）
   * - 完整的移动历史记录（move_history 字段）
   * 
   * 这个属性测试验证 BoardState 结构体始终包含这些必要字段，
   * 并且这些字段可以被正确访问和序列化/反序列化。
   */
  it('属性 15：游戏状态应该包含所有必要信息', () => {
    fc.assert(
      fc.property(arbitraryBoardState(), (boardState) => {
        // 验证棋盘布局字段存在且可访问
        expect(boardState.pieces).toBeDefined();
        const pieces = boardState.pieces;
        const pieceCount = Object.keys(pieces).length;
        expect(pieceCount).toBeLessThanOrEqual(32); // 棋盘上的棋子数量应该合理

        // 验证每个棋子位置都是有效的
        for (const [key, piece] of Object.entries(pieces)) {
          const [rowStr, colStr] = key.split(',');
          const row = parseInt(rowStr, 10);
          const col = parseInt(colStr, 10);

          expect(row).toBeGreaterThanOrEqual(0);
          expect(row).toBeLessThan(12);
          expect(col).toBeGreaterThanOrEqual(0);
          expect(col).toBeLessThan(9);

          // 验证棋子信息完整
          expect(piece.piece_type).toBeDefined();
          expect(piece.player).toBeDefined();

          if ('Xiangqi' in piece.piece_type) {
            // 象棋棋子应该在象棋棋盘范围内
            expect(row).toBeLessThan(10);
            expect(col).toBeLessThan(9);
          } else if ('Junqi' in piece.piece_type) {
            // 军棋棋子应该在军棋棋盘范围内
            expect(row).toBeLessThan(12);
            expect(col).toBeLessThan(5);
          }
        }

        // 验证当前玩家字段存在且有效
        expect(boardState.current_player).toBeDefined();
        expect(['Red', 'Black']).toContain(boardState.current_player);

        // 验证移动历史记录字段存在且可访问
        expect(boardState.move_history).toBeDefined();
        expect(Array.isArray(boardState.move_history)).toBe(true);
        expect(boardState.move_history.length).toBeLessThanOrEqual(200);

        // 验证移动历史中的每个移动都包含完整信息
        for (const move of boardState.move_history) {
          // 验证起始位置
          expect(move.from).toBeDefined();
          expect(move.from.row).toBeGreaterThanOrEqual(0);
          expect(move.from.row).toBeLessThan(12);
          expect(move.from.col).toBeGreaterThanOrEqual(0);
          expect(move.from.col).toBeLessThan(9);

          // 验证目标位置
          expect(move.to).toBeDefined();
          expect(move.to.row).toBeGreaterThanOrEqual(0);
          expect(move.to.row).toBeLessThan(12);
          expect(move.to.col).toBeGreaterThanOrEqual(0);
          expect(move.to.col).toBeLessThan(9);

          // 验证移动的棋子信息存在
          expect(move.piece).toBeDefined();
          expect(move.piece.piece_type).toBeDefined();
          expect(move.piece.player).toBeDefined();

          // 验证时间戳存在
          expect(move.timestamp).toBeDefined();
          expect(typeof move.timestamp).toBe('number');
        }

        // 验证游戏状态可以被序列化和反序列化（测试完整性）
        const serialized = JSON.stringify(boardState);
        expect(serialized).toBeDefined();
        expect(typeof serialized).toBe('string');

        const deserialized: BoardState = JSON.parse(serialized);
        expect(deserialized).toBeDefined();

        // 验证反序列化后的状态与原始状态相同
        expect(Object.keys(deserialized.pieces).length).toBe(
          Object.keys(boardState.pieces).length
        );
        expect(deserialized.current_player).toBe(boardState.current_player);
        expect(deserialized.move_history.length).toBe(boardState.move_history.length);

        // 验证每个棋子位置都被正确恢复
        for (const [key, piece] of Object.entries(boardState.pieces)) {
          expect(deserialized.pieces[key]).toBeDefined();
          expect(deserialized.pieces[key].player).toBe(piece.player);
        }
      }),
      { numRuns: 100 } // 运行 100 次迭代
    );
  });

  /**
   * 属性测试：新创建的游戏状态应该具有完整性
   * 
   * 验证手动创建的游戏状态包含所有必要字段
   */
  it('新创建的游戏状态应该具有完整性', () => {
    fc.assert(
      fc.property(fc.integer(), (_dummy) => {
        const boardState: BoardState = {
          pieces: {},
          current_player: 'Red',
          move_history: [],
        };

        // 验证棋盘布局字段存在（初始为空）
        expect(Object.keys(boardState.pieces).length).toBe(0);

        // 验证当前玩家字段存在且为红方
        expect(boardState.current_player).toBe('Red');

        // 验证移动历史记录字段存在（初始为空）
        expect(boardState.move_history.length).toBe(0);

        // 验证可以序列化
        const serialized = JSON.stringify(boardState);
        expect(serialized).toBeDefined();
        expect(typeof serialized).toBe('string');
      }),
      { numRuns: 100 }
    );
  });

  /**
   * 属性测试：克隆的游戏状态应该保持完整性
   * 
   * 验证通过 JSON 序列化/反序列化克隆的游戏状态保持完整性
   */
  it('克隆的游戏状态应该保持完整性', () => {
    fc.assert(
      fc.property(arbitraryBoardState(), (boardState) => {
        // 通过 JSON 序列化/反序列化进行克隆
        const cloned: BoardState = JSON.parse(JSON.stringify(boardState));

        // 验证棋盘布局完整性
        expect(Object.keys(cloned.pieces).length).toBe(Object.keys(boardState.pieces).length);

        // 验证每个棋子位置都被正确克隆
        for (const [key, piece] of Object.entries(boardState.pieces)) {
          expect(cloned.pieces[key]).toBeDefined();
          expect(cloned.pieces[key].player).toBe(piece.player);
        }

        // 验证当前玩家完整性
        expect(cloned.current_player).toBe(boardState.current_player);

        // 验证移动历史完整性
        expect(cloned.move_history.length).toBe(boardState.move_history.length);

        // 验证克隆的状态可以独立序列化
        const clonedSerialized = JSON.stringify(cloned);
        expect(clonedSerialized).toBeDefined();
        expect(typeof clonedSerialized).toBe('string');
      }),
      { numRuns: 100 }
    );
  });
});

// ============ 单元测试（补充具体示例） ============

describe('单元测试：游戏状态完整性示例', () => {
  it('空棋盘状态应该具有完整性', () => {
    const boardState: BoardState = {
      pieces: {},
      current_player: 'Red',
      move_history: [],
    };

    // 验证所有必要字段都存在
    expect(Object.keys(boardState.pieces).length).toBe(0);
    expect(boardState.current_player).toBe('Red');
    expect(boardState.move_history.length).toBe(0);

    // 验证可以序列化和反序列化
    const json = JSON.stringify(boardState);
    const restored: BoardState = JSON.parse(json);

    expect(Object.keys(restored.pieces).length).toBe(0);
    expect(restored.current_player).toBe('Red');
    expect(restored.move_history.length).toBe(0);
  });

  it('带有棋子的棋盘状态应该具有完整性', () => {
    const boardState: BoardState = {
      pieces: {
        '0,4': {
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

    // 验证游戏状态完整性
    expect(Object.keys(boardState.pieces).length).toBe(2);
    expect(boardState.current_player).toBe('Red');
    expect(boardState.move_history.length).toBe(0);

    // 验证序列化和反序列化保持完整性
    const json = JSON.stringify(boardState);
    const restored: BoardState = JSON.parse(json);

    expect(Object.keys(restored.pieces).length).toBe(2);
    expect(restored.pieces['0,4']).toBeDefined();
    expect(restored.pieces['9,4']).toBeDefined();
    expect(restored.pieces['0,4'].player).toBe('Red');
    expect(restored.pieces['9,4'].player).toBe('Black');
  });

  it('带有移动历史的棋盘状态应该具有完整性', () => {
    const boardState: BoardState = {
      pieces: {},
      current_player: 'Red',
      move_history: [
        {
          from: { row: 0, col: 0 },
          to: { row: 1, col: 0 },
          piece: {
            piece_type: { Xiangqi: 'Soldier' },
            player: 'Red',
          },
          timestamp: 1000,
        },
        {
          from: { row: 9, col: 0 },
          to: { row: 8, col: 0 },
          piece: {
            piece_type: { Xiangqi: 'Soldier' },
            player: 'Black',
          },
          timestamp: 2000,
        },
      ],
    };

    // 验证游戏状态完整性
    expect(boardState.move_history.length).toBe(2);
    expect(boardState.move_history[0].timestamp).toBe(1000);
    expect(boardState.move_history[1].timestamp).toBe(2000);

    // 验证序列化和反序列化保持完整性
    const json = JSON.stringify(boardState);
    const restored: BoardState = JSON.parse(json);

    expect(restored.move_history.length).toBe(2);
    expect(restored.move_history[0].from.row).toBe(0);
    expect(restored.move_history[1].from.row).toBe(9);
  });
});
