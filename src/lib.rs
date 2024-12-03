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
