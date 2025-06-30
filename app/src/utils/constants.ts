import { Colors } from '../types/game';

// Game constants
export const BOARD_SIZE = 56;
export const TOKENS_PER_PLAYER = 4;
export const MAX_PLAYERS = 4;
export const MIN_PLAYERS = 2;

// Safe positions where tokens cannot be captured
export const SAFE_POSITIONS = [0, 13, 26, 39, 8, 21, 34, 47];

// Color configurations
export const COLOR_CONFIG = {
  [Colors.Red]: {
    name: 'Red',
    hex: '#FF6B6B',
    lightHex: '#FFE5E5',
    darkHex: '#CC5555',
  },
  [Colors.Green]: {
    name: 'Green',
    hex: '#4ECDC4',
    lightHex: '#E5F9F7',
    darkHex: '#3EA89F',
  },
  [Colors.Yellow]: {
    name: 'Yellow',
    hex: '#FFE66D',
    lightHex: '#FFF9E5',
    darkHex: '#E6CF62',
  },
  [Colors.Blue]: {
    name: 'Blue',
    hex: '#45B7D1',
    lightHex: '#E5F4F8',
    darkHex: '#3792A7',
  },
};

// Gather.town inspired styling
export const STYLES = {
  colors: {
    primary: '#6366F1',
    secondary: '#8B5CF6',
    success: '#10B981',
    warning: '#F59E0B',
    error: '#EF4444',
    background: '#F8FAFC',
    surface: '#FFFFFF',
    text: '#1E293B',
    textSecondary: '#64748B',
    border: '#E2E8F0',
  },
  spacing: {
    xs: '0.25rem',
    sm: '0.5rem',
    md: '1rem',
    lg: '1.5rem',
    xl: '2rem',
    xxl: '3rem',
  },
  borderRadius: {
    sm: '0.375rem',
    md: '0.5rem',
    lg: '0.75rem',
    xl: '1rem',
    full: '9999px',
  },
  shadows: {
    sm: '0 1px 2px 0 rgb(0 0 0 / 0.05)',
    md: '0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)',
    lg: '0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)',
  },
}; 