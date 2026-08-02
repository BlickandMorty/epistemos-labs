# primitive-ir-lab

This is the working floor I recovered from my Epistemos EML/primitive-IR branch.

I was exploring whether a small typed family—EML, geometry, information, operator, scan, and tropical primitives—could carry its own fixtures and verification receipts. The old repo mixed that work into a much larger app. Here I kept the part that is easy to run and reason about.

Right now the repo has:

- the real-domain EML primitive `exp(x) - ln(y)` with explicit branch failure
- a checked inverse in `x`
- a small typed expression tree
- bit-exact evaluation certificates
- scan and tropical examples
- negative tests for invalid domains and non-finite values

Run `cargo test`.

The richer Rust modules and Lean schemas are preserved in the research/formal repos. I am not claiming the broad “universal elementary function” thesis is proved here; this repo is the executable substrate for testing claims like that.

