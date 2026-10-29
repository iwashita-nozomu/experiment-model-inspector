"""End-to-end stdio MCP and CLI checks using synthetic fixtures only."""
import json
import pathlib
import subprocess
import shutil
import tempfile
import sys

binary = pathlib.Path(sys.argv[1]).resolve()
root = pathlib.Path(__file__).resolve().parents[1]
fixture = root / "fixtures" / "learning"
requests = [
    {"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2025-06-18", "capabilities": {}, "clientInfo": {"name": "smoke", "version": "1"}}},
    {"jsonrpc": "2.0", "method": "notifications/initialized"},
    {"jsonrpc": "2.0", "id": 2, "method": "tools/list"},
    {"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "inspect_experiment", "arguments": {"root": str(fixture), "manifest": "experiment-model.json"}}},
    {"jsonrpc": "2.0", "id": 4, "method": "tools/call", "params": {"name": "inspect_experiment", "arguments": {"root": str(fixture), "manifest": "../outside.json"}}},
]
run = subprocess.run([str(binary), "mcp"], input="\n".join(map(json.dumps, requests)) + "\n", text=True, capture_output=True, timeout=15, check=True)
assert run.stderr == "", run.stderr
replies = [json.loads(line) for line in run.stdout.splitlines()]
assert [r["id"] for r in replies] == [1, 2, 3, 4]
assert replies[0]["result"]["protocolVersion"] == "2025-06-18"
catalog = replies[1]["result"]["tools"]
assert {t["name"] for t in catalog} == {"inspect_experiment", "observe_experiment_sources", "inspect_experiment_candidate"}
assert all(t["annotations"]["readOnlyHint"] for t in catalog)
report = replies[2]["result"]["structuredContent"]
assert report["status"] == "complete_for_declared_scope"
assert len(report["nodes"]) == 7 and len(report["edges"]) == 6
assert report["coverage"]["data_files_read"] == 0
assert replies[3]["result"]["isError"] is True
assert "flowchart LR" in report["diagram"]["text"]
assert str(root) not in json.dumps(report)
for example in ["optimization", "stochastic", "learning", "cuda"]:
    output = subprocess.check_output([str(binary), "inspect", str(root / "fixtures" / example), "experiment-model.json"], text=True, timeout=15)
    assert json.loads(output)["status"] == "complete_for_declared_scope"
# Framing, malformed input and notification behavior.
malformed = subprocess.run([str(binary), "mcp"], input="invalid\n" + json.dumps({"jsonrpc":"2.0","id":5,"method":"ping"}) + "\n", text=True, capture_output=True, timeout=15, check=True)
assert [json.loads(x).get("error", {}).get("code") for x in malformed.stdout.splitlines()] == [-32700, None]
envelopes = json.loads((root / "tests/fixtures/malformed-requests.json").read_text())
requests = envelopes["invalid"] + envelopes["notifications"]
validation = subprocess.run([str(binary), "mcp"], input="\n".join(map(json.dumps, requests)) + "\n", text=True, capture_output=True, timeout=15, check=True)
responses = [json.loads(line) for line in validation.stdout.splitlines()]
assert len(responses) == len(envelopes["invalid"])
assert all(r["error"]["code"] == -32600 and r["id"] is None for r in responses)
oversize = subprocess.run([str(binary), "mcp"], input="x" * 1048577, text=True, capture_output=True, timeout=15, check=True)
assert not oversize.stdout
# Manifest-free host workflow: source observation -> host-authored candidate -> verification.
with tempfile.TemporaryDirectory(prefix="emi-no-manifest-") as directory:
    target = pathlib.Path(directory)
    shutil.copyfile(root / "fixtures/semantic/no-manifest/train.py", target / "model.py")
    before = (target / "model.py").read_bytes()
    observation = json.loads(subprocess.check_output([str(binary), "observe", str(target), "model.py", "1", "8"], text=True, timeout=15))
    lines = observation["files"][0]["lines"]
    evidence = [{"file":"model.py", "line":line["line"], "exact":line["text"]} for line in lines if line["line"] in [5,6]]
    candidate = {"version":1,"sources":["model.py"],"nodes":[{"id":"loss","kind":"loss","label":"Squared residual objective (inferred)","evidence":evidence,"interpretation":{"expression":"L = mean(residual^2) + penalty*sum(weights^2), inferred under ordinary arithmetic","confidence":0.7}}],"edges":[]}
    verified = subprocess.run([str(binary), "inspect-candidate", str(target)], input=json.dumps(candidate), text=True, capture_output=True, timeout=15, check=True)
    report = json.loads(verified.stdout)
    assert report["input_origin"] == "inline_candidate" and report["nodes"][0]["basis"] == "candidate_declared"
    assert report["interpretations"][0]["origin"] == "candidate_author"
    assert report["interpretations"][0]["mathematically_verified"] is False
    calls = [
        {"jsonrpc":"2.0","id":10,"method":"tools/call","params":{"name":"observe_experiment_sources","arguments":{"root":str(target),"files":[{"file":"model.py","start_line":1,"end_line":8}]}}},
        {"jsonrpc":"2.0","id":11,"method":"tools/call","params":{"name":"inspect_experiment_candidate","arguments":{"root":str(target),"candidate":candidate}}},
    ]
    run = subprocess.run([str(binary),"mcp"], input="\n".join(map(json.dumps,calls))+"\n", text=True,capture_output=True,timeout=15,check=True)
    results = [json.loads(line)["result"] for line in run.stdout.splitlines()]
    assert len(results) == 2 and all(r["isError"] is False for r in results)
    assert results[0]["structuredContent"]["coverage"]["data_files_read"] == 0
    assert results[1]["structuredContent"]["status"] == "complete_for_declared_scope"
    assert (target/"model.py").read_bytes() == before and len(list(target.iterdir())) == 1
print("PASS: CLI fixtures, MCP three-tool discovery/calls/errors/framing, manifest-free workflow, no source writes/dataset reads")
