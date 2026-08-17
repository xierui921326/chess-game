// TypeScript 类型定义 - 与 Rust 后端数据模型对应

export interface Position {
  row: number;
  col: number;
}

export type Player = 'Red' | 'Black';

// 象棋棋子类型
export type XiangqiPieceType =
  | 'General'    // 将/帅
  | 'Advisor'    // 士
  | 'Elephant'   // 象/相
  | 'Horse'      // 马
  | 'Chariot'    // 车
  | 'Cannon'     // 炮
  | 'Soldier';   // 兵/卒

// 军棋棋子类型
export type JunqiPieceType =
  | 'Flag'       // 军旗
  | 'Landmine'   // 地雷
  | 'Bomb'       // 炸弹
  | 'Commander'  // 司令
  | 'General'    // 军长
  | 'Major'      // 师长
  | 'Colonel'    // 旅长
  | 'Captain'    // 团长
  | 'Battalion'  // 营长
  | 'Company'    // 连长
  | 'Platoon'    // 排长
  | 'Engineer';  // 工兵

export type PieceType =
  | { Xiangqi: XiangqiPieceType }
  | { Junqi: JunqiPieceType };

export interface Piece {
  piece_type: PieceType;
  player: Player;
}

export interface Move {
  from: Position;
  to: Position;
  piece: Piece;
  captured_piece?: Piece;
  timestamp: number;
}

export interface BoardState {
  pieces: Record<string, Piece>; // key: "row,col"
  current_player: Player;
  move_history: Move[];
}

export type GameStatus =
  | { type: 'Ongoing' }
  | { type: 'Check'; player: Player }
  | { type: 'Checkmate'; winner: Player }
  | { type: 'Stalemate' }
  | { type: 'Victory'; winner: Player };

export interface MoveResult {
  success: boolean;
  new_board_state: BoardState;
  game_status: GameStatus;
  captured_piece?: Piece;
}

export type GameType = 'xiangqi' | 'junqi';
