// derive macro
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "My RPN program",
    version = "1.0.0",
    author = "N.N.",
    about = "Super awesome sample RPN calculator"
)]
struct Opts {
    #[arg(short, long)]
    verbose: bool,

    #[arg(value_name = "FILE")]
    formula_file: Option<String>,
}

fn main() {
    let opts = Opts::parse();

    if let Some(file) = opts.formula_file {
        println!("File specified: {}", file);
    } else {
        println!("No file specified");
    }

    println!("Is verbosity specified? {}", opts.verbose);
}

// build-deps clap
/*
use clap::{Arg, ArgAction, Command};

fn main() {
    let matches = Command::new("My RPN program")
        .version("1.0.0")
        .author("N.N.")
        .about("Super awesome sample RPN calculator")
        .arg(
            Arg::new("formula_file")
                .help("Formulas written in RPN")
                .value_name("FILE")
                .index(1)
                .required(false),
        )
        .arg(
            Arg::new("verbose")
                .help("Sets the level of verbosity")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    match matches.get_one::<String>("formula_file") {
        Some(file) => println!("File specified: {}", file),
        None => println!("No file specified"),
    }

    let verbose = matches.get_flag("verbose");
    println!("Is verbosity specified? {}", verbose);
}
*/
