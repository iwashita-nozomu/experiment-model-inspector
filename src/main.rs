use experiment_model_inspector::{
    SourceSlice, VERSION, inspect, inspect_candidate, observe_sources, rpc,
};
use serde_json::json;
use std::{
    io::{self, BufRead, Read, Write},
    path::Path,
};

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    match a.first().map(String::as_str) {
        Some("mcp") if a.len() == 1 => serve(),
        Some("inspect") if a.len() == 3 => match inspect(Path::new(&a[1]), &a[2]) {
            Ok(v) => println!("{}", serde_json::to_string_pretty(&v).unwrap()),
            Err(e) => {
                println!("{}", json!({"schema_version":"1.0","error":e}));
                std::process::exit(1);
            }
        },
        Some("observe") if a.len() == 5 => {
            let range = a[3]
                .parse()
                .and_then(|start_line| a[4].parse().map(|end_line| (start_line, end_line)));
            let result = match range {
                Ok((start_line, end_line)) => observe_sources(
                    Path::new(&a[1]),
                    &[SourceSlice {
                        file: a[2].clone(),
                        start_line,
                        end_line,
                    }],
                ),
                Err(_) => Err("line_range_limit".into()),
            };
            emit(result);
        }
        Some("inspect-candidate") if a.len() == 2 => {
            let mut raw = Vec::new();
            let result = match io::stdin().take(262_145).read_to_end(&mut raw) {
                Err(_) => Err("read_failed".into()),
                Ok(_) if raw.len() > 262_144 => Err("candidate_size_limit".into()),
                Ok(_) => match serde_json::from_slice(&raw) {
                    Ok(candidate) => inspect_candidate(Path::new(&a[1]), candidate),
                    Err(_) => Err("invalid_manifest".into()),
                },
            };
            emit(result);
        }
        Some("--version") => println!("experiment-model-inspector {VERSION}"),
        _ => {
            eprintln!(
                "Usage: experiment-model-inspector inspect ROOT MANIFEST.json | observe ROOT SOURCE START END | inspect-candidate ROOT < candidate.json | mcp | --version"
            );
            std::process::exit(2);
        }
    }
}

fn emit(result: Result<serde_json::Value, String>) {
    match result {
        Ok(v) => println!("{}", serde_json::to_string_pretty(&v).unwrap()),
        Err(e) => {
            println!("{}", json!({"schema_version":"1.0","error":e}));
            std::process::exit(1);
        }
    }
}

fn serve() {
    let mut input = io::stdin().lock();
    let mut out = io::stdout().lock();
    loop {
        let mut buf = Vec::new();
        // Newline-delimited stdio MCP; at most 1 MiB per request.
        match input.by_ref().take(1_048_577).read_until(b'\n', &mut buf) {
            Ok(0) | Err(_) => break,
            _ => {}
        }
        if buf.len() > 1_048_576 {
            break;
        }
        let response = match serde_json::from_slice(&buf) {
            Ok(v) => rpc(v),
            Err(_) => Some(
                json!({"jsonrpc":"2.0","id":null,"error":{"code":-32700,"message":"Parse error"}}),
            ),
        };
        if let Some(v) = response
            && writeln!(out, "{v}").and_then(|_| out.flush()).is_err()
        {
            break;
        }
    }
}
