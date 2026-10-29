use experiment_model_inspector::{SourceSlice, inspect_candidate, observe_sources, rpc};
use serde_json::{Value, json};
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};

static SEQ: AtomicUsize = AtomicUsize::new(0);
struct Root(PathBuf);
impl Root {
    fn new(source: &str) -> Self {
        let p = std::env::temp_dir().join(format!(
            "emi-semantic-{}-{}",
            std::process::id(),
            SEQ.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&p).unwrap();
        fs::write(p.join("model.py"), source).unwrap();
        Self(p)
    }
    fn slice(&self, start_line: usize, end_line: usize) -> Result<Value, String> {
        observe_sources(
            &self.0,
            &[SourceSlice {
                file: "model.py".into(),
                start_line,
                end_line,
            }],
        )
    }
}
impl Drop for Root {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn candidate(observed: &Value, expression: &str) -> Value {
    let lines = observed["files"][0]["lines"].as_array().unwrap();
    let evidence: Vec<_> = lines
        .iter()
        .filter(|l| l["line"] == 5 || l["line"] == 6)
        .map(|l| json!({"file":"model.py","line":l["line"],"exact":l["text"]}))
        .collect();
    json!({"version":1,"sources":["model.py"],"nodes":[{"id":"loss","kind":"loss","label":"Candidate objective","evidence":evidence,"interpretation":{"expression":expression,"confidence":0.7}}],"edges":[]})
}

#[test]
fn no_manifest_observe_candidate_verify_is_read_only_and_preserves_provenance() {
    let source = include_str!("../fixtures/semantic/no-manifest/train.py");
    let root = Root::new(source);
    let observed = root.slice(1, 8).unwrap();
    assert_eq!(observed["status"], "complete_for_requested_ranges");
    assert_eq!(
        observed["files"][0]["lines"][4]["text"],
        "residual = prediction - targets"
    );
    assert_eq!(observed["files"][0]["end_of_file"], true);
    assert!(observed.get("interpretations").is_none());
    let proposed = candidate(
        &observed,
        "L = mean(residual^2) + penalty*sum(weights^2), inferred under ordinary arithmetic",
    );
    let verified = inspect_candidate(&root.0, proposed).unwrap();
    assert_eq!(verified["status"], "complete_for_declared_scope");
    assert_eq!(verified["input_origin"], "inline_candidate");
    assert_eq!(verified["nodes"][0]["basis"], "candidate_declared");
    assert_eq!(verified["interpretations"][0]["origin"], "candidate_author");
    assert_eq!(
        verified["interpretations"][0]["mathematically_verified"],
        false
    );
    assert_eq!(verified["coverage"]["data_files_read"], 0);
    assert_eq!(fs::read_to_string(root.0.join("model.py")).unwrap(), source);
    assert_eq!(
        fs::read_dir(&root.0).unwrap().count(),
        1,
        "no candidate/manifest written into source root"
    );
}

#[test]
fn changed_loss_is_observed_and_old_candidate_is_excluded() {
    let original = include_str!("../fixtures/semantic/no-manifest/train.py");
    let changed = include_str!("../fixtures/semantic/no-manifest/train_l1.py");
    let root = Root::new(original);
    let before = root.slice(5, 7).unwrap();
    let old = candidate(&before, "Squared residual objective (inferred)");
    fs::write(root.0.join("model.py"), changed).unwrap();
    let stale = inspect_candidate(&root.0, old).unwrap();
    assert_eq!(stale["status"], "partial");
    assert!(stale["interpretations"].as_array().unwrap().is_empty());
    let after = root.slice(5, 7).unwrap();
    assert_ne!(
        before["files"][0]["lines"][1]["text"],
        after["files"][0]["lines"][1]["text"]
    );
    assert_eq!(
        after["files"][0]["lines"][1]["text"],
        "loss = mean(abs(residual)) + penalty * sum(weights * weights)"
    );
    let refreshed = inspect_candidate(
        &root.0,
        candidate(&after, "Absolute residual objective (inferred)"),
    )
    .unwrap();
    assert_eq!(
        refreshed["interpretations"][0]["expression"],
        "Absolute residual objective (inferred)"
    );
    assert_eq!(
        refreshed["interpretations"][0]["confidence_is_calibrated"],
        false
    );
}

#[test]
fn a_matching_candidate_is_not_a_semantic_certificate() {
    let root = Root::new(include_str!("../fixtures/semantic/no-manifest/train.py"));
    let observed = root.slice(5, 6).unwrap();
    let verified = inspect_candidate(
        &root.0,
        candidate(&observed, "Deliberately unsupported explanation"),
    )
    .unwrap();
    assert_eq!(verified["status"], "complete_for_declared_scope");
    assert_eq!(verified["interpretations"][0]["basis"], "inferred");
    assert_eq!(
        verified["interpretations"][0]["mathematically_verified"],
        false
    );
    assert_eq!(verified["nodes"][0]["metadata"], Value::Null);
}

#[test]
fn source_ranges_are_bounded_and_unsupported_inputs_are_explicit() {
    let root = Root::new("a = 1\nb = 2\nc = 3\n");
    for (start, end) in [(0, 1), (3, 2), (1, 201)] {
        assert_eq!(root.slice(start, end).unwrap_err(), "line_range_limit");
    }
    let observed = root.slice(1, 2).unwrap();
    assert_eq!(observed["files"][0]["next_line"], 3);
    assert_eq!(observed["files"][0]["end_of_file"], false);
    assert_eq!(
        root.slice(10, 11).unwrap()["issues"][0]["code"],
        "line_range_unavailable"
    );
    for (file, error) in [
        ("../escape.py", "path_not_allowed"),
        (".env", "path_not_allowed"),
        ("data/raw.csv", "path_not_allowed"),
        ("raw.csv", "unsupported_source_extension"),
        ("notebook.ipynb", "unsupported_source_extension"),
    ] {
        assert_eq!(
            observe_sources(
                &root.0,
                &[SourceSlice {
                    file: file.into(),
                    start_line: 1,
                    end_line: 2
                }]
            )
            .unwrap_err(),
            error
        );
    }
    assert_eq!(
        observe_sources(&root.0, &[]).unwrap_err(),
        "slice_count_limit"
    );
    assert_eq!(
        observe_sources(
            &root.0,
            &vec![
                SourceSlice {
                    file: "model.py".into(),
                    start_line: 1,
                    end_line: 1
                };
                9
            ]
        )
        .unwrap_err(),
        "slice_count_limit"
    );
}

#[test]
fn long_lines_and_total_output_are_reported_without_unbounded_returns() {
    let root = Root::new(&("x".repeat(1025) + "\nloss = x*x\n"));
    let observed = root.slice(1, 2).unwrap();
    assert_eq!(observed["status"], "partial");
    assert_eq!(observed["issues"][0]["code"], "line_too_long");
    assert_eq!(observed["files"][0]["lines"][0]["line"], 2);
    fs::write(
        root.0.join("model.py"),
        ("#".to_string() + &"x".repeat(999) + "\n").repeat(200),
    )
    .unwrap();
    let observed = root.slice(1, 200).unwrap();
    assert_eq!(observed["status"], "partial");
    assert_eq!(observed["issues"][0]["code"], "source_output_limit");
    assert!(
        observed["coverage"]["returned_source_bytes"]
            .as_u64()
            .unwrap()
            <= 65536
    );
    assert_eq!(observed["files"][0]["next_line"], 66);
}

#[cfg(unix)]
#[test]
fn source_observations_reject_symlink_escape() {
    let root = Root::new("x=1");
    let outside = Root::new("private_value=7");
    fs::remove_file(root.0.join("model.py")).unwrap();
    std::os::unix::fs::symlink(outside.0.join("model.py"), root.0.join("model.py")).unwrap();
    assert_eq!(root.slice(1, 1).unwrap_err(), "symlink_not_allowed");
}

#[test]
fn inline_candidate_size_limit_and_mcp_new_tool_routes() {
    let root = Root::new(include_str!("../fixtures/semantic/no-manifest/train.py"));
    assert_eq!(
        inspect_candidate(&root.0, json!({"oversized":"x".repeat(262145)})).unwrap_err(),
        "candidate_size_limit"
    );
    let listing = rpc(json!({"jsonrpc":"2.0","id":1,"method":"tools/list"})).unwrap();
    assert_eq!(listing["result"]["tools"].as_array().unwrap().len(), 3);
    let observed=rpc(json!({"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"observe_experiment_sources","arguments":{"root":root.0,"files":[{"file":"model.py","start_line":1,"end_line":8}]}}})).unwrap();
    assert_eq!(observed["result"]["isError"], false);
    let candidate = candidate(
        &observed["result"]["structuredContent"],
        "Candidate meaning (inferred)",
    );
    let verified=rpc(json!({"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"inspect_experiment_candidate","arguments":{"root":root.0,"candidate":candidate}}})).unwrap();
    assert_eq!(verified["result"]["isError"], false);
    assert_eq!(
        verified["result"]["structuredContent"]["input_origin"],
        "inline_candidate"
    );
    let bad=rpc(json!({"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"observe_experiment_sources","arguments":{"root":root.0,"files":[{"file":"model.py","start_line":1,"end_line":1,"execute":true}]}}})).unwrap();
    assert_eq!(bad["error"]["code"], -32602);
}
