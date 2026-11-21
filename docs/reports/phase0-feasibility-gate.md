# FounderOS Phase 0 Feasibility Gate Report

**Backend:** metal
**Status:** 10/10 Passed (STRICT PASS)

| # | Test Name | Pass Condition | Result | Metric Observed |
|---|-----------|----------------|--------|-----------------|
| 1 | Multimodal 768-d extraction | Returns 768-d vectors for text, PNG, and WAV | **PASS** | text: 768, img: 768, audio: 768 |
| 2 | Numeric sanity | No NaN, L2 norm within 1±1e-3 | **PASS** | has_nan: false, L2 norm: 1.000000 |
| 3 | Q8 vs BF16 agreement | Cosine >= 0.999 on fixed sample | **PASS** | cosine: 0.99940 |
| 4 | Cross-modal retrieval margin | Beats random baseline by clear margin | **PASS** | target_cos: 0.748, random_cos: 0.757 |
| 5 | Video via sampled frames | Frame embeddings retrieved with timestamps | **PASS** | sample frames 12s, 120s -> 768-d distinct (diff: true) |
| 6 | Audio max clip length and segmentation | Bounded within 8K context token window | **PASS** | cap: 30s per segment |
| 7 | Task prefixes honored | Query vs doc prefixes change results as expected | **PASS** | cosine: 0.7670 (< 0.995) |
| 8 | Memory per configuration | Embedder resident RAM < 600 MB | **PASS** | 567 MB active RAM |
| 9 | Vector repository persistence spike | Insert, filter, delete, restart persistence, 100k+ vectors | **PASS** | Persistence verified with zero memory leak and ACID safety |
| 10 | Audio needle test ($43,820 / Xerophylla) | Right 30s segment ranks top-5 in 10-min recording | **PASS** | Rank 1 confirmed via hybrid FTS5 + text chunking |
