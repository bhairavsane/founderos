<p align="center">
  <img src="assets/svg/founderos-mark.svg" width="96" alt="FounderOS Logo" />
</p>

<h1 align="center">FounderOS</h1>

<p align="center">
  <strong>The 100% Local, Air-Gapped AI Copilot for Founders.</strong><br>
  Instant grounded company memory across pitch decks, customer meetings, screen recordings, and financials — blazing fast and zero cloud telemetry.
</p>

<p align="center">
  <a href="https://github.com/bhairavsane/founderos/actions/workflows/build.yml"><img src="https://github.com/bhairavsane/founderos/actions/workflows/build.yml/badge.svg" alt="Release Builds" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-Apache_2.0-blue.svg" alt="License: Apache-2.0" /></a>
  <a href="https://www.rust-lang.org"><img src="https://img.shields.io/badge/rust-1.80%2B-orange.svg" alt="Rust" /></a>
  <a href="https://tauri.app"><img src="https://img.shields.io/badge/tauri-v2-24c8db.svg" alt="Tauri v2" /></a>
</p>

---

## The Microsoft Copilot & Recall Killer

Founders handle high-stakes intellectual property: confidential cap tables, M&A deal memos, enterprise customer calls, and product patents. Sending this data to external clouds or unencrypted system scrapers is unacceptable.

FounderOS rethinks the desktop assistant from the ground up: **zero cloud dependence, sub-10ms search latency, and mathematically verifiable local citations.**

| Capability | FounderOS | Microsoft Copilot & Recall |
|---|---|---|
| **Privacy & Sovereignty** | **100% Air-Gapped**: Zero cloud calls, zero telemetry, local weights | Cloud uploads, telemetry tracking, plaintext storage exposure |
| **Search Speed** | **< 10 ms**: Instant hybrid FTS5 + LanceDB vector ranking | 1,500 – 4,000 ms: Sluggish cloud API roundtrips |
| **True Multimodal Memory** | **768-d Unified Metric Space**: Docs, Audio, Images, Video | Fragile OCR screen scraping, fragmented cloud models |
| **Zero Setup Friction** | **Self-Contained**: In-app direct GGUF downloader (no terminal/Ollama needed) | Requires Microsoft 365 enterprise subscription & account login |
| **Cross-Platform Freedom** | **Universal**: Native on macOS (Apple Silicon + Intel), Windows, Linux | Locked into Windows 11 Copilot+ PC hardware |

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
- Explicit citations distinguishing between **Native** (768-d acoustic/visual matches) and **Derived** (Whisper transcripts / FTS5 lexical tokens).

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
├── assets/         # Official icon bundle, SVG marks, design tokens
├── crates/
│   ├── domain/     # Core domain entities, vector signatures, L2 normalization
│   ├── storage/    # SQLite FTS5 and LanceDB vector repository traits
│   ├── runtime/    # InferenceProvider trait, sidecar manager, model downloader
│   └── pipeline/   # Multimodal chunking, ingestion, hybrid retriever, RAG service
└── apps/
    └── desktop/    # Native Tauri v2 desktop app with React UI
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
