# Epistemos portfolio recovery audit

> Consolidation note (August 2026): the small Rust repositories cataloged
> below now live, with unsquashed Git history, under `experiments/` in this
> workspace. Historical standalone links are retained as source records; the
> monorepo README is the current navigation surface.

Audit date: 2026-08-02

## Outcome

The GitHub account now has 47 owned repositories: 46 public, one private, and 11 provenance-preserving forks. The only private repository is `epistemos-site`, retained privately because it is substantially duplicated by the already-public `meta-analytical-pfc` codebase.

Publication followed four rules:

1. A README-only promise was not made public until the promised implementation and tests existed.
2. Epistemos logic became an independent repository only when the user's own subsystem could be separated cleanly.
3. Third-party code stayed in GitHub forks with upstream history and license metadata intact.
4. Every new original repository required a license, limitations or falsifiers, automated tests, CI, and a clean credential-pattern scan.

## Historical source inventory

The full Epistemos checkout contained 21,791 commits. The audit used read-only `git show` and tree inspection because the historical checkout itself had unrelated worktree changes. High-signal refs included:

- `codex/research-snapshot-2026-05-08`
- `codex/t1-trifusion-2026-05-16`
- `codex/t5-emlir-2026-05-16`
- `codex/t7-eml-2026-05-16`
- `codex/t10-eidos-v0-2026-05-18`
- `codex/t12-f-ulp-oracle-2026-05-18`
- `codex/t17b-lattice-wbo-register-2026-05-18`
- `codex/t18b-acs-admission-field-2026-05-18`
- `codex/t21-vault-recall-contract-2026-05-18`
- `run-b-post-v1-research`

The strongest recovered source seams were:

| Historical source | Public destination | Boundary |
|---|---|---|
| `agent_core/src/research/belnap.rs` | [belnap-evidence-gate](https://github.com/BlickandMorty/belnap-evidence-gate) | Belnap logic is prior art; evidence admission and replay receipts are the systems contribution |
| `epistemos-research/src/interrupt_score.rs` | [interrupt-score-router](https://github.com/BlickandMorty/interrupt-score-router) | five-signal HELIOS score rebuilt with validation, hash-chain replay, and a collapse falsifier |
| `agent_core/src/research/confidence_floors.rs` | [confidence-ladder-monitor](https://github.com/BlickandMorty/confidence-ladder-monitor) | T1/T2/T3 doctrine rebuilt with policy validation and receipt verification |
| E1-E7 and later Lean modules | [epistemos-formal-primitives](https://github.com/BlickandMorty/epistemos-formal-primitives) | exact proof-status ledger separates proved declarations from candidates |
| lattice, substrate, HELIOS, EML, Eidos documents | [epistemos-research-canon](https://github.com/BlickandMorty/epistemos-research-canon) and [research](https://github.com/BlickandMorty/research) | research claims retain status tags and falsifiers |
| agent capability and receipt logic | [deterministic-agent-kernel](https://github.com/BlickandMorty/deterministic-agent-kernel) and [scope-rex-admission](https://github.com/BlickandMorty/scope-rex-admission) | executable admission kernels rather than product-wide copies |
| mutation/provenance and cognitive-DAG work | existing Epistemos history, research canon, and memory projects | not split again because it overlaps published provenance and memory systems |

## Rebuilt dormant repositories

| Original private repository | Public result | Verified behavior |
|---|---|---|
| `OLS-vs.-Gradient-Descent` | [ols-vs-gradient-descent](https://github.com/BlickandMorty/ols-vs-gradient-descent) | SVD-backed OLS, Lipschitz-bounded batch gradient descent, deterministic data, benchmark falsifiers, 9 tests |
| `Kinetic-Protocol` | [kinetic-protocol](https://github.com/BlickandMorty/kinetic-protocol) | confined file admission, fixed tool registry, bounded envelopes, atomic memory, no arbitrary executor, 8 tests |
| `Project-Ethos` | [ethos-eval](https://github.com/BlickandMorty/ethos-eval) | offline fixtures, weighted transparent rules, deterministic reports, per-category summaries, 10 tests |
| `Epistemos-Windows` | [epistemos-windows](https://github.com/BlickandMorty/epistemos-windows) | native C++ build and 2 CTest tests, 7 Rust tests plus clippy, 5 Swift tests |

All four retained their original repository history; they were completed in place and renamed only after CI passed.

## New independent extractions

| Repository | Tests | Main falsifier or limitation |
|---|---:|---|
| [belnap-evidence-gate](https://github.com/BlickandMorty/belnap-evidence-gate) | 6 | conflict or unknown evidence must abstain; altered receipts must fail replay |
| [interrupt-score-router](https://github.com/BlickandMorty/interrupt-score-router) | 6 | escalation rate above the declared bound exposes static/hybrid collapse |
| [confidence-ladder-monitor](https://github.com/BlickandMorty/confidence-ladder-monitor) | 8 | malformed scores/policies fail closed; the same stream must reproduce the same receipt head |

## External red-team references

The likely remembered AGPL project was [Shannon](https://github.com/KeygraphHQ/shannon), a white-box autonomous AI pentester. Reference copies are transparent GitHub forks, not relabeled original work:

| Fork | Upstream license | Why retain it |
|---|---|---|
| [BlickandMorty/shannon](https://github.com/BlickandMorty/shannon) | AGPL-3.0 | source-aware analysis, attack planning, exploit validation, report boundary |
| [BlickandMorty/strix](https://github.com/BlickandMorty/strix) | Apache-2.0 | multi-agent application-security orchestration and local run artifacts |
| [BlickandMorty/pentagi](https://github.com/BlickandMorty/pentagi) | MIT | hierarchical flow/task/subtask execution and isolated tool workers |

Original defensive work remains in [proof-carrying-security-lab](https://github.com/BlickandMorty/proof-carrying-security-lab). No AGPL source was copied into the original Apache-licensed security repositories.

## Deliberate non-public decision

`epistemos-site` remains private. It has 349 current files; 277 paths overlap the public `meta-analytical-pfc/brainiac-2.0` tree and 261 of those files are byte-identical. Its unique work is mostly a later presentation/site layer around the same PFC product. Publishing it as another standalone project would create misleading duplication rather than a distinct employer-facing artifact. Current files and all 37 commits produced zero matches in the credential-pattern audit.

## Verification record

- New and rebuilt repositories were scanned for common token, private-key, cloud-key, password-assignment, and secret-assignment patterns before publication.
- Every newly published default branch was pushed through Git Credential Manager without printing or storing the credential.
- GitHub Actions passed on the published commits for Belnap, interrupt routing, confidence ladder, OLS/GD, KINETIC, ETHOS, and Epistemos Windows.
- Belnap, interrupt routing, and OLS/GD were also installed and tested from clean clones.
- Empty placeholders were not exposed as finished software.

This document records repository provenance and validation, not a claim that every research hypothesis is proved or every scaffold feature is implemented. Individual READMEs carry the narrower status and non-claims for each project.
