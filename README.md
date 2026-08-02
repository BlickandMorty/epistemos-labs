# Confidence Ladder Monitor

A deterministic admission ladder for systems that turn confidence scores into tiered decisions. Every accepted, escalated, or empty decision is recorded in a SHA-256 hash chain, and aggregate health is computed from explicit, falsifiable rates.

## Why it exists

A raw confidence score is not an operational policy. A production boundary needs ordered thresholds, exact edge behavior, invalid-input handling, an escalation rule, and evidence that the recorded decision is the one the policy actually produced.

The default ladder is:

| Score | Decision |
|---|---|
| `>= 0.85` | accept at T1 |
| `>= 0.75` | accept at T2 |
| `>= 0.70` | accept at T3 |
| below T3 | escalate or return empty, according to policy |

## Properties

- rejects NaN, infinity, out-of-range scores, and malformed policies
- defines exact inclusive threshold boundaries
- reports per-tier, escalation, empty, mean, and standard-deviation statistics
- classifies a stream as healthy, degrading, or failing using configurable rate thresholds
- binds policy, score bits, decision, sequence, escalation flag, and prior receipt into each hash
- replays and verifies the complete decision chain

## Try it

```bash
cargo run -- 0.92 0.79 0.71 0.50 --escalate
cargo test --all-targets --locked
cargo clippy --all-targets --locked -- -D warnings
```

## Research status and provenance

This is a standalone reconstruction of the confidence-floor ladder originally developed inside Jordan Conley's historical Epistemos research branches. The earlier internal implementation established the T1/T2/T3 thresholds and health-rate doctrine. This repository rebuilds that idea behind a smaller public API and adds strict input validation, configurable policy validation, deterministic hash-chained receipts, replay verification, a CLI, and isolated tests.

The thresholds are policy choices, not calibrated probabilities and not a claim that a model is truthful. See [`docs/FALSIFIERS.md`](docs/FALSIFIERS.md).

## License

MIT.
