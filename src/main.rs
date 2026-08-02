use interrupt_score_router::{Router, Signals, Thresholds, Weights, audit_escalation};

fn main() {
    let mut router = Router::new(
        Weights::UNIFORM,
        Thresholds {
            recall: 0.25,
            escalate: 0.70,
        },
    )
    .expect("static policy is valid");
    for value in [0.10, 0.40, 0.85] {
        let receipt = router
            .route(Signals {
                predictive_entropy: value,
                budget_overrun_risk: value,
                consistency_residual: value,
                tool_need: value,
                component_alarm: value,
            })
            .expect("demo signal is valid");
        println!(
            "{}",
            serde_json::to_string(receipt).expect("receipt serializes")
        );
    }
    eprintln!(
        "audit={:?}",
        audit_escalation(router.receipts(), 0.20).expect("audit bound is valid")
    );
}
