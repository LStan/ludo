import React from 'react';
import type { Game, Colors } from '../types/game';
import { COLOR_CONFIG } from '../utils/constants';

interface GameBoardProps {
  game: Game | null;
  onTokenClick?: (playerIndex: number, tokenIndex: number) => void;
  selectedToken?: { playerIndex: number; tokenIndex: number } | null;
}

export const GameBoard: React.FC<GameBoardProps> = ({ 
  game, 
  onTokenClick, 
  selectedToken 
}) => {
  if (!game) {
    return (
      <div className="game-board">
        <div style={{ 
          gridColumn: '1 / -1', 
          gridRow: '1 / -1',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
          color: 'var(--color-text-secondary)',
          fontSize: 'var(--font-size-lg)'
        }}>
          No game in progress
        </div>
      </div>
    );
  }

  const renderCell = (row: number, col: number) => {
    const position = row * 15 + col;
    const tokens: Array<{ playerIndex: number; tokenIndex: number; color: Colors }> = [];

    // Find tokens at this position
    game.tokenPositions.forEach((playerTokens, playerIndex) => {
      playerTokens.forEach((tokenPosition, tokenIndex) => {
        if (tokenPosition === position) {
          tokens.push({ playerIndex, tokenIndex, color: playerIndex as Colors });
        }
      });
    });

    const isSelected = selectedToken && 
      tokens.some(token => 
        token.playerIndex === selectedToken.playerIndex && 
        token.tokenIndex === selectedToken.tokenIndex
      );

    return (
      <div 
        key={`${row}-${col}`}
        className="board-cell"
        style={{
          backgroundColor: tokens.length > 0 ? COLOR_CONFIG[tokens[0].color].lightHex : 'var(--color-background)',
          border: isSelected ? '2px solid var(--color-primary)' : '1px solid var(--color-border)',
        }}
        onClick={() => {
          if (tokens.length > 0 && onTokenClick) {
            onTokenClick(tokens[0].playerIndex, tokens[0].tokenIndex);
          }
        }}
      >
        {tokens.map((token, index) => (
          <div
            key={`${token.playerIndex}-${token.tokenIndex}`}
            className={`token token-${COLOR_CONFIG[token.color].name.toLowerCase()}`}
            style={{
              transform: `scale(${0.8 - index * 0.1})`,
              zIndex: tokens.length - index,
            }}
          />
        ))}
      </div>
    );
  };

  return (
    <div className="game-board">
      {Array.from({ length: 15 }, (_, row) =>
        Array.from({ length: 15 }, (_, col) => renderCell(row, col))
      )}
    </div>
  );
}; 