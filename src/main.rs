use clap::{App, Arg, ArgMatches, SubCommand};

use sorting::{gen_data, run_sort};

fn main() {
    let matches = get_args();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let _ = match matches.subcommand() {
        ("gen_data", Some(gen_data_matches)) => {
            let (output, len) =
                parse_args_gen_data(gen_data_matches);
            gen_data::gen_data(output, len)
        }
        ("sort", Some(sort_matches)) => {
            println!("This is a sort");
            let (algo, input, output) =
                parse_args_sort(sort_matches);
            run_sort(algo, input, output)
        }
        _ => {
            println!("No valid subcommand. Get help");
            std::process::exit(1);
        }
    };
}

fn get_args() -> ArgMatches<'static> {
    App::new("sorting")
        .version("0.1")
        .author("me")
        .about("simple sort algos to learn some rust")
        .subcommand(
            SubCommand::with_name("gen_data")
                .arg(
                    Arg::with_name("len")
                        .value_name("len")
                        .help("Number of numbers")
                        .required(true),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .value_name("output")
                        .help("Output file, else stdout")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("sort")
                .arg(
                    Arg::with_name("algo")
                        .short("a")
                        .long("algo")
                        .value_name("algo")
                        .help("Choose an algorithm")
                        .required(true),
                )
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .value_name("input")
                        .help("Source of data")
                        .required(true),
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .value_name("output")
                        .help("Output file, else stdout"),
                ),
        )
        .get_matches()
}

fn parse_args_gen_data<'a>(
    matches: &'a ArgMatches,
) -> (&'a str, usize) {
    let output = matches.value_of("output").unwrap();
    let len: usize = matches
        .value_of("len")
        .unwrap()
        .parse::<usize>()
        .expect("Cant parse as int");
    (output, len)
}

fn parse_args_sort<'a>(
    matches: &'a ArgMatches,
) -> (&'a str, &'a str, Option<&'a str>) {
    let algo = matches.value_of("algo").unwrap();
    let input = matches.value_of("input").unwrap();
    let output = matches.value_of("output");
    (algo, input, output)
}
