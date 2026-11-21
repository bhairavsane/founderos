import React, { useState, useEffect } from 'react';
import { AssetItem } from '../types';
import { ExternalLink } from './Icons';
import { fetchAssets, openSourceFile } from '../services/api';

export const Library: React.FC = () => {
  const [assets, setAssets] = useState<AssetItem[]>([]);

  useEffect(() => {
    fetchAssets().then(setAssets);
  }, []);

  const formatBytes = (bytes: number) => {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  };

  return (
    <div style={{ padding: '32px 40px', maxWidth: '960px', margin: '0 auto', width: '100%' }}>
      <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '24px' }}>
        <div>
          <h1 style={{ fontSize: '24px', fontWeight: 600, color: 'var(--text)' }}>Knowledge Library</h1>
          <p style={{ color: 'var(--muted)', fontSize: '13px', marginTop: '4px' }}>
            Local files indexed with 768-d multimodal vectors and full-text index.
          </p>
        </div>
        <button
          onClick={() => alert('Folder import opened')}
          style={{
            padding: '8px 16px',
            background: '#2563eb',
            border: 'none',
            borderRadius: '6px',
            color: '#fff',
            fontWeight: 500,
            fontSize: '13px',
            cursor: 'pointer',
          }}
        >
          + Index Folder / File
        </button>
      </div>

      <div style={{ background: 'var(--card)', border: '1px solid var(--border)', borderRadius: '8px', overflow: 'hidden' }}>
        <table style={{ width: '100%', borderCollapse: 'collapse', textAlign: 'left', fontSize: '13px' }}>
          <thead>
            <tr style={{ borderBottom: '1px solid var(--border)', color: 'var(--muted)' }}>
              <th style={{ padding: '12px 16px' }}>Name</th>
              <th style={{ padding: '12px 16px' }}>Type</th>
              <th style={{ padding: '12px 16px' }}>Size</th>
              <th style={{ padding: '12px 16px' }}>Status</th>
              <th style={{ padding: '12px 16px' }}>Indexed At</th>
              <th style={{ padding: '12px 16px' }}>Action</th>
            </tr>
          </thead>
          <tbody>
            {assets.map((a) => (
              <tr key={a.id} style={{ borderBottom: '1px solid var(--border)' }}>
                <td style={{ padding: '12px 16px', fontWeight: 500, color: 'var(--text)' }}>{a.fileName}</td>
                <td style={{ padding: '12px 16px', color: 'var(--muted)' }}>{a.mimeType}</td>
                <td style={{ padding: '12px 16px', color: 'var(--muted)' }}>{formatBytes(a.fileSize)}</td>
                <td style={{ padding: '12px 16px' }}>
                  <span style={{
                    padding: '2px 8px',
                    borderRadius: '4px',
                    fontSize: '11px',
                    fontWeight: 600,
                    background: a.status === 'indexed' ? '#064e3b' : '#374151',
                    color: a.status === 'indexed' ? '#34d399' : '#9ca3af',
                    textTransform: 'uppercase',
                  }}>
                    {a.status}
                  </span>
                </td>
                <td style={{ padding: '12px 16px', color: 'var(--muted)' }}>{a.indexedAt}</td>
                <td style={{ padding: '12px 16px' }}>
                  <button
                    onClick={() => openSourceFile(a.filePath)}
                    style={{ background: 'transparent', border: 'none', color: '#60a5fa', cursor: 'pointer', display: 'flex', alignItems: 'center' }}
                  >
                    <ExternalLink size={14} />
                  </button>

                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};
