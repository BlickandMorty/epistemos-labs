# Architecture references and license boundary

This codebase is an original clean-room implementation. It borrows architectural ideas, not source code.

| Project | Pattern studied | Upstream license | What I used |
| --- | --- | --- | --- |
| [Shannon](https://github.com/KeygraphHQ/shannon) | source-aware analysis, phased orchestration, proof before reporting, isolated workers | AGPL-3.0 | evidence-gated reporting and phase separation, reimplemented from scratch |
| [OpenAEV](https://github.com/OpenAEV-Platform/openaev) | campaign scope, scheduling/control-plane separation, auditable execution | Community Edition: Apache-2.0; separately marked Enterprise files use their own license | explicit engagement scope and report boundary |
| [HackingBuddyGPT](https://github.com/ipa-lab/hackingBuddyGPT) | small capability-oriented agent tasks | MIT | bounded task/capability shape only |
| [PentAGI](https://github.com/vxcontrol/pentagi) | persistent orchestration and task history | MIT for the repository; cloud services have separate terms | deterministic task/report ordering only |

No AGPL source is included here. If a future version directly derives from AGPL code, it belongs in a separately identified AGPL-compatible repository with complete attribution and source availability.
