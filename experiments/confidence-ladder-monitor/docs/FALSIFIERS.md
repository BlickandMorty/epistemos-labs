# Falsifiers and non-claims

The implementation is falsified if any of these occur:

1. The same ordered observations and policy produce different receipt heads.
2. A score exactly on a threshold is routed to the wrong tier.
3. NaN, infinity, an out-of-range score, or an invalid policy enters the log.
4. Per-outcome rates do not sum to one within floating-point tolerance.
5. Editing an observation without recomputing the chain still verifies.

The project does not claim that a confidence value is calibrated, that the default thresholds are universally optimal, or that acceptance implies factual correctness. Calibration must be established on data from the system that supplies the scores. Health thresholds should likewise be selected against an application-specific loss function.
