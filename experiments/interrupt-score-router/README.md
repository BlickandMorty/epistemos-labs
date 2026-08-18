# interrupt-score-router

[![CI](https://github.com/BlickandMorty/epistemos-labs/actions/workflows/ci.yml/badge.svg)](https://github.com/BlickandMorty/epistemos-labs/actions/workflows/ci.yml)

An auditable dynamic-compute policy recovered from my older Epistemos/HELIOS
research. Instead of running the most expensive reasoning path for every token
or request, the router combines five bounded signals:

```text
u = alpha * predictive_entropy
  + beta  * budget_overrun_risk
  + gamma * consistency_residual
  + delta * tool_need
  + epsilon * component_alarm
```

The score selects one of three lanes:

```text
u < tau_recall                -> continue cheaply
tau_recall <= u < tau_escalate -> recall/context episode
u >= tau_escalate             -> full escalation
```

## What makes this more than a formula

- All signals must be finite and inside `[0,1]`.
- Weights must be non-negative and sum to one.
- Thresholds must be ordered and bounded.
- Boundary behavior is explicit and tested.
- Every decision is encoded with IEEE-754 bit patterns and appended to a
  SHA-256 receipt chain.
- Replay recomputes the score, route, chain linkage, and digest.
- An escalation-rate audit makes the research falsifier executable: if full
  escalation exceeds a chosen maximum, the supposedly dynamic architecture has
  collapsed toward an always-heavy static hybrid.

## Run

```bash
cargo test --all-targets
cargo run
```

## Research status

The routing mechanism and evidence contracts are implemented. The five-signal
equation is a research policy, not a trained production controller. The uniform
weights and example thresholds are illustrative. A real deployment must
calibrate signals and thresholds on representative workloads, publish its
false-escalation/false-deferral costs, and keep the escalation-rate falsifier.

This standalone project descends from the Epistemos `InterruptScore` and
runtime-router shadow/parity work. It intentionally excludes application UI,
model code, and unsupported claims about the broader HELIOS architecture.

## License

MIT.
