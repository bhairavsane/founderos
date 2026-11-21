export type NavTab = 'search' | 'ask' | 'library' | 'models';

export type Modality = 'text' | 'image' | 'audio' | 'video';

export type MatchType = 'vector_semantic' | 'lexical_fts' | 'hybrid_fused';

export type CitationType = 'native' | 'derived';

export interface SearchResultItem {
  id: string;
  assetId: string;
  fileName: string;
  filePath: string;
  modality: Modality;
  snippet: string;
  timestampOrPage?: string;
  score: number;
  matchType: MatchType;
}

export interface CitationItem {
  fileName: string;
  filePath: string;
  citationType: CitationType;
  evidence: string;
  timestampOrPage?: string;
}

export interface AssetItem {
  id: string;
  fileName: string;
  filePath: string;
  mimeType: string;
  fileSize: number;
  status: 'indexed' | 'processing' | 'failed' | 'pending';
  indexedAt: string;
}

export interface ModelConfigState {
  ollamaConnected: boolean;
  embeddingModel: string;
  dimension: number;
  chatModel: string;
  isChatMultimodal: boolean;
  storageBudgetGb: number;
  storageUsedGb: number;
}
