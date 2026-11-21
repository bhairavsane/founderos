import React, { useState, useEffect } from 'react';
import { Modality, SearchResultItem } from '../types';
import { Search, ExternalLink } from './Icons';
import { searchFounderOS, openSourceFile } from '../services/api';

export const UniversalSearch: React.FC = () => {
  const [query, setQuery] = useState('');
  const [activeModality, setActiveModality] = useState<Modality | 'all'>('all');
  const [results, setResults] = useState<SearchResultItem[]>([]);

  useEffect(() => {
    let active = true;
    searchFounderOS(query, activeModality).then((res) => {
      if (active) setResults(res);
    });
    return () => { active = false; };
  }, [query, activeModality]);

  const handleOpenSource = (filePath: string) => {
    openSourceFile(filePath);
  };

  const filtered = results.filter((r) => activeModality === 'all' || r.modality === activeModality);

  return (
    <div style={{ padding: '32px 40px', maxWidth: '960px', margin: '0 auto', width: '100%' }}>
      <div style={{ marginBottom: '24px' }}>
        <h1 style={{ fontSize: '24px', fontWeight: 600, color: 'var(--text)' }}>Universal Search</h1>
        <p style={{ color: 'var(--muted)', fontSize: '13px', marginTop: '4px' }}>
          Search cross-modal company memory: text, screenshots, slide decks, and audio meetings.
        </p>
      </div>

      <div style={{
        display: 'flex',
        alignItems: 'center',
        gap: '12px',
        background: 'var(--card)',
        border: '1px solid var(--border)',
        borderRadius: '8px',
        padding: '12px 16px',
        boxShadow: '0 4px 12px rgba(0,0,0,0.25)',
      }}>
        <Search size={20} />
        <input
          type="text"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          placeholder="Search by natural language, quote, dollar figure, or acoustic topic..."
          style={{
            flex: 1,
            background: 'transparent',
            border: 'none',
            outline: 'none',
            color: 'var(--text)',
            fontSize: '15px',
          }}
        />
      </div>

      <div style={{ display: 'flex', gap: '8px', marginTop: '16px', marginBottom: '24px' }}>
        {(['all', 'text', 'image', 'audio', 'video'] as const).map((m) => (
          <button
            key={m}
            onClick={() => setActiveModality(m)}
            style={{
              padding: '6px 14px',
              borderRadius: '20px',
              border: '1px solid var(--border)',
              background: activeModality === m ? '#1e263d' : 'transparent',
              color: activeModality === m ? '#60a5fa' : 'var(--muted)',
              fontSize: '12px',
              cursor: 'pointer',
              textTransform: 'capitalize',
            }}
          >
            {m}
          </button>
        ))}
      </div>

      <div style={{ display: 'flex', flexDirection: 'column', gap: '12px' }}>
        {filtered.map((r) => (
          <div
            key={r.id}
            style={{
              background: 'var(--card)',
              border: '1px solid var(--border)',
              borderRadius: '8px',
              padding: '16px 20px',
              display: 'flex',
              flexDirection: 'column',
              gap: '8px',
            }}
          >
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
              <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                <span style={{ fontWeight: 600, fontSize: '14px', color: 'var(--text)' }}>{r.fileName}</span>
                <span style={{
                  fontSize: '11px',
                  background: '#1a2238',
                  color: '#60a5fa',
                  padding: '2px 8px',
                  borderRadius: '4px',
                  textTransform: 'uppercase',
                  fontWeight: 600,
                }}>
                  {r.modality}
                </span>
                {r.timestampOrPage && (
                  <span style={{ fontSize: '11px', color: 'var(--muted)' }}>({r.timestampOrPage})</span>
                )}
              </div>
              <button
                onClick={() => handleOpenSource(r.filePath)}
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  gap: '6px',
                  background: '#1a2238',
                  border: '1px solid var(--border)',
                  color: 'var(--text)',
                  fontSize: '12px',
                  padding: '6px 12px',
                  borderRadius: '6px',
                  cursor: 'pointer',
                }}
              >
                <ExternalLink size={12} />
                Open Source
              </button>
            </div>
            <p style={{ fontSize: '13px', color: '#c4cad4', lineHeight: 1.5 }}>{r.snippet}</p>
            <div style={{ display: 'flex', gap: '12px', fontSize: '11px', color: 'var(--muted)' }}>
              <span>Score: {(r.score * 100).toFixed(1)}%</span>
              <span>Match: {r.matchType.replace('_', ' ')}</span>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
};
