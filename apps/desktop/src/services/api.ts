import { SearchResultItem, AssetItem, ModelConfigState, CitationItem, Modality } from '../types';

let tauriInvoke: (<T>(cmd: string, args?: Record<string, unknown>) => Promise<T>) | null = null;

async function getInvoke() {
  if (tauriInvoke) return tauriInvoke;
  try {
    const tauri = await import('@tauri-apps/api/core');
    tauriInvoke = tauri.invoke;
    return tauriInvoke;
  } catch {
    return null;
  }
}

export async function searchFounderOS(queryText: string, modality: Modality | 'all'): Promise<SearchResultItem[]> {
  const invoke = await getInvoke();
  if (invoke) {
    return invoke<SearchResultItem[]>('search', { queryText, modality });
  }

  // Fallback dev mock
  return [
    {
      id: 'mock-1',
      assetId: 'ast-1',
      fileName: 'client_call_10min.wav',
      filePath: '/Users/bhairav/Documents/founderos/data/client_call_10min.wav',
      modality: 'audio',
      snippet: 'Client Xerophylla Corp agreed to contract pricing of $43,820 after review.',
      timestampOrPage: '07:20 - 07:50',
      score: 0.95,
      matchType: 'hybrid_fused',
    },
    {
      id: 'mock-2',
      assetId: 'ast-2',
      fileName: 'financial_plan.txt',
      filePath: '/Users/bhairav/Documents/founderos/data/financial_plan.txt',
      modality: 'text',
      snippet: 'FounderOS Q3 budget allocation for deep tech R&D and team hiring.',
      score: 0.88,
      matchType: 'vector_semantic',
    },
  ];
}

export async function askFounderOS(question: string): Promise<{ answer: string; citations: CitationItem[] }> {
  const invoke = await getInvoke();
  if (invoke) {
    return invoke<{ answer: string; citations: CitationItem[] }>('ask', { question });
  }

  return {
    answer: 'According to meeting recordings and financial notes, Xerophylla Corp agreed to contract pricing of $43,820 during the review discussion.',
    citations: [
      {
        fileName: 'client_call_10min.wav',
        filePath: '/Users/bhairav/Documents/founderos/data/client_call_10min.wav',
        citationType: 'derived',
        evidence: 'Whisper STT transcript token index matching $43,820',
        timestampOrPage: '07:20 - 07:50',
      },
      {
        fileName: 'client_call_10min.wav',
        filePath: '/Users/bhairav/Documents/founderos/data/client_call_10min.wav',
        citationType: 'native',
        evidence: '768-d acoustic vector similarity for pricing terms',
        timestampOrPage: '07:20 - 07:50',
      },
    ],
  };
}

export async function fetchAssets(): Promise<AssetItem[]> {
  const invoke = await getInvoke();
  if (invoke) {
    return invoke<AssetItem[]>('list_assets');
  }

  return [
    {
      id: 'ast-1',
      fileName: 'financial_plan.txt',
      filePath: '/Users/bhairav/Documents/founderos/data/financial_plan.txt',
      mimeType: 'text/plain',
      fileSize: 1048576,
      status: 'indexed',
      indexedAt: '2026-11-21 14:15',
    },
    {
      id: 'ast-2',
      fileName: 'client_call_10min.wav',
      filePath: '/Users/bhairav/Documents/founderos/data/client_call_10min.wav',
      mimeType: 'audio/wav',
      fileSize: 19200000,
      status: 'indexed',
      indexedAt: '2026-11-21 14:18',
    },
  ];
}

export async function fetchModelStatus(): Promise<ModelConfigState> {
  const invoke = await getInvoke();
  if (invoke) {
    return invoke<ModelConfigState>('get_model_status');
  }

  return {
    ollamaConnected: true,
    embeddingModel: 'embeddinggemma-2:740m',
    dimension: 768,
    chatModel: 'llama3.2-vision:11b',
    isChatMultimodal: true,
    storageBudgetGb: 10.0,
    storageUsedGb: 1.45,
  };
}

export async function openSourceFile(filePath: string): Promise<void> {
  const invoke = await getInvoke();
  if (invoke) {
    await invoke('open_source_file', { filePath });
  } else {
    alert(`[FounderOS Desktop] Opening source file locally: ${filePath}`);
  }
}

export async function indexDirectory(dirPath: string): Promise<number> {
  const invoke = await getInvoke();
  if (invoke) {
    return invoke<number>('index_directory', { dirPath });
  }
  return 42;
}

export interface InstalledModelInfo {
  name: string;
  filename: string;
  installed: boolean;
  sizeBytes: number;
  engineVersion: string;
}

export async function listInstalledModels(): Promise<InstalledModelInfo[]> {
  const invoke = await getInvoke();
  if (invoke) {
    return invoke<InstalledModelInfo[]>('list_installed_models');
  }
  return [
    {
      name: 'embeddinggemma-2-740m-q8_0',
      filename: 'embeddinggemma-2-740m-q8_0.gguf',
      installed: true,
      sizeBytes: 812445120,
      engineVersion: 'llama.cpp-b3800',
    },
    {
      name: 'mmproj-embeddinggemma-2-740m-f16',
      filename: 'mmproj-embeddinggemma-2-740m-f16.gguf',
      installed: true,
      sizeBytes: 345221888,
      engineVersion: 'llama.cpp-b3800',
    },
    {
      name: 'llama-3.2-3b-instruct-q4_k_m',
      filename: 'llama-3.2-3b-instruct-q4_k_m.gguf',
      installed: false,
      sizeBytes: 2020112000,
      engineVersion: 'llama.cpp-b3800',
    },
  ];
}

export async function downloadModel(modelName: string): Promise<string> {
  const invoke = await getInvoke();
  if (invoke) {
    return invoke<string>('download_model', { modelName });
  }
  return `/Users/bhairav/.founderos/models/${modelName}`;
}

