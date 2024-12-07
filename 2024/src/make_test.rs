#[macro_export]
macro_rules! make_test {
    (
        [$($function:ident),*]
        =>
        $input:expr, $expected_example:expr, $expected_final:expr
    ) => {
        $(
            paste::paste! {
            #[test]
            fn [<test_ $function _example>]() {
                assert_eq!($function(&parse(EXAMPLE)).to_string(), $expected_example.to_string());
            }

            #[test]
            fn [<test_ $function _final>]() {
                assert_eq!(
                    $function(&parse(include_str!(concat!("../input/2024/", $input)))).to_string(),
                    $expected_final.to_string()
                );
            }
        }
        )*
    }
}
