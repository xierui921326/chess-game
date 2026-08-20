import { useMemo, useState } from 'react';
import { Link, Navigate, useNavigate, useParams } from 'react-router-dom';
import type { Difficulty, Player } from '../types';
import { DIFFICULTY_LABELS, isGameType } from '../types';
import './SetupPage.css';

const DIFFICULTIES: Difficulty[] = ['Easy', 'Medium', 'Hard'];

/**
 * 对局设置：先手颜色 + AI 难度，确认后进入对局页
 */
export default function SetupPage() {
  const { gameType: rawType } = useParams<{ gameType: string }>();
  const navigate = useNavigate();
  const [playerSide, setPlayerSide] = useState<Player>('Red');
  const [difficulty, setDifficulty] = useState<Difficulty>('Medium');

  const gameType = useMemo(
    () => (rawType && isGameType(rawType) ? rawType : null),
    [rawType],
  );

  if (!gameType) {
    return <Navigate to="/" replace />;
  }

  const title = gameType === 'xiangqi' ? '中国象棋' : '军棋（翻棋）';
  const subtitle =
    gameType === 'xiangqi'
      ? '选择执子颜色与 AI 难度后开始对局'
      : '二人暗棋翻棋：选择执子与难度后开局洗牌';

  const handleStart = () => {
    const params = new URLSearchParams({
      side: playerSide,
      difficulty,
    });
    navigate(`/play/${gameType}?${params.toString()}`);
  };

  return (
    <div className="setup-page">
      <div className="setup-card">
        <Link to="/" className="setup-back">
          ← 返回主菜单
        </Link>

        <h1 className="setup-title">{title}</h1>
        <p className="setup-subtitle">{subtitle}</p>

        <section className="setup-section" aria-labelledby="side-label">
          <h2 id="side-label">执子颜色</h2>
          <div className="setup-options" role="group" aria-label="执子颜色">
            <button
              type="button"
              className={`setup-option side-red ${playerSide === 'Red' ? 'active' : ''}`}
              onClick={() => setPlayerSide('Red')}
              aria-pressed={playerSide === 'Red'}
            >
              红方
              <span className="setup-option-hint">先手</span>
            </button>
            <button
              type="button"
              className={`setup-option side-black ${playerSide === 'Black' ? 'active' : ''}`}
              onClick={() => setPlayerSide('Black')}
              aria-pressed={playerSide === 'Black'}
            >
              黑方
              <span className="setup-option-hint">后手</span>
            </button>
          </div>
        </section>

        <section className="setup-section" aria-labelledby="diff-label">
          <h2 id="diff-label">AI 难度</h2>
          <div className="setup-options" role="group" aria-label="AI 难度">
            {DIFFICULTIES.map((level) => (
              <button
                key={level}
                type="button"
                className={`setup-option ${difficulty === level ? 'active' : ''}`}
                onClick={() => setDifficulty(level)}
                aria-pressed={difficulty === level}
              >
                {DIFFICULTY_LABELS[level]}
              </button>
            ))}
          </div>
        </section>

        <button type="button" className="setup-start" onClick={handleStart}>
          开始对局
        </button>
      </div>
    </div>
  );
}
