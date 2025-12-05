use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;
use syn::{Expr, LitStr};

pub fn test_impl(input: TokenStream) -> TokenStream {
    // Custom parser for new test macro syntax
    struct TestCase {
        solutions: Vec<Expr>,
        expected: LitStr,
    }

    enum InputSpec {
        File(LitStr),
        Text(LitStr),
    }

    struct TestInput {
        input_spec: InputSpec,
        cases: Vec<TestCase>,
    }

    impl Parse for TestInput {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            // Accept either `file = "path"` or `text = "..."` here, or the legacy bare string literal (file path).
            let input_spec = if input.peek(syn::Ident) {
                let ident: syn::Ident = input.parse()?;
                input.parse::<syn::Token![=]>()?;
                let val: LitStr = input.parse()?;
                match ident.to_string().as_str() {
                    "file" => InputSpec::File(val),
                    "text" => InputSpec::Text(val),
                    other => {
                        return Err(syn::Error::new_spanned(
                            ident,
                            format!("expected `file` or `text`, got `{}`", other),
                        ));
                    }
                }
            } else {
                // Legacy form: just a string literal which is treated as a file path
                let val: LitStr = input.parse()?;
                InputSpec::File(val)
            };

            let mut cases = Vec::new();

            while !input.is_empty() {
                input.parse::<syn::Token![,]>()?;

                // Parse [solution1, solution2, ...]
                let content;
                syn::bracketed!(content in input);
                let solutions: Vec<Expr> = content
                    .parse_terminated(Expr::parse, syn::Token![,])?
                    .into_iter()
                    .collect();

                input.parse::<syn::Token![=>]>()?;
                let expected: LitStr = input.parse()?;

                cases.push(TestCase {
                    solutions,
                    expected,
                });
            }

            Ok(TestInput { input_spec, cases })
        }
    }

    let test_input = parse_macro_input!(input as TestInput);
    let input_spec = test_input.input_spec;

    // Build a unique source tag based on input type and content/path to avoid name collisions.
    fn short_hash(s: &str) -> String {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        s.hash(&mut h);
        let v = h.finish();
        format!("{:x}", v)[..8].to_string()
    }
    let source_tag_str = match &input_spec {
        InputSpec::File(p) => {
            let path = p.value();
            let hash = short_hash(&path);
            // Take last component for readability
            let last = path.rsplit('/').next().unwrap_or(&path);
            let sanitized: String = last
                .chars()
                .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
                .collect();
            format!("file_{}_{}", sanitized, hash)
        }
        InputSpec::Text(t) => {
            let text = t.value();
            let hash = short_hash(&text);
            format!("text_{}", hash)
        }
    };

    // Precompute the input binding tokens (same for all cases in this macro invocation)
    let input_binding = match &input_spec {
        InputSpec::File(p) => {
            let p_lit = p.clone();
            quote! { let input = std::fs::read_to_string(#p_lit).unwrap_or_else(|e| panic!("failed to read {}: {}", #p_lit, e)); }
        }
        InputSpec::Text(t) => {
            let t_lit = t.clone();
            quote! { let input = #t_lit.to_string(); }
        }
    };

    // Generate test functions; include case and solution indices for full uniqueness.
    let test_functions: Vec<_> = test_input
        .cases
        .iter()
        .flat_map(|test_case| {
            let expected = test_case.expected.value();
            let source_tag_str = source_tag_str.clone();
            let input_binding_outer = input_binding.clone();

            test_case
                .solutions
                .iter()
                .map(move |solution_expr| {
                    let name_str = crate::expr_to_string(solution_expr);
                    let test_name = quote::format_ident!("test_{}_{}", source_tag_str, name_str);
                    let name_lit = name_str.clone();
                    let expected_lit = expected.clone();
                    let input_binding_clone = input_binding_outer.clone();

                    quote! {
                        #[test]
                        fn #test_name() {
                            #input_binding_clone
                            let entry = crate::__aoc::get(crate::__aoc::DAY, #name_lit)
                                .unwrap_or_else(|| panic!("solution {} not found", #name_lit));
                            let result = (entry.func)(&input);
                            assert_eq!(result, #expected_lit, "test failed for {}", #name_lit);
                        }
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let expanded = quote! {
        #(#test_functions)*
    };

    TokenStream::from(expanded)
}
