# scope-rex-admission

Scope-Rex started as one of the bigger ideas in my Epistemos substrate research. This repo is the smaller part I can actually make sharp and reusable: before an agent action becomes durable, it has to fit an explicit scope and carry evidence.

The engine checks the action, resource root, risk ceiling, and evidence references. It emits the same receipt for the same canonical request, and the receipt can be recomputed later.

What works now:

- fail-closed scope manifests
- boundary-safe resource matching
- risk ceilings
- evidence-required admission
- deterministic proof receipts
- replay verification and negative tests

Run `cargo test`.

This is an admission primitive, not a claim that the full Scope-Rex cognitive substrate has been proved. The wider architecture and its open research questions are kept in `epistemos-research-canon`.

