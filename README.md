# vault-recall-benchmark

I recovered this from the F-Vault-Recall-50 work in Epistemos and made the benchmark independent of the app.

The repo gives local retrieval systems a small, deterministic test contract: ranked IDs in, recall and reciprocal-rank witnesses out. It includes a transparent 50-case synthetic fixture so CI can prove the harness itself before anyone swaps in a real vault corpus.

It has:

- deterministic lexical ranking
- stable tie-breaking
- recall@k
- mean reciprocal rank
- a generated 50-case fixture
- explicit zero-result failures

Run `cargo test`.

The synthetic fixture is a harness check, not a claim about real-world retrieval quality. Real comparisons should add a held-out corpus and publish the exact fixture manifest.

