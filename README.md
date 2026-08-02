# f-ulp-oracle

I built the first version of this inside Epistemos to stop performance work from hiding numerical drift behind an average.

This repo turns that into a small standalone binary16 witness system. It converts IEEE-754 half values, measures exact representable steps, records every comparison, and fails the run when any witness crosses the chosen ULP bar.

Included now:

- binary16 to/from `f32`
- round-to-nearest-even behavior
- sign-aware ULP distance
- per-case witnesses
- worst-case reporting
- an explicit acceptance threshold

Run `cargo test`.

The old Epistemos research used a two-ULP fp16 shipping bar. That is the example in the tests, not a universal tolerance for every numerical kernel.

