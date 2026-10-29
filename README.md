# Experiment Model Inspector

A local Rust CLI and Codex stdio MCP plugin for **source-linked mathematical model and metadata-only data-flow descriptions**. MIT. No HTTP server, WebView, AI API, credentials, code execution, or dataset loading.

The initial version uses an explicit JSON evidence manifest. It verifies literal source lines in Python, C++, CUDA and Rust, then returns an author-declared graph, observed source text, inferred mathematical interpretations, limitations and unknowns. It does not claim to turn arbitrary programs into equations. Source equality is not semantic or mathematical proof. **The manifest author must list every source definition on which an interpretation depends**, including upstream definitions substituted into an expanded equation. The inspector checks only the supplied evidence lines; it does not discover omitted dependencies or guarantee freshness when an unlisted definition changes. For example, expanding `residual` in a loss requires the residual definition as well as the loss line. Keep expressions unexpanded or split them into smaller claims when the four-evidence-record limit cannot cover their dependencies. General code ownership/dependency graphs are outside its scope.

## Try it

Download the release archive for macOS arm64, Linux x64 or Windows x64. Compare its SHA-256 against `SHA256SUMS` from the same release, then extract into a new directory. No installer, background process or automatic updater is used. Release binaries are unsigned; no signing/notarization claim is made. Do not disable OS protections to run them.

```sh
./bin/experiment-model-inspector --version
./bin/experiment-model-inspector inspect fixtures/learning experiment-model.json
```

Windows: `bin\experiment-model-inspector.exe inspect fixtures\learning experiment-model.json`.

Or build with stable Rust (edition 2024) and `cargo build --release --locked`. Use `target/release/experiment-model-inspector` instead of `bin/...` above. Building this trusted repository may run dependency build scripts/proc macros; the **runtime inspector never builds or executes the inspected experiment**.

## Codex plugin

The archive is a complete plugin with root `plugin.json`/`mcp.json`, Codex compatibility `.codex-plugin/plugin.json`/`.mcp.json`, an opt-in Skill and `bin/` executable. OpenAI local authoring guidance and the host scaffold were used. Choose the archive for your OS; Windows MCP command names may omit `.exe` as standard process resolution supports executable extensions.

Add the extracted directory through your Codex local marketplace workflow. Preserve other entries and settings. See [INSTALL.md](INSTALL.md) for a sample. Enable the plugin and explicitly invoke its `inspect-experiment` Skill. Its single `inspect_experiment` tool takes `root` and relative `manifest`. No shared settings are modified by any project script. A direct MCP smoke test verifies the process; full Codex activation must be confirmed in the host after installation.

## Model and data flow

Use the [manifest reference](docs/manifest.md) and the four **synthetic** [fixtures](fixtures/). Nodes cover variables, parameters, operations, losses, constraints, state/time updates, inputs, preprocessing, models, evaluation and outputs. Graphs are returned as stable JSON and Mermaid text for conversation rendering. Source paths in reports are relative. Missing units, shapes, probability distributions and dataset contents remain unknown.

A learning fixture follows `input → preprocessing → model → loss/evaluation → output`; the stochastic fixture leaves the noise law unknown. The optimization fixture describes an objective, nonnegative constraint and gradient step. CUDA is inspected only as source text. Fixtures are snippets for inspection, not executable research programs. All source text, config and graph labels are untrusted data.

## Boundaries and security

Only explicit files in a user-authorized root are read. No recursive scan. Symlinks, parent traversal, hidden/credential paths, special files and oversized inputs are rejected. Data files are never read. Limits and unsupported features are in every successful report; see [SECURITY.md](SECURITY.md). Reports may include private source snippets from the selected files: keep them local and review before sharing. Filename exclusions are defense in depth, not a general secret-redaction engine.

CI uses free standard public GitHub runners for macOS arm64, Linux x64 and Windows x64. It runs formatting, clippy, tests, release build and process smoke checks. CodeQL covers Rust, Python helpers and Actions. Dependabot opens security update PRs; RustSec checks are scheduled. No automatic merge. Release packages include dependency license notices and hashes. See [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
