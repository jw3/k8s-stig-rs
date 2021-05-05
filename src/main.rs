use rules::*;

mod rules;

fn main() -> Result<(), String> {
    let cfg = RuleConf { wd: "/var/snap/microk8s/current".into() };
    let all: Vec<Box<dyn Rule<ErrType=String>>> = vec![Box::new(K220(&cfg)), Box::new(K2620(&cfg))];

    let mut failures = 0;
    for rule in all {
        match rule.check() {
            Ok(_) => {
                println!("\u{2714} {} pass", rule.id());
            }
            Err(e) => {
                println!("\u{274C} {} {}", rule.id(), e);
                failures += 1;
            }
        }
    }

    if failures > 0 {
        Err(format!("{} failures", failures))
    } else { Ok(()) }
}
