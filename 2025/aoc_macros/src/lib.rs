use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream, Parser},
    parse_macro_input, Expr, ItemFn, LitStr, Token,
};

fn expr_to_string(expr: &Expr) -> String {
    match expr {
        Expr::Lit(l) => l
            .lit
            .to_token_stream()
            .to_string()
            .trim_matches('"')
            .to_string(),
        Expr::Path(p) => p.path.to_token_stream().to_string(),
        other => other.to_token_stream().to_string(),
    }
}

// Attribute macro: #[aoc::register(day, name)]
#[proc_macro_attribute]
pub fn register(attr: TokenStream, item: TokenStream) -> TokenStream {
    let parser = syn::punctuated::Punctuated::<Expr, Token![,]>::parse_terminated;
    let args = parser
        .parse(attr.into())
        .expect("expected #[aoc::register(day0, name)]");
    assert!(
        args.len() == 2,
        "expected two arguments to #[aoc::register(day0, name)]"
    );

    let day_str = expr_to_string(&args[0]);
    let name_str = expr_to_string(&args[1]);

    let func = parse_macro_input!(item as ItemFn);
    let fn_name = func.sig.ident.clone();

    let shim_ident: Ident = format_ident!("__aoc_shim_{}", fn_name);
    let entry_ident: Ident = format_ident!("__AOC_ENTRY_{}", fn_name.to_string().to_uppercase());
    let reg_ident: Ident = format_ident!("__aoc_register_{}", fn_name);

    let day_lit = day_str;
    let name_lit = name_str;

    let expanded = quote! {
        #func

        #[doc(hidden)]
        fn #shim_ident(input: &str) -> String { #fn_name(input).into() }

        #[doc(hidden)]
        static #entry_ident: crate::__aoc::Entry = crate::__aoc::Entry { day: #day_lit, name: #name_lit, func: #shim_ident };

        #[doc(hidden)]
        #[::ctor::ctor]
        fn #reg_ident() { crate::__aoc::register(&#entry_ident); }
    };

    TokenStream::from(expanded)
}

// Function-like macro to define the machine function/CLI parsing: aoc::main!(day)
#[proc_macro]
pub fn main(input: TokenStream) -> TokenStream {
    let day_expr = parse_macro_input!(input as Expr);
    let day_str = expr_to_string(&day_expr);

    let expanded = quote! {
        #[doc(hidden)]
        mod __aoc {
            use std::sync::{Mutex, OnceLock};
            pub struct Entry { pub day: &'static str, pub name: &'static str, pub func: fn(&str) -> String }

            static REGISTRY: OnceLock<Mutex<Vec<&'static Entry>>> = OnceLock::new();

            pub fn register(e: &'static Entry) { let reg = REGISTRY.get_or_init(|| Mutex::new(Vec::new())); reg.lock().unwrap().push(e); }

            pub fn entries_for_day(day: &str) -> Vec<&'static Entry> {
                let reg = REGISTRY.get_or_init(|| Mutex::new(Vec::new()));
                let mut v: Vec<&'static Entry> = reg.lock().unwrap().iter().copied().filter(|e| e.day == day).collect();
                v.sort_by(|a,b| a.name.cmp(b.name)); v
            }

            pub fn get(day: &str, name: &str) -> Option<&'static Entry> {
                let reg = REGISTRY.get_or_init(|| Mutex::new(Vec::new()));
                reg.lock().unwrap().iter().copied().find(|e| e.day == day && e.name == name)
            }
        }

        fn main() {
            let tracing_enabled = std::env::var("RUST_TRACE").is_ok();
            if tracing_enabled {
                tracing_subscriber::fmt()
                    .pretty()
                    .without_time()
                    .with_max_level(tracing::Level::DEBUG)
                    .init();
                tracing::info!("Tracing enabled");
            } else {
                env_logger::init();
            }

            let day: &str = #day_str;

            use clap::{Parser, Subcommand, Args};

            #[derive(Parser)]
            #[command(name = env!("CARGO_PKG_NAME"))]
            struct Cli { #[command(subcommand)] command: Commands }

            #[derive(Subcommand)]
            enum Commands {
                List,
                Run(RunArgs),
                Bench(BenchArgs),
            }

            #[derive(Args)]
            struct RunArgs {
                /// Run all registered solutions for the day
                #[arg(long)]
                all: bool,
                /// Name of the solution to run (if not using --all)
                name: Option<String>,
                /// Input path (use '-' for stdin)
                input: Option<String>,
            }

            #[derive(Args)]
            struct BenchArgs {
                /// Run all registered solutions for the day
                #[arg(long)]
                all: bool,
                /// Name of the solution to bench (if not using --all)
                name: Option<String>,
                /// Input path (use '-' for stdin)
                input: Option<String>,
                /// Number of warmup runs (default: 3)
                #[arg(long, default_value_t = 3)]
                warmup: usize,
                /// Number of benchmark iterations (default: 100)
                #[arg(long, default_value_t = 100)]
                iters: usize,
            }

            let cli = Cli::parse();

            let read_input = |input_path: String| -> String {
                if input_path == "-" { use std::io::Read; let mut s = String::new(); std::io::stdin().read_to_string(&mut s).expect("failed to read stdin"); s } else { std::fs::read_to_string(&input_path).unwrap_or_else(|e| panic!("failed to read {}: {}", input_path, e)) }
            };

            let benchmark = |name: &str, func: fn(&str) -> String, input: &str, warmup: usize, iters: usize| {
                // Warmup
                for _ in 0..warmup { let _ = func(input); }

                // Benchmark
                let mut times = Vec::with_capacity(iters);
                for _ in 0..iters {
                    let start = std::time::Instant::now();
                    let _ = func(input);
                    times.push(start.elapsed());
                }

                times.sort();
                let min = times[0];
                let max = times[iters - 1];
                let median = times[iters / 2];

                // Compute average and standard deviation (in nanoseconds)
                let sum_ns: u128 = times.iter().map(|d| d.as_nanos()).sum();
                let avg_ns = sum_ns as f64 / (iters as f64);
                let avg = std::time::Duration::from_nanos(avg_ns as u64);

                let var_ns = times
                    .iter()
                    .map(|d| {
                        let x = d.as_nanos() as f64;
                        let diff = x - avg_ns;
                        diff * diff
                    })
                    .sum::<f64>()
                    / (iters as f64);
                let stddev_ns = var_ns.sqrt();
                let stddev = std::time::Duration::from_nanos(stddev_ns as u64);

                println!("{name}: {avg:?} ± {stddev:?} [min: {min:?}, max: {max:?}, median: {median:?}]");
            };

            match cli.command {
                Commands::List => {
                    let entries = crate::__aoc::entries_for_day(day);
                    if entries.is_empty() {
                        println!("No solutions registered for {}", day);
                    } else {
                        for e in entries { println!("{}", e.name); }
                    }
                }
                Commands::Run(args) => {
                    if args.all {
                        // If --all is provided, require an input path. For backward compatibility we
                        // accept that callers may have supplied the input as the positional `name`.
                        let input_path = match args.input {
                            Some(ip) => ip,
                            None => match args.name {
                                Some(pos) => pos,
                                None => {
                                    eprintln!("Missing input path for --all. Provide an input path (use '-' for stdin) or --input <FILE>");
                                    std::process::exit(2);
                                }
                            },
                        };
                        let input = read_input(input_path);
                        let entries = crate::__aoc::entries_for_day(day);
                        if entries.is_empty() { eprintln!("No solutions registered for {}", day); std::process::exit(3); }
                        for e in entries { let out = (e.func)(&input); println!("{}: {}", e.name, out); }
                    } else {
                        let name = match args.name {
                            Some(n) => n,
                            None => { eprintln!("Missing solution name. Try 'list' to see registered names."); std::process::exit(2); }
                        };
                        let input_path = match args.input {
                            Some(i) => i,
                            None => { eprintln!("Missing input path. Provide an input path (use '-' for stdin) or --input <FILE>"); std::process::exit(2); }
                        };
                        let input = read_input(input_path);
                        match crate::__aoc::get(day, &name) { Some(entry) => { let out = (entry.func)(&input); println!("{}", out); } None => { eprintln!("No such solution: {}. Try 'list'.", name); std::process::exit(3); } }
                    }
                }
                Commands::Bench(args) => {
                    // Validate iteration count
                    if args.iters == 0 { eprintln!("--iters must be >= 1"); std::process::exit(2); }

                    if args.all {
                        // If --all is used, require an input path (positional or via --input)
                        let input_path = match args.input {
                            Some(ip) => ip,
                            None => match args.name {
                                Some(pos) => pos,
                                None => { eprintln!("Missing input path for --all. Provide an input path (use '-' for stdin) or --input <FILE>"); std::process::exit(2); }
                            },
                        };
                        let input = read_input(input_path);
                        let entries = crate::__aoc::entries_for_day(day);
                        if entries.is_empty() { eprintln!("No solutions registered for {}", day); std::process::exit(3); }
                        for e in entries { benchmark(e.name, e.func, &input, args.warmup, args.iters); }
                    } else {
                        let name = match args.name {
                            Some(n) => n,
                            None => { eprintln!("Missing solution name. Try 'list' to see registered names."); std::process::exit(2); }
                        };
                        let input_path = match args.input {
                            Some(i) => i,
                            None => { eprintln!("Missing input path. Provide an input path (use '-' for stdin) or --input <FILE>"); std::process::exit(2); }
                        };
                        let input = read_input(input_path);
                        match crate::__aoc::get(day, &name) { Some(entry) => { benchmark(entry.name, entry.func, &input, args.warmup, args.iters); } None => { eprintln!("No such solution: {}. Try 'list'.", name); std::process::exit(3); } }
                    }
                }
            }
        }
    };
    TokenStream::from(expanded)
}

// Function-like macro: aoc::test!(day, "input_path", [solution1, solution2, etc] => "expected", [solution] => "expected", ...)
#[proc_macro]
pub fn test(input: TokenStream) -> TokenStream {
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
        day: Expr,
        input_spec: InputSpec,
        cases: Vec<TestCase>,
    }
    
    impl Parse for TestInput {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let day = input.parse()?;
            input.parse::<Token![,]>()?;

            // Accept either `file = "path"` or `text = "..."` here, or the legacy bare string literal (file path).
            let input_spec = if input.peek(syn::Ident) {
                let ident: syn::Ident = input.parse()?;
                input.parse::<Token![=]>()?;
                let val: LitStr = input.parse()?;
                match ident.to_string().as_str() {
                    "file" => InputSpec::File(val),
                    "text" => InputSpec::Text(val),
                    other => return Err(syn::Error::new_spanned(ident, format!("expected `file` or `text`, got `{}`", other))),
                }
            } else {
                // Legacy form: just a string literal which is treated as a file path
                let val: LitStr = input.parse()?;
                InputSpec::File(val)
            };
            
            let mut cases = Vec::new();
            
            while !input.is_empty() {
                input.parse::<Token![,]>()?;
                
                // Parse [solution1, solution2, ...]
                let content;
                syn::bracketed!(content in input);
                let solutions: Vec<Expr> = content
                    .parse_terminated(Expr::parse, Token![,])?
                    .into_iter()
                    .collect();
                
                input.parse::<Token![=>]>()?;
                let expected: LitStr = input.parse()?;
                
                cases.push(TestCase { solutions, expected });
            }
            
            Ok(TestInput { day, input_spec, cases })
        }
    }
    
    let test_input = parse_macro_input!(input as TestInput);
    let day_str = expr_to_string(&test_input.day);
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
            let sanitized: String = last.chars().map(|c| if c.is_ascii_alphanumeric() { c } else { '_' }).collect();
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
        InputSpec::File(p) => { let p_lit = p.clone(); quote! { let input = std::fs::read_to_string(#p_lit).unwrap_or_else(|e| panic!("failed to read {}: {}", #p_lit, e)); } }
        InputSpec::Text(t) => { let t_lit = t.clone(); quote! { let input = #t_lit.to_string(); } }
    };

    // Generate test functions; include case and solution indices for full uniqueness.
    let test_functions: Vec<_> = test_input.cases.iter().flat_map(|test_case| {
        let expected = test_case.expected.value();
        let day_str = day_str.clone();
        let source_tag_str = source_tag_str.clone();
        let input_binding_outer = input_binding.clone();

        test_case.solutions.iter().map(move |solution_expr| {
            let name_str = expr_to_string(solution_expr);
            let test_name = format_ident!("test_{}_{}_{}", day_str, source_tag_str, name_str);
            let name_lit = name_str.clone();
            let expected_lit = expected.clone();
            let day_lit = day_str.clone();
            let input_binding_clone = input_binding_outer.clone();

            quote! {
                #[test]
                fn #test_name() {
                    #input_binding_clone
                    let entry = crate::__aoc::get(#day_lit, #name_lit)
                        .unwrap_or_else(|| panic!("solution {} not found", #name_lit));
                    let result = (entry.func)(&input);
                    assert_eq!(result, #expected_lit, "test failed for {}", #name_lit);
                }
            }
        }).collect::<Vec<_>>()
    }).collect();
    
    let expanded = quote! {
        #(#test_functions)*
    };
    
    TokenStream::from(expanded)
}
