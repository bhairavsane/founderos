use founderos_runtime::FeasibilityGateRunner;
use std::fs;
use std::path::PathBuf;

#[tokio::test]
async fn test_phase0_feasibility_gate_all_ten_tests() {
    let report = FeasibilityGateRunner::run_suite("metal")
        .await
        .expect("Feasibility gate suite failed to execute");

    println!("=== Phase 0 Feasibility Gate Results ===");
    println!("Backend: {}", report.backend);
    println!("Passed: {}/{}", report.passed_count, report.total_tests);

    for res in &report.results {
        println!(
            "Test #{}: {} -> [{}] (Observed: {})",
            res.test_id,
            res.name,
            if res.passed { "PASS" } else { "FAIL" },
            res.metric_observed
        );
        assert!(res.passed, "Test #{} ({}) must pass", res.test_id, res.name);
    }

    assert_eq!(report.passed_count, 10, "All 10 feasibility tests must pass");

    // Write machine-readable report
    let reports_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("docs")
        .join("reports");
    fs::create_dir_all(&reports_dir).unwrap();

    let json_path = reports_dir.join("phase0-feasibility-gate.json");
    let json_content = serde_json::to_string_pretty(&report).unwrap();
    fs::write(&json_path, json_content).unwrap();

    // Write markdown report
    let mut md = String::new();
    md.push_str("# FounderOS Phase 0 Feasibility Gate Report\n\n");
    md.push_str(&format!("**Backend:** {}\n", report.backend));
    md.push_str(&format!("**Status:** {}/{} Passed (STRICT PASS)\n\n", report.passed_count, report.total_tests));
    md.push_str("| # | Test Name | Pass Condition | Result | Metric Observed |\n");
    md.push_str("|---|-----------|----------------|--------|-----------------|\n");
    for res in &report.results {
        md.push_str(&format!(
            "| {} | {} | {} | **{}** | {} |\n",
            res.test_id,
            res.name,
            res.pass_condition,
            if res.passed { "PASS" } else { "FAIL" },
            res.metric_observed
        ));
    }

    let md_path = reports_dir.join("phase0-feasibility-gate.md");
    fs::write(&md_path, md).unwrap();
}
