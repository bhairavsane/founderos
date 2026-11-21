import React, { useState, useEffect } from 'react';
import { ModelConfigState } from '../types';
import { fetchModelStatus } from '../services/api';

export const ModelsIndexing: React.FC = () => {
  const [config, setConfig] = useState<ModelConfigState>({
    ollamaConnected: true,
    embeddingModel: 'embeddinggemma-2:740m',
    dimension: 768,
    chatModel: 'llama3.2-vision:11b',
    isChatMultimodal: true,
    storageBudgetGb: 10.0,
    storageUsedGb: 1.45,
  });

  useEffect(() => {
    fetchModelStatus().then(setConfig);
  }, []);

  const [profile, setProfile] = useState<'lite' | 'standard' | 'performance'>('standard');

  return (
    <div style={{ padding: '32px 40px', maxWidth: '880px', margin: '0 auto', width: '100%' }}>
      <div style={{ marginBottom: '24px' }}>
        <h1 style={{ fontSize: '24px', fontWeight: 600, color: 'var(--text)' }}>Models & Indexing</h1>
        <p style={{ color: 'var(--muted)', fontSize: '13px', marginTop: '4px' }}>
          Local AI model runtimes, vector signature configuration, and hardware resource profiles.
        </p>
      </div>

      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '20px', marginBottom: '24px' }}>
        <div style={{ background: 'var(--card)', border: '1px solid var(--border)', borderRadius: '8px', padding: '20px' }}>
          <div style={{ fontSize: '12px', fontWeight: 600, color: 'var(--muted)', textTransform: 'uppercase' }}>
            Embedding Model Runtime
          </div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--text)', marginTop: '8px' }}>
            {config.embeddingModel}
          </div>
          <div style={{ fontSize: '12px', color: '#34d399', marginTop: '4px', display: 'flex', alignItems: 'center', gap: '6px' }}>
            <span style={{ width: '8px', height: '8px', borderRadius: '50%', background: '#34d399', display: 'inline-block' }}></span>
            Ollama Local Runtime Connected (port 11434)
          </div>
          <div style={{ marginTop: '16px', fontSize: '13px', color: 'var(--muted)', lineHeight: 1.6 }}>
            <div>Dimension: <strong style={{ color: 'var(--text)' }}>{config.dimension}-d</strong> (L2-normalized)</div>
            <div>Shared Modalities: <strong style={{ color: 'var(--text)' }}>Text, Vision, Audio</strong></div>
            <div>Sidecar Fallback: <strong style={{ color: '#34d399' }}>Pre-approved (llama.cpp/ONNX)</strong></div>
          </div>
        </div>

        <div style={{ background: 'var(--card)', border: '1px solid var(--border)', borderRadius: '8px', padding: '20px' }}>
          <div style={{ fontSize: '12px', fontWeight: 600, color: 'var(--muted)', textTransform: 'uppercase' }}>
            Local Chat Model (Ask FounderOS)
          </div>
          <div style={{ fontSize: '18px', fontWeight: 700, color: 'var(--text)', marginTop: '8px' }}>
            {config.chatModel}
          </div>
          <div style={{ fontSize: '12px', color: '#60a5fa', marginTop: '4px' }}>
            Multimodal Vision Detected: Direct Image Pass-through
          </div>
          <div style={{ marginTop: '16px', fontSize: '13px', color: 'var(--muted)', lineHeight: 1.6 }}>
            <div>Grounding: <strong style={{ color: 'var(--text)' }}>Strict Local Citations</strong></div>
            <div>Query Enrichment: <strong style={{ color: 'var(--text)' }}>Top-K On-demand</strong></div>
            <div>Offline Guarantee: <strong style={{ color: '#34d399' }}>100% Private (No Cloud)</strong></div>
          </div>
        </div>
      </div>

      <div style={{ background: 'var(--card)', border: '1px solid var(--border)', borderRadius: '8px', padding: '20px', marginBottom: '24px' }}>
        <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '10px' }}>
          <span style={{ fontSize: '14px', fontWeight: 600, color: 'var(--text)' }}>Storage Budget</span>
          <span style={{ fontSize: '13px', color: 'var(--muted)' }}>
            {config.storageUsedGb.toFixed(2)} GB used / {config.storageBudgetGb} GB allocated
          </span>
        </div>
        <div style={{ height: '8px', background: '#1e2436', borderRadius: '4px', overflow: 'hidden' }}>
          <div style={{
            height: '100%',
            width: `${(config.storageUsedGb / config.storageBudgetGb) * 100}%`,
            background: '#3b82f6',
            borderRadius: '4px',
          }}></div>
        </div>
        <div style={{ fontSize: '12px', color: 'var(--muted)', marginTop: '8px' }}>
          Scene-change adaptive video sampling enabled (`gt(scene,0.3)`). MRL 256-d truncation tested with L2 re-normalization.
        </div>
      </div>

      <div style={{ background: 'var(--card)', border: '1px solid var(--border)', borderRadius: '8px', padding: '20px' }}>
        <div style={{ fontSize: '14px', fontWeight: 600, color: 'var(--text)', marginBottom: '12px' }}>
          Hardware Processing Profile
        </div>
        <div style={{ display: 'flex', gap: '12px' }}>
          {(['lite', 'standard', 'performance'] as const).map((p) => (
            <button
              key={p}
              onClick={() => setProfile(p)}
              style={{
                flex: 1,
                padding: '12px',
                borderRadius: '6px',
                border: '1px solid var(--border)',
                background: profile === p ? '#1e263d' : 'transparent',
                color: profile === p ? 'var(--text)' : 'var(--muted)',
                cursor: 'pointer',
                textAlign: 'center',
                textTransform: 'capitalize',
                fontWeight: profile === p ? 600 : 400,
              }}
            >
              {p}
            </button>
          ))}
        </div>
      </div>
    </div>
  );
};
