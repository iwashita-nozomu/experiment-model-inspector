"""Generate dependency notices from Cargo's locked registry metadata (no installer)."""
import json
import pathlib
import shutil
import subprocess

root = pathlib.Path(__file__).resolve().parents[1]
metadata = json.loads(subprocess.check_output(["cargo", "metadata", "--locked", "--format-version", "1"], cwd=root, text=True))
dest = root / "third-party" / "licenses"
if dest.exists():
    shutil.rmtree(dest)
dest.mkdir(parents=True)
lines = ["# Third-party notices", "", "This binary includes or is built using the following crates locked in Cargo.lock.", "License expressions below come from registry package metadata. Corresponding copyright and license texts are bundled in third-party/licenses. All target-specific and build dependencies are included conservatively.", "", "| Crate | Version | License |", "| --- | --- | --- |"]
for package in sorted(metadata["packages"], key=lambda p: p["name"]):
    if not package["source"]:
        continue
    assert package["source"].startswith("registry+https://github.com/rust-lang/crates.io-index")
    name, version = package["name"], package["version"]
    lines.append(f'| {name} | {version} | {package["license"]} |')
    source = pathlib.Path(package["manifest_path"]).parent
    texts = [p for p in source.iterdir() if p.is_file() and p.name.upper().startswith(("LICENSE", "COPYING", "NOTICE", "COPYRIGHT"))]
    # Some crates put canonical license files in a license/ subdirectory.
    for folder in [source / "license", source / "licenses"]:
        if folder.is_dir():
            texts.extend(p for p in folder.rglob("*") if p.is_file())
    assert texts, f"No license text: {name}"
    out = dest / f"{name}-{version}"
    out.mkdir()
    for p in texts:
        shutil.copyfile(p, out / str(p.relative_to(source)).replace("/", "_"))
(root / "THIRD_PARTY_NOTICES.md").write_text("\n".join(lines) + "\n")
print(f"Notices generated for {len(lines)-7} registry packages")
