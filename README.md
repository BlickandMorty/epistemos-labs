# Epistemos Labs

This is the map for the research and systems work I pulled out of Epistemos.

The common idea is proof-carrying software: decisions should expose their scope, numerical kernels should expose their error, retrieval should expose its citations, schema repair should expose its patches, and research should expose what is proved versus what is still a candidate.

## Start here

| Project | What it shows | Status |
| --- | --- | --- |
| [deterministic-agent-kernel](https://github.com/BlickandMorty/deterministic-agent-kernel) | deterministic decisions, hash-linked receipts, replay | working + tested |
| [scope-rex-admission](https://github.com/BlickandMorty/scope-rex-admission) | bounded capability/risk/evidence admission | working + tested |
| [eidos-closed-citation](https://github.com/BlickandMorty/eidos-closed-citation) | local retrieval whose quotes can be verified | working + tested |
| [primitive-ir-lab](https://github.com/BlickandMorty/primitive-ir-lab) | EML and typed certificate-carrying primitives | working + tested |
| [f-ulp-oracle](https://github.com/BlickandMorty/f-ulp-oracle) | binary16 conversion and ULP witnesses | working + tested |
| [lattice-wbo-ledger](https://github.com/BlickandMorty/lattice-wbo-ledger) | explicit weighted-bound accounting | working + tested |
| [hyperdynamic-schema-repair](https://github.com/BlickandMorty/hyperdynamic-schema-repair) | bounded repair with replayable patches | working + tested |
| [vault-recall-benchmark](https://github.com/BlickandMorty/vault-recall-benchmark) | deterministic recall/MRR evaluation | working + tested |
| [proof-carrying-security-lab](https://github.com/BlickandMorty/proof-carrying-security-lab) | authorized source analysis with evidence-gated findings | defensive working lab |

## The formal and research layer

| Project | What it contains | Status |
| --- | --- | --- |
| [epistemos-formal-primitives](https://github.com/BlickandMorty/epistemos-formal-primitives) | 246 actual Lean theorem/lemma declarations and an exact proof-status ledger | 209 proof terms present; 37 candidates |
| [epistemos-research-canon](https://github.com/BlickandMorty/epistemos-research-canon) | the 5,749-line lattice explainer, theorem canon, Scope-Rex/substrate/EML/Eidos research, and recovery map | public research archive |

## How I separate claims

- “working” means there is focused source and a test suite.
- “proof term present” means Lean has a term and that declaration does not use `sorry`.
- “candidate” means the idea is intentionally public but not claimed as proved.
- “research” means the document can be novel or useful without pretending it is a finished library.

## Source history

The recovered work came from the regular [Epistemos](https://github.com/BlickandMorty/Epistemos) history, including the May 2026 research/checkpoint/salvage lines and the later lattice-coordinate explainer. `Epistemos-RETRO` is not the source for this collection.

## Security architecture boundary

The security lab is an original clean-room build. Shannon, OpenAEV, HackingBuddyGPT, and PentAGI were studied as architecture references. AGPL source from Shannon is not copied into these Apache-licensed repositories.
