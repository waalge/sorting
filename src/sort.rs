// Handle and run different sort algorithms

mod bubble;
mod quick;

pub fn run(algo: &str, vec: Vec<usize>) -> Vec<usize> {
    match algo {
        "bubble" => {
            bubble::run(vec)
        }
        "quick" => {
            quick::run(vec)
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
    use rand::{thread_rng, seq::SliceRandom};
    use std::time::Instant;
    // TODO : Use cargo bench if/when use nightly.

    enum Size {
        Small,
        Medium,
        Large
    }

    fn build_test(algo: &str, size: Size) {
        let (src, trg) = match size {
            Size::Small => (vec![3, 2, 1], vec![1, 2, 3]),
            Size::Medium => { 
                (vec![5, 0, 8, 18, 11, 17, 9, 16, 15, 4, 3, 1, 2, 7, 19, 12, 14, 10, 6, 13], 
                 (0..20).collect())
            }
            Size::Large => {
                let mut src : Vec<usize> = (0..10000).collect();
                let mut rng = thread_rng();
                src.shuffle(&mut rng);
                ( src, (0..10000).collect() ) 
            }
        };
        let res = run(algo, src);
        assert_eq!(
            trg.iter()
                .zip(&res)
                .filter(|&(trg, res)| trg != res)
                .count(),
            0
        );
    }

    fn timeit(f : fn() -> ()) {
        let start = Instant::now();
        f();
        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);
    }


    #[test]
    fn bubble_small() {
        build_test("bubble", Size::Small);
    }

    #[test]
    fn bubble_medium() {
        build_test("bubble", Size::Medium);
    }

    fn bubble_large_setup() {
        build_test("bubble", Size::Large);
    }

    #[test]
    fn bubble_large() {
        timeit(bubble_large_setup);
    }

    #[test]
    fn quick_small() {
        build_test("quick", Size::Small);
    }

    #[test]
    fn quick_medium() {
        build_test("quick", Size::Medium);
    }

    fn quick_large_setup() {
        build_test("quick", Size::Large);
    }

    #[test]
    fn quick_large() {
        timeit(quick_large_setup);
    }

}

