import React, { useRef, useEffect, useState } from 'react';
import type { BoardState, Position } from '../types';
import './BoardRenderer.css';

export type GameType = 'xiangqi' | 'junqi';

interface BoardRendererProps {
  boardState: BoardState;
  selectedPiece: Position | null;
  legalMoves: Position[];
  onCellClick: (position: Position) => void;
  gameType: GameType;
}

// 象棋棋盘尺寸
const XIANGQI_ROWS = 10;
const XIANGQI_COLS = 9;

// 军棋棋盘尺寸
const JUNQI_ROWS = 12;
const JUNQI_COLS = 5;

// 绘制参数 - 缩小以适应一屏内完整展示
const CELL_SIZE = 36;
const PADDING = 20;
const PIECE_RADIUS = 14;

const BoardRenderer: React.FC<BoardRendererProps> = ({
  boardState,
  selectedPiece,
  legalMoves,
  onCellClick,
  gameType,
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const wrapperRef = useRef<HTMLDivElement>(null);
  const [scale, setScale] = useState(1);

  const rows = gameType === 'xiangqi' ? XIANGQI_ROWS : JUNQI_ROWS;
  const cols = gameType === 'xiangqi' ? XIANGQI_COLS : JUNQI_COLS;

  const canvasWidth = cols * CELL_SIZE + PADDING * 2;
  const canvasHeight = rows * CELL_SIZE + PADDING * 2;

  // 根据容器尺寸计算缩放比例，使棋盘完整显示且居中
  useEffect(() => {
    const wrapper = wrapperRef.current;
    if (!wrapper) return;

    const updateScale = () => {
      const { clientWidth: W, clientHeight: H } = wrapper;
      if (W <= 0 || H <= 0) return;
      const s = Math.min(W / canvasWidth, H / canvasHeight, 1);
      setScale(s);
    };

    updateScale();
    const ro = new ResizeObserver(updateScale);
    ro.observe(wrapper);
    return () => ro.disconnect();
  }, [canvasWidth, canvasHeight]);

  // 绘制棋盘
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // 清空画布
    ctx.clearRect(0, 0, canvasWidth, canvasHeight);

    // 绘制背景
    ctx.fillStyle = '#f0d9b5';
    ctx.fillRect(0, 0, canvasWidth, canvasHeight);

    // 绘制棋盘网格
    drawGrid(ctx, rows, cols);

    // 绘制特殊标记（象棋的九宫格、军棋的营地等）
    if (gameType === 'xiangqi') {
      drawXiangqiSpecialMarks(ctx);
    } else {
      drawJunqiSpecialMarks(ctx);
    }

    // 绘制合法移动提示
    drawLegalMoves(ctx, legalMoves);

    // 绘制选中的棋子高亮
    if (selectedPiece) {
      drawSelectedPiece(ctx, selectedPiece);
    }

    // 绘制所有棋子
    drawPieces(ctx, boardState);
  }, [boardState, selectedPiece, legalMoves, gameType, rows, cols, canvasWidth, canvasHeight]);

  // 绘制网格
  const drawGrid = (ctx: CanvasRenderingContext2D, rows: number, cols: number) => {
    ctx.strokeStyle = '#000';
    ctx.lineWidth = 1.5;

    // 绘制横线
    for (let i = 0; i < rows; i++) {
      ctx.beginPath();
      ctx.moveTo(PADDING, PADDING + i * CELL_SIZE);
      ctx.lineTo(PADDING + (cols - 1) * CELL_SIZE, PADDING + i * CELL_SIZE);
      ctx.stroke();
    }

    // 绘制竖线
    for (let i = 0; i < cols; i++) {
      // 象棋中间断开（楚河汉界）
      if (gameType === 'xiangqi') {
        // 上半部分（0-4行）
        ctx.beginPath();
        ctx.moveTo(PADDING + i * CELL_SIZE, PADDING);
        ctx.lineTo(PADDING + i * CELL_SIZE, PADDING + 4 * CELL_SIZE);
        ctx.stroke();
        
        // 下半部分（5-9行）
        ctx.beginPath();
        ctx.moveTo(PADDING + i * CELL_SIZE, PADDING + 5 * CELL_SIZE);
        ctx.lineTo(PADDING + i * CELL_SIZE, PADDING + (rows - 1) * CELL_SIZE);
        ctx.stroke();
      } else {
        // 军棋完整竖线
        ctx.beginPath();
        ctx.moveTo(PADDING + i * CELL_SIZE, PADDING);
        ctx.lineTo(PADDING + i * CELL_SIZE, PADDING + (rows - 1) * CELL_SIZE);
        ctx.stroke();
      }
    }
  };

  // 绘制象棋特殊标记
  const drawXiangqiSpecialMarks = (ctx: CanvasRenderingContext2D) => {
    ctx.strokeStyle = '#000';
    ctx.lineWidth = 1.5;

    // 绘制九宫格斜线（黑方，上方，0-2行）
    ctx.beginPath();
    ctx.moveTo(PADDING + 3 * CELL_SIZE, PADDING);
    ctx.lineTo(PADDING + 5 * CELL_SIZE, PADDING + 2 * CELL_SIZE);
    ctx.stroke();

    ctx.beginPath();
    ctx.moveTo(PADDING + 5 * CELL_SIZE, PADDING);
    ctx.lineTo(PADDING + 3 * CELL_SIZE, PADDING + 2 * CELL_SIZE);
    ctx.stroke();

    // 绘制九宫格斜线（红方，下方，7-9行）
    ctx.beginPath();
    ctx.moveTo(PADDING + 3 * CELL_SIZE, PADDING + 7 * CELL_SIZE);
    ctx.lineTo(PADDING + 5 * CELL_SIZE, PADDING + 9 * CELL_SIZE);
    ctx.stroke();

    ctx.beginPath();
    ctx.moveTo(PADDING + 5 * CELL_SIZE, PADDING + 7 * CELL_SIZE);
    ctx.lineTo(PADDING + 3 * CELL_SIZE, PADDING + 9 * CELL_SIZE);
    ctx.stroke();

    // 绘制楚河汉界（在第4-5行之间）
    ctx.font = 'bold 14px SimSun, serif';
    ctx.fillStyle = '#000';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    
    const riverY = PADDING + 4.5 * CELL_SIZE;
    ctx.fillText('楚河', PADDING + 2 * CELL_SIZE, riverY);
    ctx.fillText('汉界', PADDING + 6 * CELL_SIZE, riverY);
  };

  // 绘制军棋特殊标记（与后端 RED_CAMP_POSITIONS / BLACK_CAMP_POSITIONS 一致）
  const drawJunqiSpecialMarks = (ctx: CanvasRenderingContext2D) => {
    ctx.lineWidth = 2;
    ctx.strokeStyle = '#666';

    // 横向铁路线（第 1, 5, 6, 10 行）
    const railwayRows = [1, 5, 6, 10];
    railwayRows.forEach(row => {
      ctx.beginPath();
      ctx.moveTo(PADDING, PADDING + row * CELL_SIZE);
      ctx.lineTo(PADDING + (JUNQI_COLS - 1) * CELL_SIZE, PADDING + row * CELL_SIZE);
      ctx.stroke();
    });

    // 竖向铁路线（左右两列 0 和 4）
    [0, 4].forEach(col => {
      ctx.beginPath();
      ctx.moveTo(PADDING + col * CELL_SIZE, PADDING + CELL_SIZE);
      ctx.lineTo(PADDING + col * CELL_SIZE, PADDING + 10 * CELL_SIZE);
      ctx.stroke();
    });

    // 营地（与后端一致：黑方 (0,0),(0,2),(0,4),(1,1),(1,3)；红方 (10,1),(10,3),(11,0),(11,2),(11,4)）
    ctx.fillStyle = 'rgba(100, 100, 100, 0.25)';
    ctx.strokeStyle = '#555';
    ctx.lineWidth = 1;

    const blackCamps: { row: number; col: number }[] = [
      { row: 0, col: 0 }, { row: 0, col: 2 }, { row: 0, col: 4 },
      { row: 1, col: 1 }, { row: 1, col: 3 },
    ];
    const redCamps: { row: number; col: number }[] = [
      { row: 10, col: 1 }, { row: 10, col: 3 },
      { row: 11, col: 0 }, { row: 11, col: 2 }, { row: 11, col: 4 },
    ];

    [...blackCamps, ...redCamps].forEach(camp => {
      const x = PADDING + camp.col * CELL_SIZE;
      const y = PADDING + camp.row * CELL_SIZE;
      ctx.beginPath();
      ctx.arc(x, y, 6, 0, 2 * Math.PI);
      ctx.fill();
      ctx.stroke();
    });

    // 行营三角形连线（黑方：第0-1行五个营地；红方：第10-11行五个营地）
    ctx.strokeStyle = '#888';
    ctx.lineWidth = 1;
    // 黑方行营三角
    ctx.beginPath();
    ctx.moveTo(PADDING + 0 * CELL_SIZE, PADDING + 1 * CELL_SIZE);
    ctx.lineTo(PADDING + 1 * CELL_SIZE, PADDING + 0 * CELL_SIZE);
    ctx.lineTo(PADDING + 2 * CELL_SIZE, PADDING + 1 * CELL_SIZE);
    ctx.lineTo(PADDING + 1 * CELL_SIZE, PADDING + 1 * CELL_SIZE);
    ctx.closePath();
    ctx.stroke();
    ctx.beginPath();
    ctx.moveTo(PADDING + 2 * CELL_SIZE, PADDING + 0 * CELL_SIZE);
    ctx.lineTo(PADDING + 3 * CELL_SIZE, PADDING + 1 * CELL_SIZE);
    ctx.lineTo(PADDING + 4 * CELL_SIZE, PADDING + 0 * CELL_SIZE);
    ctx.lineTo(PADDING + 3 * CELL_SIZE, PADDING + 1 * CELL_SIZE);
    ctx.closePath();
    ctx.stroke();
    // 红方行营三角
    ctx.beginPath();
    ctx.moveTo(PADDING + 0 * CELL_SIZE, PADDING + 10 * CELL_SIZE);
    ctx.lineTo(PADDING + 1 * CELL_SIZE, PADDING + 11 * CELL_SIZE);
    ctx.lineTo(PADDING + 2 * CELL_SIZE, PADDING + 10 * CELL_SIZE);
    ctx.lineTo(PADDING + 1 * CELL_SIZE, PADDING + 10 * CELL_SIZE);
    ctx.closePath();
    ctx.stroke();
    ctx.beginPath();
    ctx.moveTo(PADDING + 2 * CELL_SIZE, PADDING + 11 * CELL_SIZE);
    ctx.lineTo(PADDING + 3 * CELL_SIZE, PADDING + 10 * CELL_SIZE);
    ctx.lineTo(PADDING + 4 * CELL_SIZE, PADDING + 11 * CELL_SIZE);
    ctx.lineTo(PADDING + 3 * CELL_SIZE, PADDING + 10 * CELL_SIZE);
    ctx.closePath();
    ctx.stroke();
  };

  // 绘制合法移动提示
  const drawLegalMoves = (ctx: CanvasRenderingContext2D, moves: Position[]) => {
    ctx.fillStyle = 'rgba(0, 255, 0, 0.3)';
    moves.forEach((pos) => {
      const x = PADDING + pos.col * CELL_SIZE;
      const y = PADDING + pos.row * CELL_SIZE;
      ctx.beginPath();
      ctx.arc(x, y, 8, 0, 2 * Math.PI);
      ctx.fill();
    });
  };

  // 绘制选中的棋子高亮
  const drawSelectedPiece = (ctx: CanvasRenderingContext2D, pos: Position) => {
    const x = PADDING + pos.col * CELL_SIZE;
    const y = PADDING + pos.row * CELL_SIZE;
    
    ctx.strokeStyle = '#ff0';
    ctx.lineWidth = 2.5;
    ctx.beginPath();
    ctx.arc(x, y, PIECE_RADIUS + 4, 0, 2 * Math.PI);
    ctx.stroke();
  };

  // 绘制所有棋子
  const drawPieces = (ctx: CanvasRenderingContext2D, board: BoardState) => {
    Object.entries(board.pieces).forEach(([key, piece]) => {
      const [row, col] = key.split(',').map(Number);
      const x = PADDING + col * CELL_SIZE;
      const y = PADDING + row * CELL_SIZE;

      // 绘制棋子圆形背景
      ctx.fillStyle = piece.player === 'Red' ? '#ff6b6b' : '#4a4a4a';
      ctx.beginPath();
      ctx.arc(x, y, PIECE_RADIUS, 0, 2 * Math.PI);
      ctx.fill();

      // 绘制棋子边框
      ctx.strokeStyle = piece.player === 'Red' ? '#c92a2a' : '#000';
      ctx.lineWidth = 2;
      ctx.stroke();

      // 绘制棋子文字
      ctx.fillStyle = '#fff';
      ctx.font = 'bold 12px SimSun, serif';
      ctx.textAlign = 'center';
      ctx.textBaseline = 'middle';
      
      const pieceText = getPieceText(piece);
      ctx.fillText(pieceText, x, y);
    });
  };

  // 获取棋子显示文字
  const getPieceText = (piece: any): string => {
    if ('Xiangqi' in piece.piece_type) {
      const xiangqiPiece = piece.piece_type.Xiangqi;
      const isRed = piece.player === 'Red';
      
      switch (xiangqiPiece) {
        case 'General': return isRed ? '帅' : '将';
        case 'Advisor': return '士';
        case 'Elephant': return isRed ? '相' : '象';
        case 'Horse': return '马';
        case 'Chariot': return '车';
        case 'Cannon': return '炮';
        case 'Soldier': return isRed ? '兵' : '卒';
        default: return '?';
      }
    } else if ('Junqi' in piece.piece_type) {
      const junqiPiece = piece.piece_type.Junqi;
      
      switch (junqiPiece) {
        case 'Flag': return '旗';
        case 'Landmine': return '雷';
        case 'Bomb': return '炸';
        case 'Commander': return '司';
        case 'General': return '军';
        case 'Major': return '师';
        case 'Colonel': return '旅';
        case 'Captain': return '团';
        case 'Battalion': return '营';
        case 'Company': return '连';
        case 'Platoon': return '排';
        case 'Engineer': return '工';
        default: return '?';
      }
    }
    return '?';
  };

  // 处理点击事件（考虑缩放后坐标）
  const handleCanvasClick = (event: React.MouseEvent<HTMLCanvasElement>) => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const rect = canvas.getBoundingClientRect();
    // 点击位置相对于显示区域的坐标，需换算回画布坐标
    const x = (event.clientX - rect.left) / scale;
    const y = (event.clientY - rect.top) / scale;

    const col = Math.round((x - PADDING) / CELL_SIZE);
    const row = Math.round((y - PADDING) / CELL_SIZE);

    if (row >= 0 && row < rows && col >= 0 && col < cols) {
      onCellClick({ row, col });
    }
  };

  return (
    <div className="board-renderer" ref={wrapperRef}>
      <div
        className="board-canvas-wrapper"
        style={{ transform: scale < 1 ? `scale(${scale})` : undefined }}
      >
        <canvas
          ref={canvasRef}
          width={canvasWidth}
          height={canvasHeight}
          onClick={handleCanvasClick}
          className="board-canvas"
        />
      </div>
    </div>
  );
};

export default BoardRenderer;
