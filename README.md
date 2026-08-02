# hyperdynamic-schema-repair

“Hyperdynamic schema” was a large name in the old Epistemos docs. The useful core is much simpler: detect drift, apply only declared repairs, repeat within a hard bound, and keep a witness that can replay the result.

That is what this repo does.

- deterministic missing-field detection
- declared defaults only
- bounded fixed-point iteration
- patch-by-patch repair witnesses
- replay from the original document
- visible non-convergence when a repair is impossible

Run `cargo test`.

I am publishing this as the working primitive, while the recursive/self-repair research stays clearly labeled in the research canon.

