# belnap-evidence-gate

[![CI](https://github.com/BlickandMorty/belnap-evidence-gate/actions/workflows/ci.yml/badge.svg)](https://github.com/BlickandMorty/belnap-evidence-gate/actions/workflows/ci.yml)

A deterministic claim-admission gate recovered from my older Epistemos
research. Evidence may support a claim, refute it, do both, or do neither. The
gate maps those cases onto Belnap's four-valued logic and refuses to manufacture
certainty when evidence conflicts or is absent.

```text
support only  -> True    -> assert supported
refute only   -> False   -> reject as refuted
both          -> Both    -> abstain: conflict
neither       -> Neither -> abstain: unknown
```

## What is original here

Belnap's bilattice is established prior art, not my invention. This repository's
contribution is the systems boundary I developed in Epistemos: canonicalize
evidence, reject duplicate identities, turn the four-valued result into an
explicit admission decision, and bind the complete decision to a replayable
SHA-256 receipt.

The implementation derives both lattice operations from the `(supported,
refuted)` bit pair rather than maintaining fragile hand-written tables. Tests
exhaustively check commutativity, associativity, absorption, involutive negation,
all four gate outcomes, order-independent receipts, and tamper rejection.

## Run it

```bash
cargo test --all-targets
cargo run -- claim-42 2 1
```

The example emits JSON. Arguments are claim ID, supporting-evidence count, and
refuting-evidence count.

## Evidence contract

Every evidence item needs a stable ID, source digest, and stance. The receipt
sorts evidence by ID before hashing, so input order does not affect the result.
Length-prefixed fields avoid delimiter ambiguity. Verification reconstructs the
value, decision, canonical order, and digest.

This is an evidence-accounting primitive—not a fact checker. A cryptographically
stable receipt can prove what evidence was admitted and how the gate decided; it
cannot prove that the source evidence was truthful.

## Research provenance

Recovered from the Epistemos `agent_core/src/research/belnap.rs` line of work,
where it powered the AnswerPacket honesty/abstention gate. The standalone design
removes application dependencies and keeps the claim boundary explicit.

## License

MIT.
