use founderos_domain::{AssetStatus, Modality, VectorSignature};
use founderos_pipeline::{DocumentParser, IngestionPipeline, ParsedDocument, ParsedPage};
use founderos_runtime::SidecarEmbeddingAdapter;
use founderos_storage::{InMemoryVectorStore, SqliteStorage};
use std::sync::Arc;

#[tokio::test]
async fn test_video_and_multipage_document_ingestion() {
    let temp_db = std::env::temp_dir().join(format!("founderos_ingest_test_{}.db", std::process::id()));
    let _ = std::fs::remove_file(&temp_db);

    let storage = SqliteStorage::open(temp_db.to_str().unwrap()).unwrap();
    let sig = VectorSignature {
        model_tag: "embeddinggemma-2:740m".into(),
        quantization: "f32".into(),
        prefix_config: "canonical".into(),
        dimension: 768,
    };
    let vector_repo = Arc::new(InMemoryVectorStore::new(sig));
    let embedding_adapter = Arc::new(SidecarEmbeddingAdapter::new("sidecar".into(), "embeddinggemma-2:740m".into()));
    let pipeline = IngestionPipeline::new(storage.clone(), vector_repo.clone(), embedding_adapter);

    // 1. Ingest multi-page document
    let doc = ParsedDocument {
        file_name: "board_deck.pdf".to_string(),
        pages: vec![
            ParsedPage {
                page_number: 1,
                text: "Executive Summary: $12M ARR with 140% net revenue retention.".into(),
            },
            ParsedPage {
                page_number: 2,
                text: "Product Vision: Autonomous founder operations powered by local AI.".into(),
            },
        ],
    };

    let doc_asset = pipeline
        .ingest_document_asset("docs/board_deck.pdf".into(), "application/pdf".into(), &doc)
        .await
        .unwrap();
    assert_eq!(doc_asset.file_name, "board_deck.pdf");
    assert_eq!(doc_asset.status, AssetStatus::Indexed);

    let doc_chunks = storage.list_chunks_for_asset(doc_asset.id).unwrap();
    assert_eq!(doc_chunks.len(), 2);
    assert_eq!(doc_chunks[0].page_number, Some(1));
    assert_eq!(doc_chunks[1].page_number, Some(2));

    // 2. Ingest video asset with scene keyframes and audio track
    let keyframes = vec![
        (0.0, vec![10u8; 64]),
        (4.5, vec![20u8; 64]),
        (12.3, vec![30u8; 64]),
    ];
    let transcripts = vec![
        (0.0, 5.0, "Welcome everyone to the sprint review.".into()),
        (5.0, 15.0, "Today we demonstrate multimodal video indexing.".into()),
    ];
    let audio_bytes = vec![0u8; 256];

    let video_asset = pipeline
        .ingest_video_asset(
            "media/sprint_demo.mp4".into(),
            "sprint_demo.mp4".into(),
            &keyframes,
            Some((&audio_bytes, &transcripts, 15.0)),
        )
        .await
        .unwrap();

    assert_eq!(video_asset.file_name, "sprint_demo.mp4");
    assert_eq!(video_asset.status, AssetStatus::Indexed);
    let video_chunks = storage.list_chunks_for_asset(video_asset.id).unwrap();
    assert!(!video_chunks.is_empty());

    // Check keyframe chunks have timestamps
    let visual_chunks: Vec<_> = video_chunks
        .iter()
        .filter(|c| c.modality == Modality::Video)
        .collect();
    assert_eq!(visual_chunks.len(), 3);
    assert_eq!(visual_chunks[0].timestamp_start, Some(0.0));
    assert_eq!(visual_chunks[1].timestamp_start, Some(4.5));
    assert_eq!(visual_chunks[2].timestamp_start, Some(12.3));

    // Clean up
    let _ = std::fs::remove_file(&temp_db);
}

#[test]
fn test_document_parser_plain_fallback() {
    let temp_file = std::env::temp_dir().join(format!("test_notes_{}.txt", std::process::id()));
    std::fs::write(&temp_file, "Note 1: Discuss customer contract.\nNote 2: Sign NDA.").unwrap();

    let parser = DocumentParser;
    let parsed = parser.parse_file(&temp_file).unwrap();

    assert_eq!(parsed.pages.len(), 1);
    assert!(parsed.pages[0].text.contains("customer contract"));

    let _ = std::fs::remove_file(&temp_file);
}
