# Security

The runtime is read-only: no shell, subprocesses, URL fetching, dataset reads, server listening, telemetry or writes. It accepts an explicitly authorized root and a relative JSON manifest, opens bounded source files with cap-std, verifies exact source lines, and returns untrusted source snippets as evidence. It rejects hidden/credential paths, symlinks, special files and traversal. Exclusions do not detect arbitrary secrets inside ordinary source. Choose the smallest root and review outputs before sharing.

Limits: 32 sources / 128 nodes / 256 edges, 256 KiB per file, 2 MiB total; cooperative five-second checks. A synchronous OS filesystem call is not a hard deadline and can stall on remote filesystems; use ordinary local regular files. Reported graph semantics, metadata and confidence are author declarations. Evidence matching is not mathematical proof.

Only synthetic code is public. Never attach private experiment code, data, manifests containing sensitive references or conversation histories to public Issues/PRs. Public Issues remain open to everyone. Use GitHub's private vulnerability reporting when available rather than publishing sensitive reproductions.

Dependabot alerts and security updates are enabled at repository level. Update PRs require validation and human review, with no automatic merge. Scheduled RustSec audit plus Rust/Python/Actions CodeQL run on isolated standard public runners. Rust CodeQL build-mode none can still execute dependency build scripts/proc macros; no custom secrets or persisted checkout credentials are supplied. Fork PR workflows require external-contributor approval. `pull_request_target` is not used. Actions are pinned to full commit SHA. Normal main pushes stay allowed; force pushes/deletion are prohibited.

Security fixes are prioritized promptly when a compatible fix exists. No claim of instantaneous guaranteed remediation is made; unavailable fixes and incompatible updates need maintainer action. Dependency build code is executed only during trusted builds/CI, never during experiment inspection.
