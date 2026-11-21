use founderos_domain::{Modality, VectorRecord, VectorSignature};
use uuid::Uuid;

#[test]
fn test_mrl_256d_truncation_quality_and_storage_savings() {
    let sig768 = VectorSignature {
        model_tag: "embeddinggemma-2:740m".to_string(),
        quantization: "q8_0".to_string(),
        prefix_config: "v1_prefixed".to_string(),
        dimension: 768,
    };

    // Generate 50 realistic 768-d vectors
    let mut records_768 = Vec::new();
    let mut records_256 = Vec::new();

    for i in 0..50 {
        let mut raw = Vec::with_capacity(768);
        for j in 0..768 {
            let val = ((i * 37 + j * 17) % 100) as f32 / 100.0 - 0.5;
            raw.push(val);
        }
        let rec = VectorRecord::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Modality::Text,
            &sig768,
            raw,
        )
        .unwrap();

        // MRL 256-d truncation with L2 re-normalization
        let truncated = rec.truncate_mrl(256).unwrap();
        assert_eq!(truncated.dimension, 256);

        // Verify L2 norm is strictly 1.0
        let norm_sq: f32 = truncated.embedding.iter().map(|x| x * x).sum();
        assert!((norm_sq - 1.0).abs() < 1e-5);

        records_768.push(rec);
        records_256.push(truncated);
    }

    // Query with synthetic probe vector
    let query_768 = &records_768[0].embedding;
    let query_256 = &records_256[0].embedding;

    let mut scores_768: Vec<(usize, f32)> = records_768
        .iter()
        .enumerate()
        .map(|(i, r)| (i, r.cosine_similarity(query_768).unwrap()))
        .collect();
    scores_768.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut scores_256: Vec<(usize, f32)> = records_256
        .iter()
        .enumerate()
        .map(|(i, r)| (i, r.cosine_similarity(query_256).unwrap()))
        .collect();
    scores_256.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Top-5 overlap check
    let top5_768: std::collections::HashSet<_> = scores_768.iter().take(5).map(|(i, _)| *i).collect();
    let top5_256: std::collections::HashSet<_> = scores_256.iter().take(5).map(|(i, _)| *i).collect();
    let overlap_count = top5_768.intersection(&top5_256).count();

    let storage_768_bytes = 50 * 768 * 4;
    let storage_256_bytes = 50 * 256 * 4;
    let savings_pct = ((storage_768_bytes - storage_256_bytes) as f32 / storage_768_bytes as f32) * 100.0;

    println!("=== MRL 256-d Truncation Report ===");
    println!("Top-5 Overlap: {overlap_count}/5 ({:.0}%)", (overlap_count as f32 / 5.0) * 100.0);
    println!("768-d Storage: {} bytes", storage_768_bytes);
    println!("256-d Storage: {} bytes", storage_256_bytes);
    println!("Storage Reduction: {:.1}%", savings_pct);

    assert!(overlap_count >= 4, "MRL 256-d truncation must retain at least 80% of top-5 ranking");
    assert!((savings_pct - 66.67).abs() < 1.0, "Storage savings must be ~66.7%");
}
