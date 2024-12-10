pub mod util;

#[macro_export]
macro_rules! aoc_day {
    ($t:ty, $expected_a:expr, $expected_b:expr) => {
        fn main() {
            let test_input = include_str!("test.txt");
            let main_input = include_str!("main.txt");
            $crate::run_aoc_day::<$t>(test_input, main_input, $expected_a, $expected_b);
        }
    };
}

pub trait AocDay {
    fn from(input: &str) -> Self;
    fn a(&self) -> i64;
    fn b(&self) -> i64;
}

pub fn run_aoc_day<T: AocDay>(
    test_input: &str,
    main_input: &str,
    expected_a: i64,
    expected_b: i64,
) {
    let test = T::from(&test_input);
    let main = T::from(&main_input);

    let test_a = test.a();
    let test_a_success = test_a == expected_a;
    println!(
        "Test A: {} {} {}",
        test_a,
        if test_a_success { "==" } else { "!=" },
        expected_a
    );
    if test_a_success {
        println!("Main B: {}", main.a());

        let test_b = test.b();
        let test_b_success = test_b == expected_b;
        println!(
            "Test B: {} {} {}",
            test_b,
            if test_b_success { "==" } else { "!=" },
            expected_b
        );
        if test_b_success {
            println!("Main B: {}", main.b());
        }
    }
}

#[macro_export]
macro_rules! init {
    ($expected_a:expr, $expected_b:expr) => {
        use std::fs::read_to_string;
        use std::path::Path;

        fn main() {
            let input = include_str!("main.txt");
            let (a, b) = try_run(input);
            print!("A: {}\nB: {}\n", a, b);
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test() {
                let input = include_str!("test.txt");
                let (a, b) = match input.split_once("=====") {
                    Some((input_a, input_b)) => (try_run(input_a).0, try_run(input_b).1),
                    None => try_run(input),
                };
                assert_eq!($expected_a, &a);
                assert_eq!($expected_b, &b);
            }
        }

        fn try_run(input: &str) -> (String, String) {
            match run(input) {
                Ok(v) => v,
                Err(e) => panic!("{}", e.to_string()),
            }
        }
    };
}
