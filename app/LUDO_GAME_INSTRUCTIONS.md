# Ludo Game Frontend Instructions

## Overview
This is a simple Ludo game frontend built with React + TypeScript + Vite, designed to interact with a Solana program. The UI follows Gather.town aesthetics with a clean, modern design.

## Game Rules (Based on Solana Program)
- 2-4 players can join a game
- Each player has 4 tokens that start at position -1 (not in play)
- Players take turns rolling dice (1-6)
- Rolling a 6 allows you to bring a token into play (position 0)
- Rolling three 6s in a row skips your turn
- Tokens move around the board based on dice rolls
- Landing on another player's token sends it back to start (unless it's on a safe position)
- First player to get all 4 tokens to position 56 wins

## Solana Program Instructions

### 1. Create Game
- **Function**: `create_game`
- **Parameters**: `seed: u64`, `num_players: u8`, `color: Colors`
- **Description**: Creates a new game with specified number of players and color
- **Colors**: Red(0), Green(1), Yellow(2), Blue(3)

### 2. Join Game
- **Function**: `join_game`
- **Parameters**: `color: Colors`
- **Description**: Joins an existing game with a specific color

### 3. Join and Start Game
- **Function**: `join_and_start_game`
- **Parameters**: `color: Colors`, `client_seed: u8`
- **Description**: Joins as the last player and starts the game with random first player

### 4. Roll Dice
- **Function**: `roll_dice_delegate`
- **Parameters**: `client_seed: u8`
- **Description**: Rolls dice for current player using VRF (Verifiable Random Function)

### 5. Move Token
- **Function**: `make_move`
- **Parameters**: `token_num: u8`
- **Description**: Moves a specific token based on current dice roll

### 6. Bring Token Into Play
- **Function**: `token_into_play`
- **Parameters**: `token_num: u8`
- **Description**: Brings a token into play when rolling a 6

### 7. Cancel Game
- **Function**: `cancel_game`
- **Parameters**: `color: Colors`
- **Description**: Cancels a game that hasn't started yet

### 8. Delegate/Undelegate
- **Functions**: `delegate`, `undelegate`
- **Description**: Manages game state delegation for rollups

## Game States
- `NotStarted`: Game created but not started
- `Starting`: Game is starting (waiting for VRF)
- `RollDice`: Current player should roll dice
- `RollingDice`: Dice is being rolled (waiting for VRF)
- `Move`: Player should make a move
- `Finished`: Game is complete

## Frontend Structure

### Components
1. **GameBoard**: Main game board with visual representation
2. **PlayerInfo**: Shows current player and game status
3. **DiceRoller**: Interface for rolling dice
4. **TokenSelector**: Select which token to move
5. **GameControls**: Create, join, cancel game buttons
6. **PlayerList**: Shows all players and their colors

### Features
- Real-time game state updates
- Visual board with token positions
- Dice rolling animation
- Player turn indicators
- Game creation and joining
- Responsive design

### Styling (Gather.town Inspired)
- Clean, modern interface
- Soft colors and rounded corners
- Minimalist design
- Good contrast for readability
- Smooth animations
- Professional appearance

## File Structure
```
src/
├── components/
│   ├── GameBoard.tsx
│   ├── PlayerInfo.tsx
│   ├── DiceRoller.tsx
│   ├── TokenSelector.tsx
│   ├── GameControls.tsx
│   └── PlayerList.tsx
├── hooks/
│   ├── useGame.ts
│   └── useSolana.ts
├── types/
│   └── game.ts
├── utils/
│   ├── constants.ts
│   └── helpers.ts
├── styles/
│   ├── components.css
│   └── variables.css
└── App.tsx
```

## Implementation Notes
- Keep the UI simple and intuitive
- Focus on functionality over complex animations
- Use TypeScript for type safety
- Implement proper error handling
- Make it easy to modify and extend
- Follow React best practices
- Use CSS modules or styled-components for styling 