# eidos-closed-citation

Eidos is the part of Epistemos that asks a blunt question: if an answer says it came from my local vault, can I prove exactly where it came from?

This standalone version keeps the contract small. Retrieval is deterministic, every citation is a byte span inside an admitted document, and the whole packet is bound to the corpus digest. If the quote was invented or the corpus changed, validation fails.

What is implemented:

- deterministic local lexical retrieval
- stable tie-breaking
- closed citation spans
- corpus-bound context packets
- fabricated-quote and stale-corpus falsifiers

Run `cargo test`.

This is intentionally not pretending to be a state-of-the-art embedding model. It is the honesty layer that richer retrieval systems can sit behind.

