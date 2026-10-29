# Evidence manifest v1

`experiment-model.json` is a small, explicit, reviewable model description. The tool does **not** infer a full model from arbitrary source. It checks exact lines in declared C++/CUDA/Python/Rust files. It can describe declarations from an experiment configuration, but does not independently parse arbitrary config formats.

Start with `fixtures/learning/experiment-model.json`. Paths are relative to the authorized root. Line numbers are 1-based. `exact` contains the whole source line (including indentation, without the line terminator). LF and CRLF are supported. A mismatch removes the node and its interpretations/connected edges and reports partial coverage. An edge needs evidence of its own. The author must include all upstream definitions used by each interpretation in that node's evidence. The tool verifies only those listed lines and cannot detect missing dependencies. An unchanged use line does not keep an expanded expression fresh if an unlisted definition changes. The optimization fixture lists both residual and loss definitions (and the update line for the gradient interpretation); the stochastic recurrence lists both drift and next-state lines. Stay within four records by leaving expressions unexpanded or splitting claims.

```json
{
  "version": 1,
  "sources": ["train.py"],
  "nodes": [{
    "id": "loss",
    "kind": "loss",
    "label": "Squared residual",
    "evidence": [{"file": "train.py", "line": 1, "exact": "loss = x * x"}],
    "interpretation": {"expression": "L(x) = x^2", "confidence": 0.8},
    "metadata": {"units": "unknown", "shape": "unknown"}
  }],
  "edges": []
}
```

Kinds: `variable`, `parameter`, `operation`, `loss`, `constraint`, `state`, `input`, `preprocess`, `model`, `evaluation`, `output`.
Relations: `uses`, `feeds`, `updates`, `constrains`, `evaluates`, `produces`.
An edge has `from`, `to`, `relation`, and 1–4 `evidence` records. IDs use ASCII letters, digits or underscore, up to 64 bytes. Labels up to 160 bytes. Evidence lines up to 1024 bytes. Each node also needs 1–4 evidence records. An interpretation has a nonempty expression up to 512 bytes and confidence in [0,1]. The author supplies both; confidence is **not calibrated by the tool**.

Optional metadata fields: `path_reference`, `format`, `shape`, `schema`, `units`. Omitted values become JSON null (unknown). These are declarations, not measurements. Data references are relative metadata only and can point to nonexistent data. No dataset content is opened. URLs, absolute references and parent traversal are rejected. Do not put secrets or actual sample values in the manifest.

`observations` contains literal source evidence; `nodes`/`edges` are `user_declared` with `source_verified`; `interpretations` are `inferred`, `origin: manifest_author`, and `mathematically_verified: false`. Literal matching does not prove semantic relationships, mathematical correctness, probability laws, dimensions, or runtime behavior. Duplicate source observations may occur when multiple nodes share a line.

Limits: 32 explicit source files, 128 nodes, 256 edges; 256 KiB per file including manifest, 2 MiB total; cooperative 5-second checks between reads and model items (not a hard OS I/O timeout). No directory scan. Only `.py`, `.rs`, `.cpp`, `.cc`, `.cxx`, `.cu`, `.cuh`, `.hpp`, `.h`. Notebooks and automatic equation extraction are unsupported. Hidden paths, sensitive filenames, data directories and common generated/vendor directories are excluded from source reads. Symlinks and special files are rejected; capability-relative opens prevent root escape. The user-selected root is the read authority, so choose the smallest experiment directory.

Stable output discriminator: `schema_version: "1.0"`. Success status is `complete_for_declared_scope` or `partial`; errors have stable code strings, never raw OS messages or root paths. Consumers must handle unknown future fields, `issues` and `coverage.unsupported`. CLI errors exit 1; malformed usage exits 2. MCP tool failures use `isError: true`.
