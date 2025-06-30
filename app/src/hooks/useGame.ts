import { useState, useCallback } from 'react';
import type { Game, Player } from '../types/game';
import { GameState, Colors } from '../types/game';
import { generateGameSeed, generateClientSeed, getGameStatusMessage } from '../utils/helpers';

interface UseGameReturn {
  game: Game | null;
  currentPlayer: Player | null;
  isLoading: boolean;
  error: string | null;
  createGame: (numPlayers: number, color: Colors) => void;
  joinGame: (color: Colors) => void;
  joinAndStartGame: (color: Colors) => void;
  rollDice: () => void;
  makeMove: (tokenNum: number) => void;
  tokenIntoPlay: (tokenNum: number) => void;
  cancelGame: (color: Colors) => void;
  resetGame: () => void;
  getStatusMessage: () => string;
}

export const useGame = (): UseGameReturn => {
  const [game, setGame] = useState<Game | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const currentPlayer = game ? {
    publicKey: game.players[game.curPlayer] || '',
    color: game.curPlayer as Colors,
    isCurrentPlayer: true,
  } : null;

  const createGame = useCallback(async (numPlayers: number, color: Colors) => {
    setIsLoading(true);
    setError(null);
    
    try {
      // TODO: Implement Solana transaction
      const seed = generateGameSeed();
      const newGame: Game = {
        seed,
        bump: 0, // Will be set by Solana
        numPlayers,
        curPlayer: 1,
        tokenPositions: [[-1, -1, -1, -1], [-1, -1, -1, -1], [-1, -1, -1, -1], [-1, -1, -1, -1]],
        gameState: GameState.NotStarted,
        currentRoll: 0,
        sixCount: 0,
        players: ['', '', '', ''],
        winner: '',
      };
      
      // Set the current player
      newGame.players[color] = 'current-player-key'; // TODO: Get actual player key
      
      setGame(newGame);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to create game');
    } finally {
      setIsLoading(false);
    }
  }, []);

  const joinGame = useCallback(async (color: Colors) => {
    if (!game) return;
    
    setIsLoading(true);
    setError(null);
    
    try {
      // TODO: Implement Solana transaction
      const updatedGame = { ...game };
      updatedGame.players[color] = 'current-player-key'; // TODO: Get actual player key
      updatedGame.curPlayer += 1;
      
      setGame(updatedGame);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to join game');
    } finally {
      setIsLoading(false);
    }
  }, [game]);

  const joinAndStartGame = useCallback(async (color: Colors) => {
    if (!game) return;
    
    setIsLoading(true);
    setError(null);
    
    try {
      // TODO: Implement Solana transaction
      generateClientSeed(); // Generate seed for VRF
      const updatedGame = { ...game };
      updatedGame.players[color] = 'current-player-key'; // TODO: Get actual player key
      updatedGame.curPlayer = 0;
      updatedGame.gameState = GameState.Starting;
      
      setGame(updatedGame);
      
      // Simulate VRF callback
      setTimeout(() => {
        setGame(prev => prev ? {
          ...prev,
          gameState: GameState.RollDice,
          curPlayer: Math.floor(Math.random() * prev.numPlayers),
        } : null);
      }, 2000);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to start game');
    } finally {
      setIsLoading(false);
    }
  }, [game]);

  const rollDice = useCallback(async () => {
    if (!game || game.gameState !== GameState.RollDice) return;
    
    setIsLoading(true);
    setError(null);
    
    try {
      // TODO: Implement Solana transaction
      generateClientSeed(); // Generate seed for VRF
      const updatedGame = { ...game };
      updatedGame.gameState = GameState.RollingDice;
      
      setGame(updatedGame);
      
      // Simulate VRF callback
      setTimeout(() => {
        const roll = Math.floor(Math.random() * 6) + 1;
        setGame(prev => prev ? {
          ...prev,
          gameState: GameState.Move,
          currentRoll: roll,
        } : null);
      }, 1500);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to roll dice');
    } finally {
      setIsLoading(false);
    }
  }, [game]);

  const makeMove = useCallback(async (tokenNum: number) => {
    if (!game || game.gameState !== GameState.Move) return;
    
    setIsLoading(true);
    setError(null);
    
    try {
      // TODO: Implement Solana transaction
      const updatedGame = { ...game };
      const currentPositions = updatedGame.tokenPositions[game.curPlayer];
      const newPosition = currentPositions[tokenNum] + game.currentRoll;
      
      if (newPosition <= 56) {
        updatedGame.tokenPositions[game.curPlayer][tokenNum] = newPosition;
        
        // Check for win
        if (updatedGame.tokenPositions[game.curPlayer].every(pos => pos === 56)) {
          updatedGame.gameState = GameState.Finished;
          updatedGame.winner = 'current-player-key'; // TODO: Get actual player key
        } else {
          // Handle turn logic
          if (game.currentRoll === 6) {
            updatedGame.sixCount += 1;
          } else {
            updatedGame.sixCount = 0;
            // TODO: Implement next player logic
          }
          updatedGame.gameState = GameState.RollDice;
        }
        
        setGame(updatedGame);
      }
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to make move');
    } finally {
      setIsLoading(false);
    }
  }, [game]);

  const tokenIntoPlay = useCallback(async (tokenNum: number) => {
    if (!game || game.gameState !== GameState.Move || game.currentRoll !== 6) return;
    
    setIsLoading(true);
    setError(null);
    
    try {
      // TODO: Implement Solana transaction
      const updatedGame = { ...game };
      updatedGame.tokenPositions[game.curPlayer][tokenNum] = 0;
      updatedGame.gameState = GameState.RollDice;
      
      setGame(updatedGame);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to bring token into play');
    } finally {
      setIsLoading(false);
    }
  }, [game]);

  const cancelGame = useCallback(async () => {
    if (!game || game.gameState !== GameState.NotStarted) return;
    
    setIsLoading(true);
    setError(null);
    
    try {
      // TODO: Implement Solana transaction
      setGame(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to cancel game');
    } finally {
      setIsLoading(false);
    }
  }, [game]);

  const resetGame = useCallback(() => {
    setGame(null);
    setError(null);
    setIsLoading(false);
  }, []);

  const getStatusMessage = useCallback(() => {
    if (!game) return 'No game in progress';
    return getGameStatusMessage(game.gameState);
  }, [game]);

  return {
    game,
    currentPlayer,
    isLoading,
    error,
    createGame,
    joinGame,
    joinAndStartGame,
    rollDice,
    makeMove,
    tokenIntoPlay,
    cancelGame,
    resetGame,
    getStatusMessage,
  };
}; 