use deterministic_agent_kernel::{stable_hash, verify_replay, Action, Capability, Kernel, Policy};

fn main() {
    let policy = Policy::default()
        .allow(Capability::Read)
        .allow(Capability::Write);
    let mut kernel = Kernel::new(policy);
    for (capability, resource) in [
        (Capability::Read, "vault/research.md"),
        (Capability::Network, "unadmitted-remote"),
    ] {
        kernel.decide(Action {
            capability,
            resource: resource.into(),
            payload_digest: stable_hash(resource.as_bytes()),
        });
    }
    for receipt in kernel.receipts() {
        println!(
            "{} {:?} {:016x}",
            receipt.sequence, receipt.outcome, receipt.hash
        );
    }
    verify_replay(kernel.receipts()).expect("receipt chain must replay");
}
