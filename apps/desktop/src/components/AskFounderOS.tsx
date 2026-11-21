import React, { useState } from 'react';
import { CitationItem } from '../types';
import { Sparkles, ExternalLink } from './Icons';
import { askFounderOS, openSourceFile } from '../services/api';

interface ChatMessage {
  id: string;
  sender: 'user' | 'assistant';
  text: string;
  citations?: CitationItem[];
}

export const AskFounderOS: React.FC = () => {
  const [input, setInput] = useState('');
  const [loading, setLoading] = useState(false);
  const [messages, setMessages] = useState<ChatMessage[]>([
    {
      id: 'm1',
      sender: 'user',
      text: 'What did Xerophylla agree to in our meeting and what was the quote?',
    },
    {
      id: 'm2',
      sender: 'assistant',
      text: 'During the 10-minute client discussion, Xerophylla Corp agreed to contract pricing of $43,820 after final scope review.',
      citations: [
        {
          fileName: 'client_call_10min.wav',
          filePath: '/Users/bhairav/Documents/founderos/data/client_call_10min.wav',
          citationType: 'derived',
          evidence: 'Whisper STT lexical index matching $43,820',
          timestampOrPage: '07:20 - 07:50',
        },
        {
          fileName: 'client_call_10min.wav',
          filePath: '/Users/bhairav/Documents/founderos/data/client_call_10min.wav',
          citationType: 'native',
          evidence: '768-d acoustic segment vector alignment for contract terms',
          timestampOrPage: '07:20 - 07:50',
        },
      ],
    },
  ]);

  const handleSend = async () => {
    if (!input.trim() || loading) return;
    const q = input;
    const userMsg: ChatMessage = { id: `u-${Date.now()}`, sender: 'user', text: q };
    setMessages((prev) => [...prev, userMsg]);
    setInput('');
    setLoading(true);

    try {
      const res = await askFounderOS(q);
      const botMsg: ChatMessage = {
        id: `b-${Date.now()}`,
        sender: 'assistant',
        text: res.answer,
        citations: res.citations,
      };
      setMessages((prev) => [...prev, botMsg]);
    } finally {
      setLoading(false);
    }
  };


  return (
    <div style={{ padding: '32px 40px', maxWidth: '880px', margin: '0 auto', width: '100%', display: 'flex', flexDirection: 'column', height: '100vh', boxSizing: 'border-box' }}>
      <div style={{ marginBottom: '20px' }}>
        <h1 style={{ fontSize: '24px', fontWeight: 600, color: 'var(--text)' }}>Ask FounderOS</h1>
        <p style={{ color: 'var(--muted)', fontSize: '13px', marginTop: '4px' }}>
          Conversational intelligence grounded in private company documents and media.
        </p>
      </div>

      <div style={{ flex: 1, overflowY: 'auto', display: 'flex', flexDirection: 'column', gap: '16px', paddingRight: '8px' }}>
        {messages.map((m) => (
          <div
            key={m.id}
            style={{
              alignSelf: m.sender === 'user' ? 'flex-end' : 'flex-start',
              maxWidth: '85%',
              background: m.sender === 'user' ? '#1e293b' : 'var(--card)',
              border: '1px solid var(--border)',
              borderRadius: '8px',
              padding: '16px 20px',
            }}
          >
            <div style={{ fontSize: '14px', lineHeight: 1.6, color: 'var(--text)' }}>{m.text}</div>

            {m.citations && m.citations.length > 0 && (
              <div style={{ marginTop: '16px', paddingTop: '12px', borderTop: '1px solid var(--border)' }}>
                <div style={{ fontSize: '12px', fontWeight: 600, color: 'var(--muted)', marginBottom: '8px' }}>
                  Sources & Evidence:
                </div>
                <div style={{ display: 'flex', flexDirection: 'column', gap: '6px' }}>
                  {m.citations.map((c, idx) => (
                    <div
                      key={idx}
                      style={{
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'space-between',
                        fontSize: '11px',
                        background: '#0e111a',
                        padding: '6px 10px',
                        borderRadius: '4px',
                      }}
                    >
                      <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                        <span style={{
                          padding: '1px 6px',
                          borderRadius: '3px',
                          fontSize: '10px',
                          fontWeight: 700,
                          background: c.citationType === 'native' ? '#065f46' : '#78350f',
                          color: c.citationType === 'native' ? '#34d399' : '#fde68a',
                          textTransform: 'uppercase',
                        }}>
                          {c.citationType}
                        </span>
                        <span style={{ color: 'var(--text)', fontWeight: 500 }}>{c.fileName}</span>
                        {c.timestampOrPage && <span style={{ color: 'var(--muted)' }}>({c.timestampOrPage})</span>}
                        <span style={{ color: 'var(--muted)' }}>— {c.evidence}</span>
                      </div>
                      <button
                        onClick={() => openSourceFile(c.filePath)}
                        style={{ background: 'transparent', border: 'none', color: '#60a5fa', cursor: 'pointer', display: 'flex', alignItems: 'center' }}
                      >
                        <ExternalLink size={12} />
                      </button>

                    </div>
                  ))}
                </div>
              </div>
            )}
          </div>
        ))}
      </div>

      <div style={{ marginTop: '20px', display: 'flex', gap: '10px' }}>
        <input
          type="text"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={(e) => e.key === 'Enter' && handleSend()}
          placeholder="Ask FounderOS about meetings, decisions, financial numbers, or designs..."
          style={{
            flex: 1,
            background: 'var(--card)',
            border: '1px solid var(--border)',
            borderRadius: '6px',
            padding: '12px 16px',
            color: 'var(--text)',
            fontSize: '14px',
            outline: 'none',
          }}
        />
        <button
          onClick={handleSend}
          style={{
            padding: '0 20px',
            background: '#2563eb',
            border: 'none',
            borderRadius: '6px',
            color: '#fff',
            fontWeight: 600,
            cursor: 'pointer',
            display: 'flex',
            alignItems: 'center',
            gap: '6px',
          }}
        >
          <Sparkles size={16} />
          Ask
        </button>
      </div>
    </div>
  );
};
