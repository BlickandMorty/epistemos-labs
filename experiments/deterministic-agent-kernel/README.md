# deterministic-agent-kernel

I pulled this out of my older Epistemos work because the idea deserved to stand on its own.

The point is simple: the same admitted inputs should produce the same decision and the same receipt. Every allow and every denial goes into one hash-linked log, so a replay can tell me if the history changed after the fact.

This is a real working kernel, not the whole “deterministic AI” research claim. It covers the part I can make concrete today:

- capability-based decisions
- canonical, deterministic receipts
- hash-linked history
- fail-closed replay verification
- tests for determinism, denial, and tampering

Run it with `cargo test` and `cargo run --bin dak-demo`.

## Where this came from

This is an original standalone rebuild of the deterministic runtime, admission-proof, and no-hidden-authority ideas in my Epistemos history. The broader autogenous-kernel theorem remains research and lives in the separate theorem/research repositories.

## What I am not claiming

The FNV-1a digest used here is a deterministic integrity checksum, not a cryptographic signature. A production deployment should bind receipts to a real signing key and a defined trust root.

