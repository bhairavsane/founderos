use founderos_domain::{SearchQuery, VectorSignature};
use founderos_pipeline::{HybridRetriever, IngestionPipeline};
use founderos_runtime::DeterministicSidecarProvider;
use founderos_storage::{InMemoryVectorStore, SqliteStorage};
use std::sync::Arc;

#[tokio::test]
async fn test_needle_and_hybrid_search() {
    let sig = VectorSignature::canonical("metal", "canonical_v1");

    let storage = SqliteStorage::open_in_memory().unwrap();
    let vector_repo = Arc::new(InMemoryVectorStore::new(sig));
    let provider = Arc::new(DeterministicSidecarProvider::new("metal"));

    let pipeline = IngestionPipeline::new(
        storage.clone(),
        vector_repo.clone(),
        provider.clone(),
    );

    // Ingest text doc
    pipeline
        .ingest_text_document(
            "/workspace/docs/financial_plan.txt".to_string(),
            "financial_plan.txt".to_string(),
            "FounderOS Q3 budget allocation for deep tech R&D and team hiring.",
        )
        .await
        .unwrap();

    // Ingest 10-minute audio meeting with the needle test payload
    let transcript_segments = vec![
        (0.0, 30.0, "Welcome everyone to the Monday sync.".to_string()),
        (440.0, 470.0, "Client Xerophylla Corp agreed to contract pricing of $43,820 after review.".to_string()),
        (550.0, 580.0, "Let us wrap up today's discussion.".to_string()),
    ];
    let dummy_audio = vec![0u8; 1024];
    pipeline
        .ingest_audio_recording(
            "/workspace/audio/client_call_10min.wav".to_string(),
            "client_call_10min.wav".to_string(),
            600.0, // 10 minutes
            &dummy_audio,
            &transcript_segments,
        )
        .await
        .unwrap();

    let retriever = HybridRetriever::new(storage, vector_repo, provider);

    // Query needle
    let query = SearchQuery::new_text("Xerophylla pricing");
    let results = retriever.search(&query).await.unwrap();

    assert!(!results.is_empty(), "Should retrieve results for needle query");
    let top = &results[0];
    assert_eq!(top.file_name, "client_call_10min.wav");
    assert!(top.snippet.contains("$43,820"));
}
