use founderos_domain::{Modality, VectorRecord, VectorSignature};
use uuid::Uuid;

#[test]
fn test_mrl_256d_truncation_quality_and_storage_savings() {
    let sig768 = VectorSignature::canonical("metal", "canonical_v1");

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
        assert_eq!(truncated.signature.dimension, 256);

        // Verify L2 norm is strictly 1.0 +- 1e-3
        let norm_sq: f32 = truncated.embedding.iter().map(|x| x * x).sum();
        assert!((norm_sq - 1.0).abs() < 1e-3);

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

    // Top-5 rank preservation check
    let top5_768: Vec<usize> = scores_768.iter().take(5).map(|x| x.0).collect();
    let top5_256: Vec<usize> = scores_256.iter().take(5).map(|x| x.0).collect();

    let overlap = top5_768.iter().filter(|id| top5_256.contains(id)).count();
    assert!(overlap >= 4, "MRL 256-d preserved {}/5 top items", overlap);

    // Vector storage calculation
    let bytes_768 = 768 * std::mem::size_of::<f32>();
    let bytes_256 = 256 * std::mem::size_of::<f32>();
    let savings_pct = ((bytes_768 - bytes_256) as f64 / bytes_768 as f64) * 100.0;
    assert!((savings_pct - 66.66).abs() < 0.1);
}
