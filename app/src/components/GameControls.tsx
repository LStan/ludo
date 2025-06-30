import React, { useState } from 'react';
import type { Game } from '../types/game';
import { Colors } from '../types/game';
import { COLOR_CONFIG } from '../utils/constants';

interface GameControlsProps {
  game: Game | null;
  onCreateGame: (numPlayers: number, color: Colors) => void;
  onJoinGame: (color: Colors) => void;
  onJoinAndStartGame: (color: Colors) => void;
  onCancelGame: () => void;
  onResetGame: () => void;
  isLoading: boolean;
}

export const GameControls: React.FC<GameControlsProps> = ({
  game,
  onCreateGame,
  onJoinGame,
  onJoinAndStartGame,
  onCancelGame,
  onResetGame,
  isLoading,
}) => {
  const [numPlayers, setNumPlayers] = useState(2);
  const [selectedColor, setSelectedColor] = useState<Colors>(Colors.Red);

  const availableColors = Object.values(Colors).filter((color: Colors) => {
    if (!game) return true;
    return !game.players[color] || game.players[color] === '11111111111111111111111111111111';
  });

  const canJoin = game && game.gameState === 'NotStarted';
  const canStart = game && game.curPlayer + 1 === game.numPlayers;
  const canCancel = game && game.gameState === 'NotStarted';

  return (
    <div className="card">
      <h3 style={{ 
        margin: '0 0 var(--spacing-md) 0', 
        fontSize: 'var(--font-size-lg)',
        color: 'var(--color-text)'
      }}>
        Game Controls
      </h3>

      {!game ? (
        <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--spacing-md)' }}>
          <div>
            <label style={{ 
              display: 'block', 
              marginBottom: 'var(--spacing-xs)',
              fontSize: 'var(--font-size-sm)',
              fontWeight: '500',
              color: 'var(--color-text)'
            }}>
              Number of Players
            </label>
            <select
              value={numPlayers}
              onChange={(e) => setNumPlayers(Number(e.target.value))}
              style={{
                width: '100%',
                padding: 'var(--spacing-sm)',
                border: '1px solid var(--color-border)',
                borderRadius: 'var(--radius-md)',
                fontSize: 'var(--font-size-sm)',
                backgroundColor: 'var(--color-surface)',
                color: 'var(--color-text)'
              }}
            >
              <option value={2}>2 Players</option>
              <option value={3}>3 Players</option>
              <option value={4}>4 Players</option>
            </select>
          </div>

          <div>
            <label style={{ 
              display: 'block', 
              marginBottom: 'var(--spacing-xs)',
              fontSize: 'var(--font-size-sm)',
              fontWeight: '500',
              color: 'var(--color-text)'
            }}>
              Choose Your Color
            </label>
            <div style={{ display: 'flex', gap: 'var(--spacing-sm)', flexWrap: 'wrap' }}>
              {availableColors.map((color) => (
                <button
                  key={color}
                  onClick={() => setSelectedColor(color)}
                  style={{
                    width: '60px',
                    height: '60px',
                    borderRadius: 'var(--radius-full)',
                    border: selectedColor === color ? '3px solid var(--color-primary)' : '2px solid var(--color-border)',
                    backgroundColor: COLOR_CONFIG[color].hex,
                    cursor: 'pointer',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    color: 'white',
                    fontWeight: 'bold',
                    fontSize: 'var(--font-size-sm)',
                    transition: 'all var(--transition-fast)'
                  }}
                >
                  {COLOR_CONFIG[color].name.charAt(0)}
                </button>
              ))}
            </div>
          </div>

          <button
            className="btn btn-primary"
            onClick={() => onCreateGame(numPlayers, selectedColor)}
            disabled={isLoading}
            style={{ width: '100%' }}
          >
            {isLoading ? 'Creating...' : 'Create Game'}
          </button>
        </div>
      ) : (
        <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--spacing-sm)' }}>
          {canJoin && (
            <div>
              <label style={{ 
                display: 'block', 
                marginBottom: 'var(--spacing-xs)',
                fontSize: 'var(--font-size-sm)',
                fontWeight: '500',
                color: 'var(--color-text)'
              }}>
                Join as Color
              </label>
              <div style={{ display: 'flex', gap: 'var(--spacing-sm)', flexWrap: 'wrap', marginBottom: 'var(--spacing-sm)' }}>
                {availableColors.map((color) => (
                  <button
                    key={color}
                    onClick={() => canStart ? onJoinAndStartGame(color) : onJoinGame(color)}
                    disabled={isLoading}
                    style={{
                      width: '50px',
                      height: '50px',
                      borderRadius: 'var(--radius-full)',
                      border: '2px solid var(--color-border)',
                      backgroundColor: COLOR_CONFIG[color].hex,
                      cursor: isLoading ? 'not-allowed' : 'pointer',
                      display: 'flex',
                      alignItems: 'center',
                      justifyContent: 'center',
                      color: 'white',
                      fontWeight: 'bold',
                      fontSize: 'var(--font-size-xs)',
                      opacity: isLoading ? 0.5 : 1
                    }}
                  >
                    {COLOR_CONFIG[color].name.charAt(0)}
                  </button>
                ))}
              </div>
              <button
                className={`btn ${canStart ? 'btn-success' : 'btn-secondary'}`}
                onClick={() => canStart ? onJoinAndStartGame(selectedColor) : onJoinGame(selectedColor)}
                disabled={isLoading || availableColors.length === 0}
                style={{ width: '100%' }}
              >
                {isLoading ? 'Joining...' : canStart ? 'Join & Start Game' : 'Join Game'}
              </button>
            </div>
          )}

          {canCancel && (
            <button
              className="btn btn-outline"
              onClick={onCancelGame}
              disabled={isLoading}
              style={{ width: '100%' }}
            >
              {isLoading ? 'Canceling...' : 'Cancel Game'}
            </button>
          )}

          <button
            className="btn btn-outline"
            onClick={onResetGame}
            disabled={isLoading}
            style={{ width: '100%' }}
          >
            New Game
          </button>
        </div>
      )}
    </div>
  );
}; 