// Handle and run different sort algorithms

mod bubble;

pub fn run(algo: &str, vec: Vec<usize>) -> Vec<usize> {
    match algo {
        "bubble" => {
            bubble::run(vec)
        }
        _ => {
            println!("I don't know how to do {}", algo);
            std::process::exit(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn small(algo: &str) {
        let src: Vec<usize> = vec![3, 2, 1];
        let trg: Vec<usize> = vec![1, 2, 3];
        let res = run(algo, src);
        assert_eq!(
            trg.iter()
                .zip(&res)
                .filter(|&(trg, res)| trg != res)
                .count(),
            0
        );
    }

    #[test]
    fn bubble() {
        small("bubble");
    }
}
