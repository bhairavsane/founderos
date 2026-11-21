use founderos_domain::{Citation, CitationType};
use founderos_runtime::OllamaChatAdapter;
use uuid::Uuid;

#[test]
fn test_multimodal_heuristic_detection() {
    assert!(OllamaChatAdapter::check_multimodal_heuristic("llava:latest"));
    assert!(OllamaChatAdapter::check_multimodal_heuristic("llama3.2-vision:11b"));
    assert!(OllamaChatAdapter::check_multimodal_heuristic("minicpm-v:8b"));
    assert!(!OllamaChatAdapter::check_multimodal_heuristic("qwen2.5:7b"));
    assert!(!OllamaChatAdapter::check_multimodal_heuristic("llama3:8b"));
}

#[test]
fn test_citation_native_vs_derived() {
    let native_cite = Citation {
        asset_id: Uuid::new_v4(),
        file_path: "/data/mockup.png".to_string(),
        file_name: "mockup.png".to_string(),
        citation_type: CitationType::Native,
        evidence_detail: "Direct 768-d vision embedding match".to_string(),
        page_number: None,
        timestamp_start: None,
        timestamp_end: None,
    };
    let derived_cite = Citation {
        asset_id: Uuid::new_v4(),
        file_path: "/data/meeting.wav".to_string(),
        file_name: "meeting.wav".to_string(),
        citation_type: CitationType::Derived,
        evidence_detail: "Whisper STT token index".to_string(),
        page_number: None,
        timestamp_start: Some(120.0),
        timestamp_end: Some(150.0),
    };

    let f1 = native_cite.format_citation();
    let f2 = derived_cite.format_citation();

    assert!(f1.contains("Type: Native"));
    assert!(f2.contains("Type: Derived"));
    assert!(f2.contains("02:00 - 02:30"));
}
