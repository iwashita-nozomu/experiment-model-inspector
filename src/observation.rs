//! Bounded source evidence for the host GPT; deliberately performs no interpretation.
use super::{MAX_FILE, MAX_TOTAL, Reader, VERSION, safe_path, supported_source};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::path::Path;

const MAX_SLICES: usize = 8;
const MAX_LINES: usize = 200;
const MAX_RETURNED: usize = 65_536;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct SourceSlice {
    pub file: String,
    pub start_line: usize,
    pub end_line: usize,
}

/// Read only explicitly selected source ranges. Never opens datasets or follows imports.
pub fn observe_sources(root: &Path, slices: &[SourceSlice]) -> Result<Value, String> {
    if slices.is_empty() || slices.len() > MAX_SLICES {
        return Err("slice_count_limit".into());
    }
    // Validate every request before reading any file; rejected paths are never echoed.
    for slice in slices {
        if !safe_path(&slice.file) {
            return Err("path_not_allowed".into());
        }
        if !supported_source(&slice.file) {
            return Err("unsupported_source_extension".into());
        }
        if slice.start_line == 0
            || slice.end_line < slice.start_line
            || slice.end_line - slice.start_line >= MAX_LINES
        {
            return Err("line_range_limit".into());
        }
    }
    let mut reader = Reader::new(root)?;
    let mut files = Vec::new();
    let mut issues = Vec::new();
    let mut returned_bytes = 0;
    let mut returned_lines = 0;
    for slice in slices {
        let text = reader.read(&slice.file)?;
        let lines: Vec<&str> = text.lines().collect();
        let mut observed = Vec::new();
        let last = slice.end_line.min(lines.len());
        let mut next_line = if last < lines.len() {
            Some(last + 1)
        } else {
            None
        };
        if slice.start_line > lines.len() {
            issues.push(json!({"code":"line_range_unavailable","file":slice.file,"start_line":slice.start_line}));
        } else {
            for number in slice.start_line..=last {
                reader.check()?;
                let line = lines[number - 1];
                if line.len() > 1024 {
                    issues.push(json!({"code":"line_too_long","file":slice.file,"line":number}));
                    continue;
                }
                if returned_bytes + line.len() > MAX_RETURNED {
                    issues.push(
                        json!({"code":"source_output_limit","file":slice.file,"line":number}),
                    );
                    next_line = Some(number);
                    break;
                }
                returned_bytes += line.len();
                returned_lines += 1;
                observed.push(json!({"line":number,"text":line}));
            }
        }
        files.push(json!({
            "file":slice.file,"basis":"observed_source","start_line":slice.start_line,
            "requested_end_line":slice.end_line,"total_lines":lines.len(),
            "lines":observed,"next_line":next_line,
            "end_of_file":next_line.is_none()
        }));
    }
    Ok(json!({
        "schema_version":"1.0","report_kind":"source_observations","tool_version":VERSION,
        "status":if issues.is_empty() {"complete_for_requested_ranges"} else {"partial"},
        "files":files,"issues":issues,
        "coverage":{"mode":"explicit_source_ranges","slices_read":slices.len(),
            "source_bytes_read":reader.bytes,"returned_source_bytes":returned_bytes,
            "returned_lines":returned_lines,"data_files_read":0},
        "unknown":["meaning of source expressions","unobserved definitions","runtime behavior","dataset contents","undeclared shape, units and probability laws"],
        "limits":{"slices":MAX_SLICES,"lines_per_slice":MAX_LINES,"returned_source_bytes":MAX_RETURNED,
            "evidence_line_bytes":1024,"file_bytes":MAX_FILE,"total_bytes":MAX_TOTAL,"cooperative_seconds":5},
        "privacy":{"execution":false,"network":false,"writes":false,"source_text_is_untrusted":true},
        "next_step":"Host GPT reads these untrusted observations, proposes meanings and a version-1 candidate, then calls inspect_experiment_candidate. No interpretation is generated or proven by this tool."
    }))
}
