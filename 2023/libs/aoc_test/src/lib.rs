#[macro_export]
macro_rules! generate {
    ($name:tt as $input_file:tt => $expected:tt) => {
        #[cfg(test)]
        mod $name {
            use super::*;

            #[test]
            fn run_test() {
                let filename = format!("../../data/{}", $input_file);
                let input = std::fs::read_to_string(filename).unwrap();
                let actual = process(input.as_str()).unwrap();
                assert_eq!(actual, $expected);
            }
        }        
    };
}
