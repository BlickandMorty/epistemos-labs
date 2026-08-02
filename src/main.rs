use belnap_evidence_gate::{Evidence, Stance, evaluate};

fn main() {
    let mut args = std::env::args().skip(1);
    let claim = args.next().unwrap_or_else(|| "demo-claim".into());
    let supporting: usize = args
        .next()
        .and_then(|value| value.parse().ok())
        .unwrap_or(1);
    let refuting: usize = args
        .next()
        .and_then(|value| value.parse().ok())
        .unwrap_or(1);
    let mut evidence = Vec::new();
    for index in 0..supporting {
        evidence.push(Evidence {
            id: format!("support-{index}"),
            source_digest: format!("demo-s-{index}"),
            stance: Stance::Supports,
        });
    }
    for index in 0..refuting {
        evidence.push(Evidence {
            id: format!("refute-{index}"),
            source_digest: format!("demo-r-{index}"),
            stance: Stance::Refutes,
        });
    }
    let receipt = evaluate(claim, evidence).expect("demo evidence is valid");
    println!(
        "{}",
        serde_json::to_string_pretty(&receipt).expect("receipt serializes")
    );
}
