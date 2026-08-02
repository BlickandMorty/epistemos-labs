use confidence_ladder_monitor::{ConfidenceLadder, LadderPolicy};

fn main() {
    let mut escalate_on_empty = false;
    let mut scores = Vec::new();
    for argument in std::env::args().skip(1) {
        if argument == "--escalate" {
            escalate_on_empty = true;
        } else {
            match argument.parse::<f64>() {
                Ok(score) => scores.push(score),
                Err(error) => {
                    eprintln!("invalid score {argument:?}: {error}");
                    std::process::exit(2);
                }
            }
        }
    }
    if scores.is_empty() {
        eprintln!("usage: confidence-ladder-monitor [--escalate] SCORE...");
        std::process::exit(2);
    }

    let mut ladder =
        ConfidenceLadder::new(LadderPolicy::default()).expect("default policy is valid");
    for score in scores {
        match ladder.observe(score, escalate_on_empty) {
            Ok(entry) => println!(
                "sequence={} score={:.4} decision={:?} receipt={}",
                entry.sequence, entry.score, entry.decision, entry.receipt_hash
            ),
            Err(error) => {
                eprintln!("rejected score {score}: {error}");
                std::process::exit(2);
            }
        }
    }
    println!("health={:?} verified={}", ladder.health(), ladder.verify());
}
