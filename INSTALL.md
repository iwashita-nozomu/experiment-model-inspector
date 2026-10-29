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

Resolve `source.path` against the **active marketplace root reported by the host**, not the directory containing `marketplace.json`. Run `codex plugin marketplace list --json` and find the root for the selected marketplace. The example above resolves to `<reported-root>/plugins/experiment-model-inspector`; extract the plugin at that exact location. On the verified Codex host, `personal-local` used the home directory as its root even though its manifest was `~/.agents/plugins/marketplace.json`. Confirm your host's current result instead of assuming the same layout.

Preserve existing entries and back up the marketplace/config before editing. Refuse an existing destination rather than overwriting it. After adding the entry, use `codex plugin add experiment-model-inspector@personal-local --json`, then `codex plugin list --marketplace personal-local --json` to confirm that the installed source path matches the resolved location and that the plugin is enabled. This repository never changes shared host configuration automatically. Confirm discovery of `inspect_experiment`, call it on a synthetic fixture, and invoke the bundled Skill explicitly.

Portable `mcp.json` and legacy `.mcp.json` point to `${PLUGIN_ROOT}/bin/experiment-model-inspector mcp`. Windows executable resolution supports `.exe`. No PATH/global shell profile changes are required. For an existing plugin update, compare hashes, extract a new version to a separate folder, test it, then switch through the host. There is no daemon, cron, unattended auto-update or auto-merge.
