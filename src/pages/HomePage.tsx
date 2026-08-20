import { useNavigate } from 'react-router-dom';
import GameSelector, { type GameType } from '../components/GameSelector';

/**
 * 主菜单：选择棋种后进入对局设置页
 */
export default function HomePage() {
  const navigate = useNavigate();

  const handleGameSelect = (gameType: GameType) => {
    navigate(`/setup/${gameType}`);
  };

  return <GameSelector onGameSelect={handleGameSelect} />;
}
