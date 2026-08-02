# proof-carrying-security-lab

I wanted the part of autonomous pentesting that I actually trust: strict scope, read-only source intake, parallelizable analysis stages, and no finding unless the evidence can be replayed.

This first release is deliberately defensive. It does not execute exploits, touch a network target, or ship payloads. It scans only the files named in an authorized engagement manifest, redacts credential material, and binds every finding to a stable source witness.

What works:

- explicit authorization gate
- exact file allowlist
- read-only in-memory analysis
- source-backed findings with line evidence
- evidence redaction and replay
- deterministic report ordering
- negative tests for authorization and scope drift

Run `cargo test`.

## Why this architecture

The project is a clean-room build inspired by the strongest public patterns I found: Shannon’s source-aware, proof-before-report pipeline; OpenAEV’s campaign/control-plane separation; and HackingBuddyGPT/PentAGI’s bounded agent-task model. I did not copy their code.

See [INSPIRATION.md](INSPIRATION.md) for the exact upstream links and license boundary.

## Use boundary

Only analyze systems you own or have explicit written authorization to test. This repository is built to make that boundary part of the program instead of a sentence people skip.

