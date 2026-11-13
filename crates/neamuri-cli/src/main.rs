use neamuri_core::{Sex, classify, parse_step, term};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("--version") {
        println!("neamuri {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    // neamuri term LANG male|female STEP...
    if args.len() >= 3 && args[0] == "term" {
        let sex = match args[2].as_str() {
            "male" => Sex::Male,
            "female" => Sex::Female,
            _ => return usage(),
        };
        let steps: Option<Vec<_>> = args[3..].iter().map(|s| parse_step(s)).collect();
        if let Some(kin) = steps.as_deref().and_then(classify) {
            match term(&args[1], kin, sex) {
                Some(t) => return println!("{t}"),
                None => eprintln!("unknown language: {}", args[1]),
            }
        } else {
            eprintln!("path not covered yet");
        }
        std::process::exit(1);
    }
    usage();
}

fn usage() {
    eprintln!("usage: neamuri term LANG male|female STEP... | --version");
    std::process::exit(2);
}
