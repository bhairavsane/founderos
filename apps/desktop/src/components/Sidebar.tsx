import React from 'react';
import { Search, Sparkles, FolderLibrary, Cpu } from './Icons';
import { NavTab } from '../types';

interface SidebarProps {
  currentTab: NavTab;
  onSelectTab: (tab: NavTab) => void;
}

export const Sidebar: React.FC<SidebarProps> = ({ currentTab, onSelectTab }) => {
  const items: { tab: NavTab; label: string; icon: React.ReactNode }[] = [
    { tab: 'search', label: 'Universal Search', icon: <Search size={18} /> },
    { tab: 'ask', label: 'Ask FounderOS', icon: <Sparkles size={18} /> },
    { tab: 'library', label: 'Knowledge Library', icon: <FolderLibrary size={18} /> },
    { tab: 'models', label: 'Models & Indexing', icon: <Cpu size={18} /> },
  ];

  return (
    <aside style={{
      width: '240px',
      borderRight: '1px solid var(--border)',
      background: 'var(--card)',
      padding: '20px 14px',
      display: 'flex',
      flexDirection: 'column',
      gap: '24px',
      height: '100vh',
      boxSizing: 'border-box'
    }}>
      <div>
        <div style={{ fontSize: '15px', fontWeight: 700, letterSpacing: '0.04em', color: 'var(--text)' }}>
          FOUNDEROS
        </div>
        <div style={{ fontSize: '11px', color: 'var(--muted)', marginTop: '2px' }}>
          Local Company Intelligence
        </div>
      </div>

      <nav style={{ display: 'flex', flexDirection: 'column', gap: '4px' }}>
        {items.map(({ tab, label, icon }) => {
          const isActive = currentTab === tab;
          return (
            <button
              key={tab}
              onClick={() => onSelectTab(tab)}
              style={{
                display: 'flex',
                alignItems: 'center',
                gap: '10px',
                padding: '9px 12px',
                borderRadius: '6px',
                border: 'none',
                cursor: 'pointer',
                background: isActive ? '#1d2335' : 'transparent',
                color: isActive ? 'var(--text)' : 'var(--muted)',
                fontWeight: isActive ? 600 : 400,
                fontSize: '13px',
                textAlign: 'left',
                transition: 'background 0.15s, color 0.15s',
              }}
            >
              {icon}
              {label}
            </button>
          );
        })}
      </nav>
    </aside>
  );
};
