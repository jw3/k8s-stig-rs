use clap::Clap;

use rules::*;

mod rules;

#[derive(Clap)]
struct Opts {
    /// microk8s current dir
    #[clap(long, default_value = "/var/snap/microk8s/current")]
    current_dir: String
}

fn main() -> Result<(), String> {
    let opts = Opts::parse();

    let cfg = RuleConf { wd: opts.current_dir };
    let all: Vec<Box<dyn Rule<ErrType=String>>> = vec![Box::new(K220(&cfg)), Box::new(K2620(&cfg))];

    let mut failures = 0;
    for rule in all {
        match rule.check() {
            Ok(_) => {
                println!("\u{2705} {} pass", rule.id());
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
