use std::process::ExitCode;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_parser)]
    name: Option<String>,

    #[clap(short)]
   case: bool,

   // TODO: other feature flags of a typical uniq implementation
}

fn main() -> ExitCode {
    let args = Args::parse();
    
    let filename = match args.name {
        Some(f) => f,
        None => "/dev/stdin".to_string(),   // TODO use standard lib stdio
    };

    let u = uniq::Uniq::from_file(filename, args.case);

    match u.run() {
        Ok(_) => {},
        Err(_) => {
            // TODO: return proper values here
            return ExitCode::from(1);
        },
    };
    ExitCode::from(0)
}