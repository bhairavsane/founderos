# Spec: founderos-core
Status: draft

## Problem
Founders and small teams struggle to search and connect their local digital assets across mixed modalities (documents, screenshots, slide decks, audio memos, and videos) without surrendering data privacy to cloud AI vendors or maintaining fragmented search tools.

## Non-goals
- Cloud AI sync or remote telemetry.
- Hardcoding a proprietary generation model.
- High-level intelligence modules (Meetings, Customers, Decisions) in v1.
- Ingestion of raw video files directly into embedding APIs without frame/audio extraction.
- Automatic vector dimension truncation without verified L2 re-normalization and recall benchmarking.

## Acceptance criteria
- AC1: System detects local Ollama instance and verifies `embeddinggemma-2:740m` availability and dimension (768).
- AC2: Ingests text, PDF (pdfium), image, audio (30s chunks), and video (scene-change sampling) into unified 768-d L2-normalized vector space.
- AC3: Vector records strictly pin model tag, quantization, and prefix config; mismatched vectors are rejected at query and ingestion time.
- AC4: Hybrid retrieval fuses LanceDB vector similarity and SQLite FTS5 lexical scores via Reciprocal Rank Fusion (RRF).
- AC5: Needle test succeeds: exact dollar amounts and rare proper nouns in 10-minute audio recordings are retrieved via STT+FTS5, while semantic audio concepts are retrieved via native audio embeddings.
- AC6: Ask FounderOS detects if the configured chat model is multimodal; passes images directly if multimodal, or performs query-time enrichment (OCR/caption/STT) on top-K chunks only.
- AC7: Every citation clearly distinguishes whether evidence was native or derived with exact timestamps/pages.
- AC8: Universal Search UI delivers text, image, and audio query inputs with preview and "Open Original Source" action.

## Constraints
- Performance: Search latency < 150ms for 10k vectors; peak RAM < 2.2GB in standard mode.
- Security/Privacy: 100% offline; localhost only; no document content in logs.
- Modularity: Clean 4-crate Rust workspace (`domain`, `storage`, `runtime`, `pipeline`).

## Assumptions (veto-able)
- Ollama is running locally on port 11434. If multimodal embedding fails in Ollama, llama.cpp/ONNX sidecar with identical weights takes over embeddings.
- Audio is resampled to 16kHz mono PCM for embedding and STT.
- Video keyframes are extracted on scene change (`gt(scene,0.3)`) rather than fixed intervals.

## Risks
1. Memory spikes during multimodal batch embedding -> Bounded concurrency queues and selective encoder loading.
2. Incompatible vector dimensions or prefix drift -> Cryptographic/pinned signature on every vector record.
3. Audio needle miss -> Asynchronous background STT with SQLite FTS5 fallback.

## Tasks
- T1 -> AC1,AC3 | Scaffold 4 crates (`domain`, `storage`, `runtime`, `pipeline`) with vector pinning | verify: cargo check --all | files: ~8
- T2 -> AC2,AC3 | Implement `VectorRepository` with LanceDB and SQLite FTS5 in `storage` | verify: cargo test -p founderos-storage | files: ~6
- T3 -> AC1,AC2 | Implement `runtime` with Ollama adapter and sidecar pre-approval | verify: cargo test -p founderos-runtime | files: ~5
- T4 -> AC2,AC5 | Implement `pipeline` with media processors (FFmpeg, pdfium, chunking, STT) | verify: cargo test -p founderos-pipeline | files: ~8
- T5 -> AC4,AC5 | Implement RRF query router and fusion engine | verify: cargo test -p founderos-pipeline | files: ~4
- T6 -> AC6,AC7 | Implement Ask FounderOS engine with chat modality detector and citations | verify: cargo test -p founderos-runtime | files: ~4
- T7 -> AC8 | Build desktop shell and Universal Search UI | verify: npm run build | files: ~8
