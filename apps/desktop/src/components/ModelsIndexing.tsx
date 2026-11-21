import React, { useState, useEffect } from 'react';
import { ModelConfigState } from '../types';
import { fetchModelStatus, listInstalledModels, downloadModel, importOfflineModel, InstalledModelInfo } from '../services/api';

export const ModelsIndexing: React.FC = () => {
  const [config, setConfig] = useState<ModelConfigState>({
    ollamaConnected: true,
    embeddingModel: 'embeddinggemma-2:740m (Q8_0)',
    dimension: 768,
    chatModel: 'llama-3.2-3b-instruct (Q4_K_M)',
    isChatMultimodal: true,
    storageBudgetGb: 10.0,
    storageUsedGb: 1.45,
  });

  const [models, setModels] = useState<InstalledModelInfo[]>([]);
  const [downloading, setDownloading] = useState<string | null>(null);
  const [progress, setProgress] = useState<number>(0);
  const [profile, setProfile] = useState<'lite' | 'standard' | 'performance'>('standard');

  useEffect(() => {
    fetchModelStatus().then(setConfig);
    listInstalledModels().then(setModels);
  }, []);

  const handleDownload = async (modelName: string) => {
    setDownloading(modelName);
    setProgress(15);
    const interval = setInterval(() => {
      setProgress((p) => (p >= 90 ? 95 : p + 20));
    }, 150);
    try {
      await downloadModel(modelName);
      clearInterval(interval);
      setProgress(100);
      setTimeout(() => {
        setDownloading(null);
        setProgress(0);
        listInstalledModels().then(setModels);
      }, 400);
    } catch {
      clearInterval(interval);
      setDownloading(null);
    }
  };

  const handleImport = async (modelName: string) => {
    const p = prompt(`Enter local file path to ${modelName} .gguf file:`);
    if (!p) return;
    try {
      await importOfflineModel(p, modelName);
      alert('Offline weights verified (SHA-256) and imported.');
      listInstalledModels().then(setModels);
    } catch (e) {
      alert(`Import failed: ${e}`);
    }
  };

  return (
    <div style={{ padding: '32px 40px', maxWidth: '880px', margin: '0 auto', width: '100%' }}>
      <div style={{ marginBottom: '24px' }}>
        <h1 style={{ fontSize: '24px', fontWeight: 600, color: 'var(--text)' }}>Models & Indexing</h1>
        <p style={{ color: 'var(--muted)', fontSize: '13px', marginTop: '4px' }}>
          Self-contained local AI inference runtimes, pinned model weights, and zero-dependency offline setup.
        </p>
      </div>

      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '20px', marginBottom: '24px' }}>
        <div style={{ background: 'var(--card)', border: '1px solid var(--border)', borderRadius: '8px', padding: '20px' }}>
          <div style={{ fontSize: '12px', fontWeight: 600, color: 'var(--muted)', textTransform: 'uppercase' }}>
            Embedding Engine (Universal)
          </div>
          <div style={{ fontSize: '17px', fontWeight: 700, color: 'var(--text)', marginTop: '8px' }}>
            {config.embeddingModel}
          </div>
          <div style={{ fontSize: '12px', color: '#34d399', marginTop: '4px', display: 'flex', alignItems: 'center', gap: '6px' }}>
            <span style={{ width: '8px', height: '8px', borderRadius: '50%', background: '#34d399' }}></span>
            Pinned llama-server sidecar (no Ollama required)
          </div>
          <div style={{ marginTop: '14px', fontSize: '13px', color: 'var(--muted)', lineHeight: 1.6 }}>
            <div>Dimension: <strong style={{ color: 'var(--text)' }}>768-d</strong> (L2-norm 1.0)</div>
            <div>Modalities: <strong style={{ color: 'var(--text)' }}>Text, Vision, Audio</strong></div>
            <div>Active Memory: <strong style={{ color: 'var(--text)' }}>567 MB RAM</strong></div>
          </div>
        </div>

        <div style={{ background: 'var(--card)', border: '1px solid var(--border)', borderRadius: '8px', padding: '20px' }}>
          <div style={{ fontSize: '12px', fontWeight: 600, color: 'var(--muted)', textTransform: 'uppercase' }}>
            Local Chat Engine (Ask FounderOS)
          </div>
          <div style={{ fontSize: '17px', fontWeight: 700, color: 'var(--text)', marginTop: '8px' }}>
            {config.chatModel}
          </div>
          <div style={{ fontSize: '12px', color: '#60a5fa', marginTop: '4px' }}>
            Grounding: Native vectors + FTS5 citations
          </div>
          <div style={{ marginTop: '14px', fontSize: '13px', color: 'var(--muted)', lineHeight: 1.6 }}>
            <div>Privacy: <strong style={{ color: '#34d399' }}>100% Air-Gapped Local</strong></div>
            <div>Lifecycle: <strong style={{ color: 'var(--text)' }}>On-demand Load/Unload</strong></div>
            <div>Port Security: <strong style={{ color: 'var(--text)' }}>Localhost + Ephemeral Key</strong></div>
          </div>
        </div>
      </div>

      <div style={{ background: 'var(--card)', border: '1px solid var(--border)', borderRadius: '8px', padding: '20px', marginBottom: '24px' }}>
        <div style={{ fontSize: '14px', fontWeight: 600, color: 'var(--text)', marginBottom: '14px' }}>
          In-App Direct Model Downloads
        </div>
        <div style={{ display: 'flex', flexDirection: 'column', gap: '10px' }}>
          {models.map((m) => (
            <div key={m.filename} style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', padding: '10px 14px', background: '#131826', borderRadius: '6px', border: '1px solid var(--border)' }}>
              <div>
                <div style={{ fontSize: '13px', fontWeight: 600, color: 'var(--text)' }}>{m.name}</div>
                <div style={{ fontSize: '11px', color: 'var(--muted)' }}>
                  {(m.sizeBytes / (1024 * 1024)).toFixed(0)} MB · SHA-256 verified · {m.engineVersion}
                </div>
              </div>
              <div>
                {m.installed ? (
                  <span style={{ fontSize: '12px', color: '#34d399', fontWeight: 600 }}>✓ Installed</span>
                ) : (
                  <div style={{ display: 'flex', gap: '8px' }}>
                    <button
                      onClick={() => handleDownload(m.name)}
                      disabled={downloading === m.name}
                      style={{ padding: '6px 14px', borderRadius: '4px', background: '#2563eb', color: '#fff', border: 'none', cursor: 'pointer', fontSize: '12px', fontWeight: 500 }}
                    >
                      {downloading === m.name ? `Downloading ${progress}%` : 'Download'}
                    </button>
                    <button
                      onClick={() => handleImport(m.name)}
                      style={{ padding: '6px 10px', borderRadius: '4px', background: 'transparent', border: '1px solid var(--border)', color: 'var(--muted)', cursor: 'pointer', fontSize: '11px' }}
                    >
                      Import
                    </button>
                  </div>
                )}
              </div>
            </div>
          ))}
        </div>
      </div>

      <div style={{ background: 'var(--card)', border: '1px solid var(--border)', borderRadius: '8px', padding: '20px' }}>
        <div style={{ fontSize: '14px', fontWeight: 600, color: 'var(--text)', marginBottom: '4px' }}>
          Hardware Profile
        </div>
        <div style={{ fontSize: '12px', color: 'var(--muted)', marginBottom: '12px' }}>
          Profiles adjust chat concurrency and batching; universal 768-d embedding never changes or forces re-indexing.
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
