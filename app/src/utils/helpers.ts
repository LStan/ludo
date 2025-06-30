import { Colors, GameState } from '../types/game';
import { SAFE_POSITIONS } from './constants';

// Generate a random seed for game creation
export const generateGameSeed = (): number => {
  return Math.floor(Math.random() * Number.MAX_SAFE_INTEGER);
};

// Generate a random client seed for VRF
export const generateClientSeed = (): number => {
  return Math.floor(Math.random() * 256);
};

// Check if a position is safe (cannot be captured)
export const isSafePosition = (position: number): boolean => {
  return SAFE_POSITIONS.includes(position);
};

// Get color name from enum
export const getColorName = (color: Colors): string => {
  const colorNames = ['Red', 'Green', 'Yellow', 'Blue'];
  return colorNames[color] || 'Unknown';
};

// Check if a player can make a move
export const canPlayerMove = (
  gameState: GameState,
  currentRoll: number,
  tokenPositions: number[],
  sixCount: number
): boolean => {
  if (gameState !== GameState.Move) return false;
  
  // If three 6s in a row, no moves possible
  if (sixCount === 2 && currentRoll === 6) return false;
  
  // Check if any token can move
  for (const position of tokenPositions) {
    if (position === -1 && currentRoll === 6) return true; // Can bring token into play
    if (position !== -1 && position + currentRoll <= 56) return true; // Can move token
  }
  
  return false;
};

// Get available moves for current player
export const getAvailableMoves = (
  tokenPositions: number[],
  currentRoll: number,
  sixCount: number
): number[] => {
  const moves: number[] = [];
  
  if (sixCount === 2 && currentRoll === 6) return moves;
  
  tokenPositions.forEach((position, index) => {
    if (position === -1 && currentRoll === 6) {
      moves.push(index); // Can bring token into play
    } else if (position !== -1 && position + currentRoll <= 56) {
      moves.push(index); // Can move token
    }
  });
  
  return moves;
};

// Format public key for display
export const formatPublicKey = (publicKey: string): string => {
  if (!publicKey || publicKey === '11111111111111111111111111111111') {
    return 'Not joined';
  }
  return `${publicKey.slice(0, 4)}...${publicKey.slice(-4)}`;
};

// Get game status message
export const getGameStatusMessage = (gameState: GameState): string => {
  switch (gameState) {
    case GameState.NotStarted:
      return 'Waiting for players to join...';
    case GameState.Starting:
      return 'Starting game...';
    case GameState.RollDice:
      return 'Roll the dice!';
    case GameState.RollingDice:
      return 'Rolling dice...';
    case GameState.Move:
      return 'Choose a token to move';
    case GameState.Finished:
      return 'Game finished!';
    default:
      return 'Unknown state';
  }
}; 