import { useState } from "react";
import "./App.css";
import GameSelector, { GameType } from "./components/GameSelector";
import GameBoard from "./components/GameBoard";

function App() {
  const [selectedGame, setSelectedGame] = useState<GameType | null>(null);

  const handleGameSelect = (gameType: GameType) => {
    setSelectedGame(gameType);
  };

  const handleBackToMenu = () => {
    setSelectedGame(null);
  };

  return (
    <div className="app">
      {!selectedGame ? (
        <GameSelector onGameSelect={handleGameSelect} />
      ) : (
        <GameBoard gameType={selectedGame} onBackToMenu={handleBackToMenu} />
      )}
    </div>
  );
}

export default App;
