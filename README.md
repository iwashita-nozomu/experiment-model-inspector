# Experiment Model Inspector

A local Rust CLI and Codex stdio MCP plugin for **source-linked mathematical model and metadata-only data-flow descriptions**. MIT. No HTTP server, WebView, additional AI API, new credentials, experiment execution, or dataset loading.

The host Codex GPT reads bounded code observations, explains the variables, operations, losses, constraints, state updates and metadata-only data flow, and returns a native Mermaid diagram. No pre-existing manifest is required. The Rust binary supplies exact line-numbered observations and checks an inline candidate; it does not generate a canned explanation or make an AI API call.

Observed source, GPT-proposed meanings and unknowns remain separate. A source match does not prove semantics or mathematics. The host must cite all definitions used by an expanded interpretation; omitted dependencies are not discovered automatically. Missing shape, units, probability laws and data contents stay unknown. See the [semantic workflow and acceptance examples](skills/inspect-experiment/references/semantic-workflow.md) and [source-specific GPT explanation](docs/semantic-example.md). General module responsibility/dependency graphs belong to a separate plugin.

## Try it

Download the release archive for macOS arm64, Linux x64 or Windows x64. Compare its SHA-256 against `SHA256SUMS` from the same release, then extract into a new directory. No installer, background process or automatic updater is used. Release binaries are unsigned; no signing/notarization claim is made. Do not disable OS protections to run them.

```sh
./bin/experiment-model-inspector --version
./bin/experiment-model-inspector observe fixtures/semantic/no-manifest train.py 1 8
# Host GPT interprets these observations and keeps a candidate in private scratch space:
./bin/experiment-model-inspector inspect-candidate fixtures/semantic/no-manifest < candidate.json
# Optional compatibility path for an existing user manifest:
./bin/experiment-model-inspector inspect fixtures/learning experiment-model.json
```

Windows: `bin\experiment-model-inspector.exe inspect fixtures\learning experiment-model.json`.

Or build with stable Rust (edition 2024) and `cargo build --release --locked`. Use `target/release/experiment-model-inspector` instead of `bin/...` above. Building this trusted repository may run dependency build scripts/proc macros; the **runtime inspector never builds or executes the inspected experiment**.

## Codex plugin

The archive is a complete plugin with root `plugin.json`/`mcp.json`, Codex compatibility `.codex-plugin/plugin.json`/`.mcp.json`, an opt-in Skill and `bin/` executable. OpenAI local authoring guidance and the host scaffold were used. Choose the archive for your OS; Windows MCP command names may omit `.exe` as standard process resolution supports executable extensions.

Add the extracted directory through your Codex local marketplace workflow. Preserve other entries and settings. See [INSTALL.md](INSTALL.md) for a sample. Enable the plugin and explicitly invoke its `inspect-experiment` Skill. Use `observe_experiment_sources` to read explicit source ranges, then let the host GPT reason and call `inspect_experiment_candidate` with an inline candidate. Neither tool writes into the inspected root. `inspect_experiment` remains available for an existing user manifest. No shared settings are modified by any project script. A direct MCP smoke test verifies the process; full Codex activation must be confirmed in the host after installation.

## Model and data flow

Use the [tool/workflow reference](docs/semantic-tools.md), [candidate format](docs/manifest.md) and **synthetic** [fixtures](fixtures/). The manifest-free learning pair differs only in the main loss expression, so the host must give different explanations. Automated tests verify transport, bounds, evidence freshness, provenance and no source writes; prose quality and mathematical reasoning are host GPT responsibilities. Nodes cover variables, parameters, operations, losses, constraints, state/time updates, inputs, preprocessing, models, evaluation and outputs. Graphs are returned as stable JSON and Mermaid text for conversation rendering. Source paths in reports are relative. Missing units, shapes, probability distributions and dataset contents remain unknown.

A learning fixture follows `input → preprocessing → model → loss/evaluation → output`; the stochastic fixture leaves the noise law unknown. The optimization fixture describes an objective, nonnegative constraint and gradient step. CUDA is inspected only as source text. Fixtures are snippets for inspection, not executable research programs. All source text, config and graph labels are untrusted data.

## Boundaries and security

Only explicit files in a user-authorized root are read. Source observation permits 8 explicit slices of up to 200 lines, at most 64 KiB returned source per call; continuation and omissions are explicit. Candidate input is bounded to 256 KiB. No recursive scan. Symlinks, parent traversal, hidden/credential paths, special files and oversized inputs are rejected. Data files are never read. Limits and unsupported features are in every successful report; see [SECURITY.md](SECURITY.md). Reports may include private source snippets from the selected files: keep them local and review before sharing. Filename exclusions are defense in depth, not a general secret-redaction engine.

CI uses free standard public GitHub runners for macOS arm64, Linux x64 and Windows x64. It runs formatting, clippy, tests, release build and process smoke checks. CodeQL covers Rust, Python helpers and Actions. Dependabot opens security update PRs; RustSec checks are scheduled. No automatic merge. Release packages include dependency license notices and hashes. See [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
