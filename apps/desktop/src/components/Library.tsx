import React, { useState, useEffect } from 'react';
import { AssetItem } from '../types';
import { ExternalLink } from './Icons';
import { fetchAssets, openSourceFile, indexDirectory } from '../services/api';

export const Library: React.FC = () => {
  const [assets, setAssets] = useState<AssetItem[]>([]);
  const [indexing, setIndexing] = useState(false);
  const [folderPath, setFolderPath] = useState('/Users/bhairav/Documents/founderos/data');
  const [statusMessage, setStatusMessage] = useState<string | null>(null);
  const [isDragOver, setIsDragOver] = useState(false);

  useEffect(() => {
    fetchAssets().then(setAssets);
  }, []);

  const handleIndexFolder = async (pathToIndex?: string) => {
    const dir = pathToIndex || folderPath.trim();
    if (!dir) return;
    setIndexing(true);
    setStatusMessage(`Scanning and indexing files from ${dir}...`);
    try {
      const count = await indexDirectory(dir);
      setStatusMessage(`Successfully indexed ${count} files into 768-d multimodal store.`);
      const updated = await fetchAssets();
      setAssets(updated);
    } catch (e) {
      setStatusMessage(`Indexing failed: ${e}`);
    } finally {
      setIndexing(false);
    }
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragOver(false);
    if (e.dataTransfer.files && e.dataTransfer.files.length > 0) {
      const droppedPath = (e.dataTransfer.files[0] as unknown as { path?: string }).path || folderPath;
      handleIndexFolder(droppedPath);
    }
  };

  const formatBytes = (bytes: number) => {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  };

  return (
    <div style={{ padding: '32px 40px', maxWidth: '960px', margin: '0 auto', width: '100%' }}>
      <div style={{ marginBottom: '20px' }}>
        <h1 style={{ fontSize: '24px', fontWeight: 600, color: 'var(--text)' }}>Knowledge Library</h1>
        <p style={{ color: 'var(--muted)', fontSize: '13px', marginTop: '4px' }}>
          Index company files into private local memory with universal 768-d vectors and FTS5.
        </p>
      </div>

      <div
        onDragOver={(e) => { e.preventDefault(); setIsDragOver(true); }}
        onDragLeave={() => setIsDragOver(false)}
        onDrop={handleDrop}
        style={{
          background: isDragOver ? '#1e284a' : 'var(--card)',
          border: isDragOver ? '2px dashed #3b82f6' : '1px solid var(--border)',
          borderRadius: '8px',
          padding: '16px 20px',
          marginBottom: '20px',
          display: 'flex',
          flexDirection: 'column',
          gap: '12px',
        }}
      >
        <div style={{ display: 'flex', gap: '10px' }}>
          <input
            type="text"
            value={folderPath}
            onChange={(e) => setFolderPath(e.target.value)}
            placeholder="Enter directory path (e.g. /path/to/company/docs)"
            style={{
              flex: 1,
              background: '#0d111d',
              border: '1px solid var(--border)',
              borderRadius: '6px',
              padding: '8px 12px',
              color: 'var(--text)',
              fontSize: '13px',
              outline: 'none',
            }}
          />
          <button
            onClick={() => handleIndexFolder()}
            disabled={indexing}
            style={{
              padding: '8px 18px',
              background: indexing ? '#4b5563' : '#2563eb',
              border: 'none',
              borderRadius: '6px',
              color: '#fff',
              fontWeight: 500,
              fontSize: '13px',
              cursor: indexing ? 'not-allowed' : 'pointer',
              whiteSpace: 'nowrap',
            }}
          >
            {indexing ? 'Indexing...' : '+ Index Directory'}
          </button>
        </div>
        <div style={{ fontSize: '12px', color: 'var(--muted)', display: 'flex', justifyContent: 'space-between' }}>
          <span>Supported: PDF, DOCX, TXT, PNG, JPG, WAV, MP4 · Drag & drop supported</span>
          {statusMessage && <span style={{ color: '#34d399', fontWeight: 500 }}>{statusMessage}</span>}
        </div>
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
                    title="Open Source"
                    style={{ background: 'transparent', border: 'none', color: '#60a5fa', cursor: 'pointer' }}
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
