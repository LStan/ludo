import React from 'react';
import type { Game } from '../types/game';
import { COLOR_CONFIG } from '../utils/constants';
import { formatPublicKey, getGameStatusMessage } from '../utils/helpers';

interface PlayerInfoProps {
  game: Game | null;
}

export const PlayerInfo: React.FC<PlayerInfoProps> = ({ game }) => {
  if (!game) {
    return (
      <div className="card">
        <h2 style={{ margin: '0 0 var(--spacing-md) 0', color: 'var(--color-text)' }}>
          Ludo Game
        </h2>
        <p style={{ color: 'var(--color-text-secondary)', margin: 0 }}>
          Create or join a game to get started
        </p>
      </div>
    );
  }

  const getPlayerColor = (playerIndex: number) => {
    return COLOR_CONFIG[playerIndex as keyof typeof COLOR_CONFIG];
  };

  return (
    <div className="card">
      <div style={{ 
        display: 'flex', 
        justifyContent: 'space-between', 
        alignItems: 'center',
        marginBottom: 'var(--spacing-md)'
      }}>
        <h2 style={{ margin: 0, color: 'var(--color-text)' }}>
          Ludo Game
        </h2>
        <div style={{ 
          padding: 'var(--spacing-xs) var(--spacing-sm)',
          backgroundColor: 'var(--color-primary)',
          color: 'white',
          borderRadius: 'var(--radius-md)',
          fontSize: 'var(--font-size-sm)',
          fontWeight: '500'
        }}>
          {game.numPlayers} Players
        </div>
      </div>

      <div className="status-message">
        {getGameStatusMessage(game.gameState)}
        {game.currentRoll > 0 && (
          <span style={{ marginLeft: 'var(--spacing-sm)' }}>
            Roll: {game.currentRoll}
          </span>
        )}
      </div>

      <div style={{ marginTop: 'var(--spacing-md)' }}>
        <h3 style={{ 
          margin: '0 0 var(--spacing-sm) 0', 
          fontSize: 'var(--font-size-lg)',
          color: 'var(--color-text)'
        }}>
          Players
        </h3>
        <div style={{ display: 'flex', flexDirection: 'column', gap: 'var(--spacing-sm)' }}>
          {game.players.map((playerKey, index) => {
            const color = getPlayerColor(index);
            const isCurrentPlayer = game.curPlayer === index;
            const hasJoined = playerKey && playerKey !== '11111111111111111111111111111111';
            
            return (
              <div 
                key={index}
                className={`player-info ${isCurrentPlayer ? 'player-current' : ''}`}
                style={{
                  opacity: hasJoined ? 1 : 0.5,
                  borderColor: isCurrentPlayer ? 'var(--color-primary)' : 'var(--color-border)',
                }}
              >
                <div 
                  className="player-avatar"
                  style={{ 
                    backgroundColor: color.hex,
                    border: isCurrentPlayer ? '2px solid var(--color-primary)' : 'none'
                  }}
                >
                  {color.name.charAt(0)}
                </div>
                <div style={{ flex: 1 }}>
                  <div style={{ 
                    fontWeight: '500', 
                    color: 'var(--color-text)',
                    fontSize: 'var(--font-size-sm)'
                  }}>
                    {color.name} Player
                  </div>
                  <div style={{ 
                    color: 'var(--color-text-secondary)',
                    fontSize: 'var(--font-size-xs)'
                  }}>
                    {formatPublicKey(playerKey)}
                  </div>
                </div>
                {isCurrentPlayer && (
                  <div style={{ 
                    color: 'var(--color-primary)',
                    fontWeight: '500',
                    fontSize: 'var(--font-size-sm)'
                  }}>
                    Current Turn
                  </div>
                )}
              </div>
            );
          })}
        </div>
      </div>

      {game.winner && (
        <div style={{ 
          marginTop: 'var(--spacing-md)',
          padding: 'var(--spacing-md)',
          backgroundColor: 'var(--color-success)',
          color: 'white',
          borderRadius: 'var(--radius-md)',
          textAlign: 'center',
          fontWeight: '500'
        }}>
          🎉 Game won by {formatPublicKey(game.winner)}! 🎉
        </div>
      )}
    </div>
  );
}; 