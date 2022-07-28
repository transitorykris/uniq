use std::process::ExitCode;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_parser)]
    name: Option<String>,

    #[clap(short, long)]
    ignore_case: bool,

    #[clap(short,long)]
    count: bool,

    // TODO: other feature flags of a typical uniq implementation
}

fn main() -> ExitCode {
    let args = Args::parse();

    let mut u = uniq::Uniq::new();
    match args.name {
        Some(f) => {
            u = match uniq::Uniq::from_file(f) {
                Ok(u) => u,
                Err(_) => return ExitCode::from(2),
            };
        }
        None => {}
    };

    if args.ignore_case {
        u.ignore_case = true;
    }

    if args.count {
        u.count = true;
    }

    match u.run() {
        Ok(_) => {}
        Err(_) => {
            // TODO: return proper values here
            return ExitCode::from(1);
        }
    };
    ExitCode::from(0)
}
