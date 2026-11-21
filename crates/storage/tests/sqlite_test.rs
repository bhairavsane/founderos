use founderos_domain::{Asset, Chunk};
use founderos_storage::SqliteStorage;

#[test]
fn test_sqlite_fts5_needle_search() {
    let storage = SqliteStorage::open_in_memory().unwrap();
    let asset = Asset::new(
        "/data/meeting.mp3".to_string(),
        "meeting.mp3".to_string(),
        "audio/mp3".to_string(),
        1024,
        "hash123".to_string(),
    );
    storage.insert_asset(&asset).unwrap();

    let chunk = Chunk::new_audio_segment(
        asset.id,
        1,
        440.0,
        470.0,
        Some("Client Xerophylla Corp agreed to contract pricing of $43,820".to_string()),
    );
    storage.insert_chunk(&chunk).unwrap();

    let results = storage.search_fts("Xerophylla", 5).unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].0, chunk.id);
    assert!(results[0].3.contains("$43,820"));
}
