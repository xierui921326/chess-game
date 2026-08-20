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

const XIANGQI_ROWS = 10;
const XIANGQI_COLS = 9;
const JUNQI_ROWS = 12;
const JUNQI_COLS = 5;

const MIN_CELL = 22;
const FALLBACK_CELL = 40;
const MIN_PADDING = 16;

interface BoardLayout {
  cellSize: number;
  padding: number;
  pieceRadius: number;
  canvasWidth: number;
  canvasHeight: number;
}

function computeLayout(
  rows: number,
  cols: number,
  availWidth: number,
  availHeight: number,
): BoardLayout {
  const spansX = cols - 1;
  const spansY = rows - 1;

  const fitFromSize = (cellSize: number): BoardLayout => {
    const pieceRadius = Math.max(8, Math.floor(cellSize * (rows > 10 ? 0.42 : 0.38)));
    const padding = Math.max(MIN_PADDING, pieceRadius + 6);
    return {
      cellSize,
      padding,
      pieceRadius,
      canvasWidth: spansX * cellSize + padding * 2,
      canvasHeight: spansY * cellSize + padding * 2,
    };
  };

  if (availWidth <= 0 || availHeight <= 0) {
    return fitFromSize(FALLBACK_CELL);
  }

  // 减去棋盘描边，避免 100% 高度时边框撑破容器
  const maxW = Math.max(1, availWidth - 4);
  const maxH = Math.max(1, availHeight - 4);

  for (let cell = Math.min(96, Math.floor(Math.min(maxW, maxH))); cell >= MIN_CELL; cell--) {
    const layout = fitFromSize(cell);
    if (layout.canvasWidth <= maxW && layout.canvasHeight <= maxH) {
      return layout;
    }
  }

  return fitFromSize(MIN_CELL);
}

const BoardRenderer: React.FC<BoardRendererProps> = ({
  boardState,
  selectedPiece,
  legalMoves,
  onCellClick,
  gameType,
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const wrapperRef = useRef<HTMLDivElement>(null);
  const rows = gameType === 'xiangqi' ? XIANGQI_ROWS : JUNQI_ROWS;
  const cols = gameType === 'xiangqi' ? XIANGQI_COLS : JUNQI_COLS;
  const [layout, setLayout] = useState<BoardLayout>(() =>
    computeLayout(rows, cols, 0, 0),
  );

  useEffect(() => {
    const wrapper = wrapperRef.current;
    if (!wrapper) return;

    const updateLayout = () => {
      setLayout(computeLayout(rows, cols, wrapper.clientWidth, wrapper.clientHeight));
    };

    updateLayout();
    const ro = new ResizeObserver(updateLayout);
    ro.observe(wrapper);
    return () => ro.disconnect();
  }, [rows, cols]);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const dpr = window.devicePixelRatio || 1;
    canvas.width = Math.round(layout.canvasWidth * dpr);
    canvas.height = Math.round(layout.canvasHeight * dpr);
    canvas.style.width = `${layout.canvasWidth}px`;
    canvas.style.height = `${layout.canvasHeight}px`;
    ctx.setTransform(dpr, 0, 0, dpr, 0, 0);

    ctx.clearRect(0, 0, layout.canvasWidth, layout.canvasHeight);
    ctx.fillStyle = '#f0d9b5';
    ctx.fillRect(0, 0, layout.canvasWidth, layout.canvasHeight);

    drawGrid(ctx, layout, rows, cols, gameType);

    if (gameType === 'xiangqi') {
      drawXiangqiSpecialMarks(ctx, layout);
    } else {
      drawJunqiSpecialMarks(ctx, layout);
    }

    drawLegalMoves(ctx, layout, legalMoves);

    if (selectedPiece) {
      drawSelectedPiece(ctx, layout, selectedPiece);
    }

    drawPieces(ctx, layout, boardState);
  }, [boardState, selectedPiece, legalMoves, gameType, rows, cols, layout]);

  const handleCanvasClick = (event: React.MouseEvent<HTMLCanvasElement>) => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const rect = canvas.getBoundingClientRect();
    if (rect.width === 0 || rect.height === 0) return;

    const x = ((event.clientX - rect.left) / rect.width) * layout.canvasWidth;
    const y = ((event.clientY - rect.top) / rect.height) * layout.canvasHeight;

    const col = Math.round((x - layout.padding) / layout.cellSize);
    const row = Math.round((y - layout.padding) / layout.cellSize);

    if (row >= 0 && row < rows && col >= 0 && col < cols) {
      onCellClick({ row, col });
    }
  };

  return (
    <div className="board-renderer" ref={wrapperRef}>
      <div className="board-canvas-wrapper">
        <canvas
          ref={canvasRef}
          width={layout.canvasWidth}
          height={layout.canvasHeight}
          onClick={handleCanvasClick}
          className="board-canvas"
        />
      </div>
    </div>
  );
};

function point(layout: BoardLayout, col: number, row: number) {
  return {
    x: layout.padding + col * layout.cellSize,
    y: layout.padding + row * layout.cellSize,
  };
}

function drawGrid(
  ctx: CanvasRenderingContext2D,
  layout: BoardLayout,
  rows: number,
  cols: number,
  gameType: GameType,
) {
  ctx.strokeStyle = '#000';
  ctx.lineWidth = 1.5;

  for (let i = 0; i < rows; i++) {
    const start = point(layout, 0, i);
    const end = point(layout, cols - 1, i);
    ctx.beginPath();
    ctx.moveTo(start.x, start.y);
    ctx.lineTo(end.x, end.y);
    ctx.stroke();
  }

  for (let i = 0; i < cols; i++) {
    if (gameType === 'xiangqi') {
      const top = point(layout, i, 0);
      const riverTop = point(layout, i, 4);
      ctx.beginPath();
      ctx.moveTo(top.x, top.y);
      ctx.lineTo(riverTop.x, riverTop.y);
      ctx.stroke();

      const riverBottom = point(layout, i, 5);
      const bottom = point(layout, i, rows - 1);
      ctx.beginPath();
      ctx.moveTo(riverBottom.x, riverBottom.y);
      ctx.lineTo(bottom.x, bottom.y);
      ctx.stroke();
    } else {
      const top = point(layout, i, 0);
      const bottom = point(layout, i, rows - 1);
      ctx.beginPath();
      ctx.moveTo(top.x, top.y);
      ctx.lineTo(bottom.x, bottom.y);
      ctx.stroke();
    }
  }
}

function drawXiangqiSpecialMarks(ctx: CanvasRenderingContext2D, layout: BoardLayout) {
  ctx.strokeStyle = '#000';
  ctx.lineWidth = 1.5;

  const drawPalace = (topRow: number) => {
    const a = point(layout, 3, topRow);
    const b = point(layout, 5, topRow + 2);
    ctx.beginPath();
    ctx.moveTo(a.x, a.y);
    ctx.lineTo(b.x, b.y);
    ctx.stroke();

    const c = point(layout, 5, topRow);
    const d = point(layout, 3, topRow + 2);
    ctx.beginPath();
    ctx.moveTo(c.x, c.y);
    ctx.lineTo(d.x, d.y);
    ctx.stroke();
  };

  drawPalace(0);
  drawPalace(7);

  ctx.font = `bold ${Math.max(12, Math.floor(layout.cellSize * 0.42))}px SimSun, serif`;
  ctx.fillStyle = '#000';
  ctx.textAlign = 'center';
  ctx.textBaseline = 'middle';

  const riverLeft = point(layout, 2, 4.5);
  const riverRight = point(layout, 6, 4.5);
  ctx.fillText('楚河', riverLeft.x, riverLeft.y);
  ctx.fillText('汉界', riverRight.x, riverRight.y);
}

function drawJunqiSpecialMarks(ctx: CanvasRenderingContext2D, layout: BoardLayout) {
  ctx.lineWidth = Math.max(2, Math.floor(layout.cellSize * 0.06));
  ctx.strokeStyle = '#666';

  [1, 5, 6, 10].forEach((row) => {
    const start = point(layout, 0, row);
    const end = point(layout, JUNQI_COLS - 1, row);
    ctx.beginPath();
    ctx.moveTo(start.x, start.y);
    ctx.lineTo(end.x, end.y);
    ctx.stroke();
  });

  [0, 4].forEach((col) => {
    const start = point(layout, col, 1);
    const end = point(layout, col, 10);
    ctx.beginPath();
    ctx.moveTo(start.x, start.y);
    ctx.lineTo(end.x, end.y);
    ctx.stroke();
  });

  ctx.fillStyle = 'rgba(100, 100, 100, 0.25)';
  ctx.strokeStyle = '#555';
  ctx.lineWidth = 1;

  const camps = [
    { row: 0, col: 0 }, { row: 0, col: 2 }, { row: 0, col: 4 },
    { row: 1, col: 1 }, { row: 1, col: 3 },
    { row: 10, col: 1 }, { row: 10, col: 3 },
    { row: 11, col: 0 }, { row: 11, col: 2 }, { row: 11, col: 4 },
  ];

  const campRadius = Math.max(4, Math.floor(layout.cellSize * 0.16));
  camps.forEach((camp) => {
    const { x, y } = point(layout, camp.col, camp.row);
    ctx.beginPath();
    ctx.arc(x, y, campRadius, 0, 2 * Math.PI);
    ctx.fill();
    ctx.stroke();
  });

  ctx.strokeStyle = '#888';
  ctx.lineWidth = 1;
  const triangle = (points: { col: number; row: number }[]) => {
    ctx.beginPath();
    points.forEach((p, index) => {
      const { x, y } = point(layout, p.col, p.row);
      if (index === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    });
    ctx.closePath();
    ctx.stroke();
  };

  triangle([
    { col: 0, row: 1 }, { col: 1, row: 0 }, { col: 2, row: 1 }, { col: 1, row: 1 },
  ]);
  triangle([
    { col: 2, row: 0 }, { col: 3, row: 1 }, { col: 4, row: 0 }, { col: 3, row: 1 },
  ]);
  triangle([
    { col: 0, row: 10 }, { col: 1, row: 11 }, { col: 2, row: 10 }, { col: 1, row: 10 },
  ]);
  triangle([
    { col: 2, row: 11 }, { col: 3, row: 10 }, { col: 4, row: 11 }, { col: 3, row: 10 },
  ]);
}

function drawLegalMoves(ctx: CanvasRenderingContext2D, layout: BoardLayout, moves: Position[]) {
  ctx.fillStyle = 'rgba(0, 255, 0, 0.3)';
  const radius = Math.max(5, Math.floor(layout.cellSize * 0.22));
  moves.forEach((pos) => {
    const { x, y } = point(layout, pos.col, pos.row);
    ctx.beginPath();
    ctx.arc(x, y, radius, 0, 2 * Math.PI);
    ctx.fill();
  });
}

function drawSelectedPiece(ctx: CanvasRenderingContext2D, layout: BoardLayout, pos: Position) {
  const { x, y } = point(layout, pos.col, pos.row);
  ctx.strokeStyle = '#ff0';
  ctx.lineWidth = 2.5;
  ctx.beginPath();
  ctx.arc(x, y, layout.pieceRadius + 4, 0, 2 * Math.PI);
  ctx.stroke();
}

function drawPieces(ctx: CanvasRenderingContext2D, layout: BoardLayout, board: BoardState) {
  Object.entries(board.pieces).forEach(([key, piece]) => {
    const [row, col] = key.split(',').map(Number);
    const { x, y } = point(layout, col, row);

    ctx.fillStyle = piece.player === 'Red' ? '#ff6b6b' : '#4a4a4a';
    ctx.beginPath();
    ctx.arc(x, y, layout.pieceRadius, 0, 2 * Math.PI);
    ctx.fill();

    ctx.strokeStyle = piece.player === 'Red' ? '#c92a2a' : '#000';
    ctx.lineWidth = 2;
    ctx.stroke();

    ctx.fillStyle = '#fff';
    ctx.font = `bold ${Math.max(10, Math.floor(layout.cellSize * 0.34))}px SimSun, serif`;
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    ctx.fillText(getPieceText(piece), x, y);
  });
}

function getPieceText(piece: { piece_type: Record<string, string>; player: string }): string {
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
  }
  if ('Junqi' in piece.piece_type) {
    switch (piece.piece_type.Junqi) {
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
}

export default BoardRenderer;
