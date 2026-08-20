import { Navigate, useNavigate, useParams, useSearchParams } from 'react-router-dom';
import GameBoard from '../components/GameBoard';
import type { Difficulty, Player } from '../types';
import { isGameType } from '../types';

function parseSide(value: string | null): Player {
  return value === 'Black' ? 'Black' : 'Red';
}

function parseDifficulty(value: string | null): Difficulty {
  if (value === 'Easy' || value === 'Hard' || value === 'Medium') {
    return value;
  }
  return 'Medium';
}

/**
 * 对局页：从查询参数读取执子与难度，交给 GameBoard
 */
export default function PlayPage() {
  const { gameType: rawType } = useParams<{ gameType: string }>();
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();

  if (!rawType || !isGameType(rawType)) {
    return <Navigate to="/" replace />;
  }

  const playerSide = parseSide(searchParams.get('side'));
  const difficulty = parseDifficulty(searchParams.get('difficulty'));

  return (
    <div className="play-page">
      <GameBoard
        gameType={rawType}
        playerSide={playerSide}
        difficulty={difficulty}
        onBackToMenu={() => navigate('/')}
        onBackToSetup={() => navigate(`/setup/${rawType}`)}
      />
    </div>
  );
}
