use std::fs;
use std::error::Error;

use rand::Rng;

fn rng_vec(len: usize) -> Vec<usize> {
    let mut v = Vec::new();
    for _ii in 0..len {
        v.push(rand::thread_rng().gen_range(0, 10000));
    }
    v
}

fn to_string(x: usize) -> String {
    ToString::to_string(&x)
}

fn write_vec(
    output: &str,
    vec: Vec<usize>,
) -> Result<(), Box<dyn Error>> {
    match fs::write(
                output,
                vec.into_iter()
                    .map(to_string)
                    .collect::<Vec<String>>()
                    .join("\n")
                    .as_bytes(),
            ){
        Err(why) => {
            panic!("couldn't write to {}: {}", output, why);
        }
        Ok(_) => {
            println!("successfully wrote to {}", output);
        }
    };
    Ok(())
}

pub fn gen_data(
    output: &str,
    len: usize,
) -> Result<(), Box<dyn Error>> {
    write_vec(output, rng_vec(len))
}
