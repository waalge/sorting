use regex::Regex;
use std::fs;
use std::error::Error;

pub mod gen_data;

pub mod sort;

fn to_string(x: usize) -> String {
    // TODO : Work out how this is supposed to work
    ToString::to_string(&x)
}

pub fn run_sort(
    algo: &str,
    input: &str,
    output: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let vec = parse_file(input);
    let res = sort::run(algo, vec);
    match output {
        Some(output) => {
            fs::write(
                output,
                res.into_iter()
                    .map(to_string)
                    .collect::<Vec<String>>()
                    .join("\n")
                    .as_bytes(),
            ).expect("Could not ouput to file");
            Ok(())
        }
        None => {
            println!("{:#?}", res);
            Ok(())
        }
    }
}

pub fn parse_file(input: &str) -> Vec<usize> {
    let contents = fs::read_to_string(input)
        .expect("Could not read file");

    let re = Regex::new(r"([0-9]+)")
        .expect("Unable to create regex pattern");
    let mut v: Vec<usize> = Vec::new();
    for field in re.find_iter(&contents) {
        let u: usize = field
            .as_str()
            .parse()
            .expect("Looking for digits");
        v.push(u);
    }
    v
}
