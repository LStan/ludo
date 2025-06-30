import React from 'react';
import type { GameState } from '../types/game';

interface DiceRollerProps {
  currentRoll: number;
  gameState: GameState;
  onRoll: () => void;
  isLoading: boolean;
}

export const DiceRoller: React.FC<DiceRollerProps> = ({
  currentRoll,
  gameState,
  onRoll,
  isLoading,
}) => {
  const canRoll = gameState === 'RollDice' && !isLoading;

  return (
    <div className="card">
      <h3 style={{ 
        margin: '0 0 var(--spacing-md) 0', 
        fontSize: 'var(--font-size-lg)',
        color: 'var(--color-text)',
        textAlign: 'center'
      }}>
        Dice
      </h3>
      
      <div style={{ 
        display: 'flex', 
        flexDirection: 'column', 
        alignItems: 'center',
        gap: 'var(--spacing-md)'
      }}>
        <div 
          className={`dice ${gameState === 'RollingDice' ? 'rolling' : ''}`}
          onClick={canRoll ? onRoll : undefined}
          style={{
            cursor: canRoll ? 'pointer' : 'default',
            opacity: canRoll ? 1 : 0.7,
          }}
        >
          {currentRoll > 0 ? currentRoll : '?'}
        </div>
        
        <button
          className="btn btn-primary"
          onClick={onRoll}
          disabled={!canRoll}
          style={{ minWidth: '120px' }}
        >
          {isLoading ? 'Rolling...' : 'Roll Dice'}
        </button>
        
        {gameState === 'RollingDice' && (
          <p style={{ 
            color: 'var(--color-text-secondary)',
            fontSize: 'var(--font-size-sm)',
            margin: 0,
            textAlign: 'center'
          }}>
            Waiting for random number...
          </p>
        )}
      </div>
    </div>
  );
}; 