"""Build a self-contained OS-specific plugin ZIP; no shared configuration writes."""
import hashlib
import json
import pathlib
import platform
import sys
import zipfile

root = pathlib.Path(__file__).resolve().parents[1]
label = sys.argv[1]
assert label in ["macos-arm64", "linux-x64", "windows-x64"]
binary = root / "target" / "release" / ("experiment-model-inspector.exe" if label == "windows-x64" else "experiment-model-inspector")
assert binary.is_file()
version = json.loads((root / "plugin.json").read_text())["version"]
dest = root / "dist"
dest.mkdir(exist_ok=True)
archive = dest / f"experiment-model-inspector-v{version}-{label}.zip"
assert not archive.exists(), "Refusing to overwrite an existing archive"
files = ["plugin.json", "mcp.json", ".codex-plugin/plugin.json", ".mcp.json", "LICENSE", "README.md", "INSTALL.md", "SECURITY.md", "THIRD_PARTY_NOTICES.md"]
for folder in ["skills", "docs", "fixtures", "third-party"]:
    files.extend(str(p.relative_to(root)) for p in (root / folder).rglob("*") if p.is_file())
with zipfile.ZipFile(archive, "w", compression=zipfile.ZIP_DEFLATED) as z:
    for name in sorted(files):
        p = root / name
        assert not p.is_symlink()
        data = p.read_bytes()
        if label == "windows-x64" and name in ["mcp.json", ".mcp.json"]:
            config = json.loads(data)
            config["mcpServers"]["experiment-model-inspector"]["command"] += ".exe"
            data = (json.dumps(config, indent=2) + "\n").encode()
        info = zipfile.ZipInfo("experiment-model-inspector/" + name)
        info.external_attr = 0o100644 << 16
        z.writestr(info, data)
    info = zipfile.ZipInfo("experiment-model-inspector/bin/" + binary.name)
    info.external_attr = 0o100755 << 16
    z.writestr(info, binary.read_bytes())
    z.writestr("experiment-model-inspector/BUILD.json", json.dumps({"version": version, "target": label, "python_platform": platform.system()}, indent=2))
with zipfile.ZipFile(archive) as z:
    assert z.testzip() is None
    names = z.namelist()
    assert all(n.startswith("experiment-model-inspector/") and ".." not in pathlib.PurePosixPath(n).parts for n in names)
    for manifest in ["plugin.json", ".codex-plugin/plugin.json", "mcp.json", ".mcp.json"]:
        json.loads(z.read("experiment-model-inspector/" + manifest))
    assert any("third-party/licenses/" in n for n in names)
    assert "experiment-model-inspector/skills/inspect-experiment/references/experiment-selection.js" in names
sha = hashlib.sha256(archive.read_bytes()).hexdigest()
archive.with_suffix(".zip.sha256").write_text(sha + "  " + archive.name + "\n")
print(archive.name + " " + sha)
