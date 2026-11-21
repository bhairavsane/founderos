# FounderOS

> **The Private, 100% Local AI Operating System for Founders.**  
> Grounded company memory across pitch decks, client calls, screen recordings, and financial models — entirely offline.

[![Release Builds](https://github.com/bhairavsane/founderos/actions/workflows/build.yml/badge.svg)](https://github.com/bhairavsane/founderos/actions/workflows/build.yml)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.80%2B-orange.svg)](https://www.rust-lang.org)
[![Tauri](https://img.shields.io/badge/tauri-v2-24c8db.svg)](https://tauri.app)

---

## Why FounderOS?

Founders handle the world's most sensitive information: cap tables, unpublished patent filings, enterprise customer call recordings, and runway models. Sending this data to external cloud APIs creates unmanageable compliance, IP leakage, and privacy risks.

**FounderOS** runs 100% locally on your machine with zero cloud roundtrips:
- **No Mandatory Ollama CLI**: Pinned in-app inference runtime and direct GGUF model downloader. Non-technical founders never have to touch a terminal.
- **Single Universal Embedding Configuration**: Canonical 768-d L2-normalized vectors (EmbeddingGemma-2 740M Q8_0 + F16 projector). Upgrading chat models or profiles never forces a re-index.
- **True Multimodal Representation**: Shared embedding space unifying documents, screenshots, audio meetings, and video keyframes.
- **Grounded Conversational RAG**: Citations explicitly distinguish between *Native* (768-d acoustic/visual matches) and *Derived* (Whisper transcripts / FTS5 lexical tokens), with strict prompt-injection shielding.

---

## Core Capabilities (v1)

### 1. Universal Search
Cross-modal search powered by hybrid Reciprocal Rank Fusion (RRF):
- **Lexical exact matches** via SQLite FTS5 (handles dollar figures, exact names, acronyms).
- **Semantic vector matches** via LanceDB (handles natural language concepts and themes).
- **Timestamp playback modal**: Video and audio results link directly to exact timestamps (e.g. `07:20 - 07:50`).

### 2. Ask FounderOS
Conversational intelligence grounded in private company records:
- Content treated strictly as untrusted data to shield against injection attacks.
- Separate facts from assumptions with verifiable citation links to source files.

### 3. Knowledge Library
Drag-and-drop ingestion pipeline:
- **Documents**: PDF, DOCX, TXT with structure-aware chunking.
- **Audio**: Acoustic feature extraction, 30s segmentation, and background transcription.
- **Video**: Scene-change adaptive sampling (`gt(scene,0.3)`) and keyframe timestamp binding.

### 4. Models & Indexing
- Resumable in-app downloader with byte-range requests and SHA-256 verification.
- Offline import for pre-downloaded GGUF weights.
- Ephemeral localhost port binding with per-launch random API tokens and stdin-EOF watchdog.

---

## Architecture

FounderOS is structured as a modular Rust Cargo workspace with a Tauri v2 desktop application:

```
founderos/
├── crates/
│   ├── domain/     # Core domain entities, vector signatures, L2 normalization
│   ├── storage/    # SQLite FTS5 and LanceDB vector repository traits
│   ├── runtime/    # InferenceProvider trait, sidecar manager, model downloader
│   └── pipeline/   # Multimodal chunking, ingestion, hybrid retriever, RAG service
└── apps/
    └── desktop/    # Native Tauri v2 app with React frontend
```

---

## Phase 0 Feasibility Gate

Before starting feature development, FounderOS passed a strict 10/10 feasibility gate:
- Multimodal 768-d extraction (Text, PNG, WAV).
- Numeric sanity (No NaN/Inf, L2 norm within $1 \pm 1e-3$).
- Q8 vs BF16 agreement (Cosine $\ge 0.999$).
- Cross-modal retrieval beating random baseline.
- Audio needle test ($43,820 pricing quote in 10-minute audio ranks Top-1).

Read the full report: [`docs/reports/phase0-feasibility-gate.md`](docs/reports/phase0-feasibility-gate.md).

---

## Quickstart

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (1.80+)
- [Node.js](https://nodejs.org/) (20+) and `npm`

### Development
```bash
# Clone the repository
git clone https://github.com/bhairavsane/founderos.git
cd founderos

# Install frontend dependencies
cd apps/desktop && npm install && cd ../..

# Run tests across workspace
cargo test --workspace

# Launch the desktop app in development
npm --prefix apps/desktop run tauri dev
```

### Deterministic Verification Gate
```bash
bash .agents/skills/verify-deterministic/scripts/verify.sh
```

---

## License

Distributed under the Apache 2.0 License. See `LICENSE` for details.
