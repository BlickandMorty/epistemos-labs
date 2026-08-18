# lattice-wbo-ledger

The lattice/WBO work in Epistemos kept coming back to the same problem: compression and residency decisions sound scientific until every term has to fit in one auditable row.

This repo is that row made executable. Each item records distortion, rate, side information, multipliers, a tier, and a hard budget. The ledger refuses negative, non-finite, duplicate, or over-budget entries.

It currently provides:

- explicit weighted-bound terms
- hot/warm/cold/archive residency tiers
- fail-closed validation
- deterministic ledger ordering
- per-tier and global accounting
- falsifiers for budget and identity errors

Run `cargo test`.

This does not prove the full historical WBO-7 inequality. It gives that research a concrete accounting surface where proposed bounds can be tested instead of just named.

