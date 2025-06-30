export enum GameState {
  NotStarted = 'NotStarted',
  Starting = 'Starting',
  RollDice = 'RollDice',
  RollingDice = 'RollingDice',
  Move = 'Move',
  Finished = 'Finished',
}

export enum Colors {
  Red = 0,
  Green = 1,
  Yellow = 2,
  Blue = 3,
}

export interface Game {
  seed: number;
  bump: number;
  numPlayers: number;
  curPlayer: number;
  tokenPositions: number[][]; // [player][token] -> position
  gameState: GameState;
  currentRoll: number;
  sixCount: number;
  players: string[]; // Public keys
  winner: string;
}

export interface Player {
  publicKey: string;
  color: Colors;
  isCurrentPlayer: boolean;
}

export interface GameAction {
  type: 'create' | 'join' | 'roll' | 'move' | 'tokenIntoPlay' | 'cancel';
  payload?: Record<string, unknown>;
}

export interface DiceRoll {
  value: number;
  isRolling: boolean;
}

export interface Token {
  id: number;
  position: number;
  color: Colors;
  isInPlay: boolean;
  isSelected: boolean;
} 