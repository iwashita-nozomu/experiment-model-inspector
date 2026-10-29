# Local Codex installation

Extract an OS-specific release into a **new** folder. Compare SHA256SUMS before use. Test `bin/experiment-model-inspector --version` and the synthetic fixture first.

Use the host's local plugin UI/marketplace support. A personal marketplace entry has this shape, appended to the existing `plugins` array without deleting other plugins:

```json
{
  "name": "experiment-model-inspector",
  "source": {"source": "local", "path": "./plugins/experiment-model-inspector"},
  "policy": {"installation": "AVAILABLE", "authentication": "ON_INSTALL"},
  "category": "Productivity"
}
```

This assumes the extracted plugin lives in `plugins/experiment-model-inspector` relative to the marketplace file. The default host marketplace is `~/.agents/plugins/marketplace.json`. Do not copy that example over an existing marketplace. Registration/activation is a separate host operation; this repository never changes shared host configuration. After enabling the plugin, confirm discovery of `inspect_experiment` and call it on a synthetic fixture. Invoke the bundled Skill explicitly.

Portable `mcp.json` and legacy `.mcp.json` point to `${PLUGIN_ROOT}/bin/experiment-model-inspector mcp`. Windows executable resolution supports `.exe`. No PATH/global shell profile changes are required. For an existing plugin update, compare hashes, extract a new version to a separate folder, test it, then switch through the host. There is no daemon, cron, unattended auto-update or auto-merge.
