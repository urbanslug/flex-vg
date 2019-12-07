use clap::{App, Arg, SubCommand};
use std::env;

// Env vars
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

pub fn start() {
    let matches = App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .long("debug")
                .help("print debug information verbosely"),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .subcommand(
            SubCommand::with_name("construct")
                .about("Create a graph out of a reference and a vcf")
                .version(VERSION)
                .author(AUTHORS)
                .arg(
                    Arg::with_name("REFERENCE")
                        .help("Sets the reference file to use")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("VCF")
                        .help("Sets the VCF file to use")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            SubCommand::with_name("update")
                .about("Update an existing graph with variation data")
                .version(VERSION)
                .author(AUTHORS)
                .arg(
                    Arg::with_name("GRAPH")
                        .help("Sets the graph file to use")
                        .required(true)
                        .index(1),
                )
                .arg(
                    Arg::with_name("VCF")
                        .help("Sets the VCF file to use")
                        .required(true)
                        .index(2),
                ),
        )
        .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }
}
