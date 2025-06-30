# Ludo Game Frontend

A simple, modern Ludo game frontend built with React + TypeScript + Vite, designed to interact with a Solana program. The UI follows Gather.town aesthetics with a clean, modern design.

## Features

- 🎲 **Ludo Game Logic**: Complete implementation of Ludo game rules
- 🎨 **Modern UI**: Clean, Gather.town inspired design
- 📱 **Responsive**: Works on desktop and mobile devices
- 🔗 **Solana Integration**: Ready for Solana wallet connection and transactions
- 🎯 **TypeScript**: Full type safety throughout the application
- ⚡ **Vite**: Fast development and build times

## Game Rules

- 2-4 players can join a game
- Each player has 4 tokens that start at position -1 (not in play)
- Players take turns rolling dice (1-6)
- Rolling a 6 allows you to bring a token into play (position 0)
- Rolling three 6s in a row skips your turn
- Tokens move around the board based on dice rolls
- Landing on another player's token sends it back to start (unless it's on a safe position)
- First player to get all 4 tokens to position 56 wins

## Project Structure

```
src/
├── components/          # React components
│   ├── GameBoard.tsx   # Main game board
│   ├── PlayerInfo.tsx  # Player information display
│   ├── DiceRoller.tsx  # Dice rolling interface
│   └── GameControls.tsx # Game creation/management
├── hooks/              # Custom React hooks
│   ├── useGame.ts      # Game state management
│   └── useSolana.ts    # Solana wallet integration
├── types/              # TypeScript type definitions
│   └── game.ts         # Game-related types
├── utils/              # Utility functions
│   ├── constants.ts    # Game constants and styling
│   └── helpers.ts      # Helper functions
├── styles/             # CSS styles
│   ├── variables.css   # CSS custom properties
│   └── components.css  # Component-specific styles
└── App.tsx             # Main application component
```

## Getting Started

### Prerequisites

- Node.js (v16 or higher)
- npm or yarn

### Installation

1. Install dependencies:
```bash
npm install
```

2. Start the development server:
```bash
npm run dev
```

3. Open your browser and navigate to `http://localhost:5173`

### Building for Production

```bash
npm run build
```

## Development

### Key Components

- **GameBoard**: Renders the visual Ludo board with tokens
- **PlayerInfo**: Shows current players and game status
- **DiceRoller**: Handles dice rolling with animations
- **GameControls**: Manages game creation and joining

### Styling

The app uses CSS custom properties for consistent theming, inspired by Gather.town:
- Clean, modern interface
- Soft colors and rounded corners
- Minimalist design
- Good contrast for readability
- Smooth animations

### Solana Integration

The app is prepared for Solana integration with:
- Wallet connection handling
- Transaction simulation
- VRF (Verifiable Random Function) support for dice rolling
- Game state management on-chain

## TODO

- [ ] Implement actual Solana wallet connection
- [ ] Add real transaction handling
- [ ] Implement VRF for dice rolling
- [ ] Add multiplayer support
- [ ] Improve game board visualization
- [ ] Add sound effects
- [ ] Add game history
- [ ] Implement proper error handling

## Contributing

This is a simple frontend implementation. Feel free to modify and extend it according to your needs. The code is structured to be easy to understand and modify.

## License

This project is open source and available under the MIT License.
