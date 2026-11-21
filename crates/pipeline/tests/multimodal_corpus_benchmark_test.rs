use founderos_domain::{SearchQuery, VectorSignature};
use founderos_pipeline::{HybridRetriever, IngestionPipeline};
use founderos_runtime::DeterministicSidecarProvider;
use founderos_storage::{InMemoryVectorStore, SqliteStorage, VectorRepository};
use std::sync::Arc;
use std::time::Instant;

#[tokio::test]
async fn test_multimodal_corpus_benchmark_and_acceptance() {
    let sig = VectorSignature::canonical("metal", "canonical_v1");

    let storage = SqliteStorage::open_in_memory().unwrap();
    let vector_repo = Arc::new(InMemoryVectorStore::new(sig));
    let provider = Arc::new(DeterministicSidecarProvider::new("metal"));

    let pipeline = IngestionPipeline::new(
        storage.clone(),
        vector_repo.clone(),
        provider.clone(),
    );

    let start_ingest = Instant::now();

    // 1. Ingest 20 Text Documents with distinct topics
    let text_topics = [
        "Founding Cap Table and Convertible Note Agreement",
        "SOC2 Offline Privacy Compliance and Security Audit",
        "Product Launch Roadmap and Beta Customer Interviews",
        "Deep Tech AI Vector Search Engine Architecture",
        "Employee Stock Option Plan ESOP Allocation Guidelines",
        "Patent Filing on Multimodal Local Representation",
        "Quarterly Financial Statement Cash Runway Analysis",
        "Disaster Recovery and Local Backup Encryption Policy",
        "Marketing Strategy for Developer Tools Community",
        "Board of Directors Meeting Minutes for Seed Round",
        "Sales Playbook Enterprise SaaS Contract Terms",
        "Customer Support SLA and Issue Escalation Matrix",
        "Cloud Migration Fallback to Local Infrastructure",
        "Zero-Trust Access Control Hardware Key Authentication",
        "Open Source License Compliance Apache and MIT Rules",
        "Product Analytics Retention Curves and Cohort Report",
        "Hiring Plan Engineering Leadership and Staff Compensation",
        "Supplier Vendor Assessment and Risk Registry",
        "Corporate Governance Code of Ethics and Conduct",
        "Annual Operating Plan Budget and Capital Expenditure",
    ];

    for (idx, topic) in text_topics.iter().enumerate() {
        let i = idx + 1;
        let content = format!("Document #{i} Title: {topic}. Complete company records and operational details.");
        pipeline
            .ingest_text_document(
                format!("/data/docs/doc_{i:02}.txt"),
                format!("doc_{i:02}.txt"),
                &content,
            )
            .await
            .unwrap();
    }

    // 2. Ingest 20 Images (UI mockups, architecture diagrams, charts)
    for i in 1..=20 {
        let visual_label = match i {
            1 => "Blue navigation sidebar dashboard layout UI design",
            2 => "System architecture flowchart showing SQLite and LanceDB",
            3 => "Financial growth quarterly revenue chart graph",
            4 => "Team offsite group photo modern tech office",
            _ => "Visual design screenshot component card palette",
        };
        let dummy_bytes = format!("PNG_VISUAL_CONTENT_{i}").into_bytes();
        pipeline
            .ingest_image_asset(
                format!("/data/images/img_{i:02}.png"),
                format!("img_{i:02}.png"),
                &dummy_bytes,
                Some(format!("{visual_label} asset #{i}")),
            )
            .await
            .unwrap();
    }

    // 3. Ingest 5 Audio Recordings (with Needle Test in Audio #3)
    for i in 1..=5 {
        let segs = if i == 3 {
            vec![
                (0.0, 30.0, "Welcome everyone to the pricing sync.".to_string()),
                (440.0, 470.0, "Client Xerophylla Corp agreed to contract pricing of $43,820 after review.".to_string()),
                (550.0, 580.0, "Action items finalized for next week.".to_string()),
            ]
        } else {
            vec![(0.0, 30.0, format!("General team meeting discussion number {i} with staff"))]
        };
        let dummy_audio = format!("AUDIO_RAW_PCM_{i}").into_bytes();
        pipeline
            .ingest_audio_recording(
                format!("/data/audio/meeting_{i:02}.wav"),
                format!("meeting_{i:02}.wav"),
                600.0,
                &dummy_audio,
                &segs,
            )
            .await
            .unwrap();
    }

    // 4. Ingest 5 Video Recordings (with scene timestamps)
    for i in 1..=5 {
        let dummy_video_track = format!("VIDEO_SCENE_TRACK_{i}").into_bytes();
        let scene_segs = vec![
            (12.0, 24.0, format!("Keyframe scene change at 12s demoing product UI recording {i}")),
            (120.0, 140.0, format!("Keyframe scene change at 120s displaying analytics dashboard {i}")),
        ];
        pipeline
            .ingest_audio_recording(
                format!("/data/videos/demo_{i:02}.mp4"),
                format!("demo_{i:02}.mp4"),
                300.0,
                &dummy_video_track,
                &scene_segs,
            )
            .await
            .unwrap();
    }

    let ingest_duration = start_ingest.elapsed();
    let total_vectors = vector_repo.count().await.unwrap();
    assert!(total_vectors >= 50, "Should index at least 50 multimodal items");

    // 5. Run Labeled Queries & Evaluate Metrics
    let retriever = HybridRetriever::new(storage, vector_repo, provider);

    let test_cases = vec![
        ("Xerophylla pricing", "meeting_03.wav"),
        ("Founding Cap Table Convertible", "doc_01.txt"),
        ("SOC2 Offline Privacy Compliance", "doc_02.txt"),
        ("Product Launch Roadmap Beta", "doc_03.txt"),
        ("Blue navigation sidebar dashboard", "img_01.png"),
        ("architecture flowchart showing SQLite", "img_02.png"),
    ];

    let mut reciprocal_ranks = Vec::new();
    let mut total_latency_ms = 0.0;

    for (query_str, expected_file) in &test_cases {
        let q_start = Instant::now();
        let query = SearchQuery::new_text(*query_str);
        let results = retriever.search(&query).await.unwrap();
        let elapsed_ms = q_start.elapsed().as_secs_f64() * 1000.0;
        total_latency_ms += elapsed_ms;

        assert!(!results.is_empty(), "Query '{query_str}' must return results");
        let rank = results.iter().position(|r| r.file_name == *expected_file).map(|idx| idx + 1);

        println!("Query: '{query_str}' -> Expected: '{expected_file}', Top: '{}', Rank: {:?}",
            results.first().map(|r| r.file_name.as_str()).unwrap_or("none"), rank);

        if let Some(r) = rank {
            reciprocal_ranks.push(1.0 / (r as f32));
        } else {
            reciprocal_ranks.push(0.0);
        }
    }

    let mrr: f32 = reciprocal_ranks.iter().sum::<f32>() / (test_cases.len() as f32);
    let avg_latency_ms = total_latency_ms / (test_cases.len() as f64);
    let recall_at_5 = reciprocal_ranks.iter().filter(|&&r| r >= 0.2).count() as f32 / (test_cases.len() as f32);

    println!("=== Multimodal Corpus Benchmark Report ===");
    println!("Total Items: 50 (20 docs, 20 images, 5 audio, 5 video)");
    println!("Total Indexed Vectors: {total_vectors}");
    println!("Ingestion Time: {:.2?}", ingest_duration);
    println!("Average Search Latency: {avg_latency_ms:.2} ms (budget < 150ms)");
    println!("Recall@5: {recall_at_5:.3} (target >= 0.80)");
    println!("Mean Reciprocal Rank (MRR): {mrr:.3} (target >= 0.80)");

    assert!(avg_latency_ms < 150.0, "Latency must be under 150ms");
    assert!(recall_at_5 >= 0.80, "Recall@5 must be at or above target 0.80");
    assert!(mrr >= 0.80, "MRR must be at or above target 0.80");
}
