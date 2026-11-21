import React from 'react';

interface IconProps {
  size?: number;
}

export const Search: React.FC<IconProps> = ({ size = 16 }) => (
  <svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    <circle cx="11" cy="11" r="8"></circle>
    <line x1="21" y1="21" x2="16.65" y2="16.65"></line>
  </svg>
);

export const Sparkles: React.FC<IconProps> = ({ size = 16 }) => (
  <svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    <path d="m12 3-1.912 5.813a2 2 0 0 1-1.275 1.275L3 12l5.813 1.912a2 2 0 0 1 1.275 1.275L12 21l1.912-5.813a2 2 0 0 1 1.275-1.275L21 12l-5.813-1.912a2 2 0 0 1-1.275-1.275L12 3Z"></path>
  </svg>
);

export const FolderLibrary: React.FC<IconProps> = ({ size = 16 }) => (
  <svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    <path d="M4 20h16a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.93a2 2 0 0 1-1.66-.9l-.82-1.2A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13c0 1.1.9 2 2 2Z"></path>
  </svg>
);

export const Cpu: React.FC<IconProps> = ({ size = 16 }) => (
  <svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    <rect width="16" height="16" x="4" y="4" rx="2"></rect>
    <path d="M9 9h6v6H9z"></path>
    <path d="M15 2v2"></path><path d="M15 20v2"></path><path d="M2 15h2"></path><path d="M2 9h2"></path>
    <path d="M20 15h2"></path><path d="M20 9h2"></path><path d="M9 2v2"></path><path d="M9 20v2"></path>
  </svg>
);

export const ExternalLink: React.FC<IconProps> = ({ size = 14 }) => (
  <svg width={size} height={size} viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
    <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"></path>
    <polyline points="15 3 21 3 21 9"></polyline>
    <line x1="10" y1="14" x2="21" y2="3"></line>
  </svg>
);

export const FounderOSMark: React.FC<IconProps> = ({ size = 28 }) => (
  <svg width={size} height={size} viewBox="0 0 64 64" fill="none">
    <defs>
      <linearGradient id="metalTop" x1="12" y1="10" x2="52" y2="28" gradientUnits="userSpaceOnUse">
        <stop offset="0%" stopColor="#A1A1AA" />
        <stop offset="100%" stopColor="#52525B" />
      </linearGradient>
      <linearGradient id="amberTop" x1="12" y1="21" x2="52" y2="39" gradientUnits="userSpaceOnUse">
        <stop offset="0%" stopColor="#FBBF24" />
        <stop offset="100%" stopColor="#D97706" />
      </linearGradient>
      <linearGradient id="metalBot" x1="12" y1="32" x2="52" y2="50" gradientUnits="userSpaceOnUse">
        <stop offset="0%" stopColor="#52525B" />
        <stop offset="100%" stopColor="#27272A" />
      </linearGradient>
    </defs>
    <path d="M12 41V44.5C12 45.8 13.2 46.8 15 47.7L29 54.7C30.9 55.6 33.1 55.6 35 54.7L49 47.7C50.8 46.8 52 45.8 52 44.5V41H12Z" fill="#18181B" />
    <path d="M35 32.5L49 39.5C51.5 40.7 51.5 42.3 49 43.5L35 50.5C33.1 51.4 30.9 51.4 29 50.5L15 43.5C12.5 42.3 12.5 40.7 15 39.5L29 32.5C30.9 31.6 33.1 31.6 35 32.5Z" fill="url(#metalBot)" />
    <path d="M12 30V33.5C12 34.8 13.2 35.8 15 36.7L29 43.7C30.9 44.6 33.1 44.6 35 43.7L49 36.7C50.8 35.8 52 34.8 52 33.5V30H12Z" fill="#92400E" />
    <path d="M35 21.5L49 28.5C51.5 29.7 51.5 31.3 49 32.5L35 39.5C33.1 40.4 30.9 40.4 29 39.5L15 32.5C12.5 31.3 12.5 29.7 15 28.5L29 21.5C30.9 20.6 33.1 20.6 35 21.5Z" fill="url(#amberTop)" />
    <path d="M12 19V22.5C12 23.8 13.2 24.8 15 25.7L29 32.7C30.9 33.6 33.1 33.6 35 32.7L49 25.7C50.8 24.8 52 23.8 52 22.5V19H12Z" fill="#3F3F46" />
    <path d="M35 10.5L49 17.5C51.5 18.7 51.5 20.3 49 21.5L35 28.5C33.1 29.4 30.9 29.4 29 28.5L15 21.5C12.5 20.3 12.5 18.7 15 17.5L29 10.5C30.9 9.6 33.1 9.6 35 10.5Z" fill="url(#metalTop)" />
  </svg>
);
