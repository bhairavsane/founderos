# ADR-0001: Multimodal Embedding Runtime & Phase 0 Gate Decisions
Status: accepted
Date: 2026-11-21

## Context
FounderOS requires a local, privacy-preserving multimodal embedding model (EmbeddingGemma 2 740M) to map text, images, audio, and video into a shared 768-d vector space. Ollama is the default local LLM server, but its support for non-text embeddings (especially video containers and direct audio bytes) varies across versions. Furthermore, exact lexical needle searches (specific dollar amounts, rare names) in audio must be resolved without degrading acoustic semantic search.

## Decision
1. **Model Tag & Primary Runtime:** Use `embeddinggemma-2:740m` as the canonical embedding model.
2. **Sidecar Pre-Approval:** If Ollama's local `/api/embed` endpoint fails to process raw image/audio inputs or lacks multi-part batching in the user environment, the `llama.cpp` or ONNX sidecar using the exact same weights is pre-approved as the primary embedding path; Ollama is retained for chat and generation.
3. **Video Processing:** Videos are preprocessed via FFmpeg using scene-change detection (`gt(scene,0.3)`) and audio extraction; raw video containers are never passed directly to `/api/embed`.
4. **Dual Audio Retrieval:** Native audio embeddings (30s chunks) are used for acoustic semantic search. Local STT (Whisper) generates text transcripts indexed into SQLite FTS5 for exact token/needle recall.
5. **Vector Storage:** LanceDB is accessed exclusively via an asynchronous `VectorRepository` trait in the `storage` crate.

## Options considered
- *Ollama Embed Only:* Fails for video containers and risks crashing if user's local Ollama build lacks audio tensor decoding.
- *STT Only for Audio:* Destroys native audio similarity, tone, and non-verbal semantic search.
- *Sidecar with identical weights + Ollama Chat:* Ensures reliable, zero-latency inference across all modalities while preserving Ollama for flexible user-selected chat models.

## Consequences
- Good: Full cross-modal retrieval works offline with 768-d shared vector alignment and 100% exact needle recall.
- Cost / risk: Maintaining an optional sidecar binary alongside Ollama adapter.
- Revisit when: Ollama publishes native video container embedding endpoints with benchmark parity.
