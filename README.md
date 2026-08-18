# Epistemos Labs

A Rust workspace for proof-carrying agent, evidence, numerical, retrieval, and
formal-methods experiments recovered from the broader Epistemos research line.

The individual experiments predate this monorepo. They were consolidated in
August 2026 to make the work easier to evaluate and to avoid presenting a set
of small, related prototypes as portfolio padding. Their full Git histories
were imported without squashing and remain reachable from this repository.

## Unifying question

What would software look like if a decision had to carry enough evidence to be
replayed, challenged, bounded, or refused?

The experiments answer from different levels: four-valued evidence, confidence
admission, deterministic receipts, scoped authority, closed citations, typed
intermediate representations, numerical error witnesses, and explicit
weighted-bound accounting.

## Workspace map

| Experiment | Core idea | Verification |
| --- | --- | --- |
| [belnap-evidence-gate](experiments/belnap-evidence-gate) | Belnap four-valued evidence and conflict abstention | Rust tests |
| [confidence-ladder-monitor](experiments/confidence-ladder-monitor) | Tiered confidence admission and receipt-chain replay | Rust tests |
| [interrupt-score-router](experiments/interrupt-score-router) | Bounded five-signal escalation with falsifiers | Rust tests |
| [deterministic-agent-kernel](experiments/deterministic-agent-kernel) | Deterministic decisions and hash-linked replay receipts | Rust tests + demo |
| [scope-rex-admission](experiments/scope-rex-admission) | Capability/risk/evidence admission under scope | Rust tests |
| [eidos-closed-citation](experiments/eidos-closed-citation) | Local retrieval with checkable quote witnesses | Rust tests |
| [primitive-ir-lab](experiments/primitive-ir-lab) | EML and certificate-carrying typed primitives | Rust tests |
| [f-ulp-oracle](experiments/f-ulp-oracle) | Binary16 conversion and ULP witnesses | Rust tests |
| [lattice-wbo-ledger](experiments/lattice-wbo-ledger) | Explicit weighted-bound accounting | Rust tests |
| [hyperdynamic-schema-repair](experiments/hyperdynamic-schema-repair) | Bounded schema repair with replayable patches | Rust tests |
| [vault-recall-benchmark](experiments/vault-recall-benchmark) | Deterministic recall/MRR evaluation | Rust tests |
| [proof-carrying-security-lab](experiments/proof-carrying-security-lab) | Authorized source analysis with evidence-gated findings | Defensive Rust tests |

## Run everything

```bash
cargo test --workspace --all-targets
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

Individual experiments can also be run with `cargo test -p <package-name>`.

## How claims are separated

- **Working** means executable source and focused tests are present.
- **Witness** means an output carries inspectable evidence or replay material;
  it is not automatically a formal proof.
- **Proof term present** is reserved for Lean declarations with an actual term
  and no placeholder.
- **Candidate** means the mechanism is public research, not a proved theorem or
  production guarantee.

For the commit-level recovery map and validation record, see
[RECOVERY_AUDIT.md](RECOVERY_AUDIT.md). The larger formal and research layers
remain public in
[epistemos-formal-primitives](https://github.com/BlickandMorty/epistemos-formal-primitives)
and
[epistemos-research-canon](https://github.com/BlickandMorty/epistemos-research-canon).

## Consolidation and provenance

Each directory under `experiments/` was imported with `git subtree` without
squashing. That keeps authorship, dates, and source commits in this graph while
allowing redundant standalone repository pages to be retired. The source work
came from regular Epistemos history and its recovered May 2026 research,
checkpoint, and salvage lines—not from a newly invented August portfolio batch.

The defensive security experiment is an original, authorization-bounded lab.
External red-team projects informed architectural study only; their code is not
relicensed or copied into this workspace.

## License

Apache-2.0 for the workspace unless an imported experiment contains a more
specific license file, in which case that experiment's file governs its code.
