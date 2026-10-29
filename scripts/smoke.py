"""End-to-end stdio MCP and CLI checks using synthetic fixtures only."""
import json
import pathlib
import subprocess
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
assert replies[1]["result"]["tools"][0]["name"] == "inspect_experiment"
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
oversize = subprocess.run([str(binary), "mcp"], input="x" * 1048577, text=True, capture_output=True, timeout=15, check=True)
assert not oversize.stdout
print("PASS: CLI fixtures, MCP initialize/list/call/errors/notifications/framing, no dataset reads")
