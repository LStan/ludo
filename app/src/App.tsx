import React, { useState } from 'react';
import { useGame } from './hooks/useGame';
import { useSolana } from './hooks/useSolana';
import { GameBoard } from './components/GameBoard';
import { PlayerInfo } from './components/PlayerInfo';
import { DiceRoller } from './components/DiceRoller';
import { Colors, GameState } from './types/game';
import './styles/components.css';

function App() {
  const { wallet, isConnected, isConnecting, connect, disconnect } = useSolana();
  const {
    game,
    isLoading,
    error,
    createGame,
    rollDice,
    makeMove,
    tokenIntoPlay,
    resetGame,
    getStatusMessage,
  } = useGame();

  const [selectedToken, setSelectedToken] = useState<{ playerIndex: number; tokenIndex: number } | null>(null);

  const handleTokenClick = (playerIndex: number, tokenIndex: number) => {
    if (game?.gameState === GameState.Move) {
      setSelectedToken({ playerIndex, tokenIndex });
      
      // Check if this is a valid move
      const tokenPosition = game.tokenPositions[playerIndex][tokenIndex];
      const currentRoll = game.currentRoll;
      
      if (tokenPosition === -1 && currentRoll === 6) {
        // Bring token into play
        tokenIntoPlay(tokenIndex);
      } else if (tokenPosition !== -1 && tokenPosition + currentRoll <= 56) {
        // Move token
        makeMove(tokenIndex);
      }
      
      setSelectedToken(null);
    }
  };

  const handleCreateGame = (numPlayers: number, color: Colors) => {
    createGame(numPlayers, color);
  };

  const handleCancelGame = () => {
    // For now, just reset the game since we don't have the color context
    resetGame();
  };

  return (
    <div style={{ 
      minHeight: '100vh',
      backgroundColor: 'var(--color-background)',
      padding: 'var(--spacing-lg)',
      fontFamily: 'var(--font-family)'
    }}>
      <div style={{ 
        maxWidth: '1200px', 
        margin: '0 auto',
        display: 'flex',
        flexDirection: 'column',
        gap: 'var(--spacing-lg)'
      }}>
        {/* Header */}
        <header style={{ 
          textAlign: 'center',
          marginBottom: 'var(--spacing-lg)'
        }}>
          <h1 style={{ 
            margin: '0 0 var(--spacing-sm) 0',
            color: 'var(--color-text)',
            fontSize: 'var(--font-size-3xl)',
            fontWeight: 'bold'
          }}>
            🎲 Ludo Game
          </h1>
          <p style={{ 
            color: 'var(--color-text-secondary)',
            margin: 0,
            fontSize: 'var(--font-size-lg)'
          }}>
            A decentralized Ludo game on Solana
          </p>
        </header>

        {/* Wallet Connection */}
        <div style={{ 
          display: 'flex', 
          justifyContent: 'center',
          marginBottom: 'var(--spacing-lg)'
        }}>
          {!isConnected ? (
            <button
              className="btn btn-primary"
              onClick={connect}
              disabled={isConnecting}
            >
              {isConnecting ? 'Connecting...' : 'Connect Wallet'}
            </button>
          ) : (
            <div style={{ 
              display: 'flex', 
              alignItems: 'center', 
              gap: 'var(--spacing-md)'
            }}>
              <span style={{ 
                color: 'var(--color-text-secondary)',
                fontSize: 'var(--font-size-sm)'
              }}>
                Connected: {wallet?.publicKey.slice(0, 8)}...
              </span>
              <button
                className="btn btn-outline"
                onClick={disconnect}
              >
                Disconnect
              </button>
            </div>
          )}
        </div>

        {/* Error Display */}
        {error && (
          <div style={{ 
            padding: 'var(--spacing-md)',
            backgroundColor: 'var(--color-error)',
            color: 'white',
            borderRadius: 'var(--radius-md)',
            textAlign: 'center',
            marginBottom: 'var(--spacing-lg)'
          }}>
            {error}
          </div>
        )}

        {/* Main Game Layout */}
        <div style={{ 
          display: 'grid',
          gridTemplateColumns: '1fr 2fr 1fr',
          gap: 'var(--spacing-lg)',
          alignItems: 'start'
        }}>
          {/* Left Sidebar - Game Controls & Player Info */}
          <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--spacing-md)' }}>
            <PlayerInfo game={game} />
            
            {/* Simple Game Controls */}
            <div className="card">
              <h3 style={{ 
                margin: '0 0 var(--spacing-md) 0', 
                fontSize: 'var(--font-size-lg)',
                color: 'var(--color-text)'
              }}>
                Game Controls
              </h3>
              
              {!game ? (
                <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--spacing-sm)' }}>
                  <button
                    className="btn btn-primary"
                    onClick={() => handleCreateGame(2, Colors.Red)}
                    disabled={isLoading || !isConnected}
                  >
                    {isLoading ? 'Creating...' : 'Create 2-Player Game'}
                  </button>
                  <button
                    className="btn btn-secondary"
                    onClick={() => handleCreateGame(4, Colors.Red)}
                    disabled={isLoading || !isConnected}
                  >
                    {isLoading ? 'Creating...' : 'Create 4-Player Game'}
                  </button>
                </div>
              ) : (
                <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--spacing-sm)' }}>
                  <button
                    className="btn btn-outline"
                    onClick={handleCancelGame}
                    disabled={isLoading}
                  >
                    {isLoading ? 'Canceling...' : 'Cancel Game'}
                  </button>
                  <button
                    className="btn btn-outline"
                    onClick={resetGame}
                    disabled={isLoading}
                  >
                    New Game
                  </button>
                </div>
              )}
            </div>
          </div>

          {/* Center - Game Board */}
          <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center' }}>
            <GameBoard 
              game={game} 
              onTokenClick={handleTokenClick}
              selectedToken={selectedToken}
            />
          </div>

          {/* Right Sidebar - Dice & Game Status */}
          <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--spacing-md)' }}>
            <DiceRoller
              currentRoll={game?.currentRoll || 0}
              gameState={game?.gameState || GameState.NotStarted}
              onRoll={rollDice}
              isLoading={isLoading}
            />
            
            {/* Game Status */}
            <div className="card">
              <h3 style={{ 
                margin: '0 0 var(--spacing-md) 0', 
                fontSize: 'var(--font-size-lg)',
                color: 'var(--color-text)'
              }}>
                Game Status
              </h3>
              <p style={{ 
                color: 'var(--color-text-secondary)',
                margin: 0,
                fontSize: 'var(--font-size-sm)'
              }}>
                {getStatusMessage()}
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
