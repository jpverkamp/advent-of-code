use proc_macro::TokenStream;
use quote::quote;
use syn::Expr;
use syn::parse_macro_input;

pub fn main_impl(input: TokenStream) -> TokenStream {
    let day_expr = parse_macro_input!(input as Expr);
    let day_str = crate::expr_to_string(&day_expr);

    let expanded = quote! {
        #[doc(hidden)]
        mod __aoc {
            use std::sync::{Mutex, OnceLock};
            pub const DAY: &str = #day_str;
            pub struct Entry { pub day: &'static str, pub name: &'static str, pub func: fn(&str) -> String }
            pub struct RenderEntry { pub day: &'static str, pub name: &'static str, pub func: fn(&str) }

            static REGISTRY: OnceLock<Mutex<Vec<&'static Entry>>> = OnceLock::new();
            static RENDER_REGISTRY: OnceLock<Mutex<Vec<&'static RenderEntry>>> = OnceLock::new();

            pub fn register(e: &'static Entry) { let reg = REGISTRY.get_or_init(|| Mutex::new(Vec::new())); reg.lock().unwrap().push(e); }
            pub fn register_render(e: &'static RenderEntry) { let reg = RENDER_REGISTRY.get_or_init(|| Mutex::new(Vec::new())); reg.lock().unwrap().push(e); }

            pub fn entries_for_day(day: &str) -> Vec<&'static Entry> {
                let reg = REGISTRY.get_or_init(|| Mutex::new(Vec::new()));
                let mut v: Vec<&'static Entry> = reg.lock().unwrap().iter().copied().filter(|e| e.day == day).collect();
                v.sort_by(|a,b| a.name.cmp(b.name)); v
            }

            pub fn render_entries_for_day(day: &str) -> Vec<&'static RenderEntry> {
                let reg = RENDER_REGISTRY.get_or_init(|| Mutex::new(Vec::new()));
                let mut v: Vec<&'static RenderEntry> = reg.lock().unwrap().iter().copied().filter(|e| e.day == day).collect();
                v.sort_by(|a,b| a.name.cmp(b.name)); v
            }

            pub fn get(day: &str, name: &str) -> Option<&'static Entry> {
                let reg = REGISTRY.get_or_init(|| Mutex::new(Vec::new()));
                reg.lock().unwrap().iter().copied().find(|e| e.day == day && e.name == name)
            }

            pub fn get_render(day: &str, name: &str) -> Option<&'static RenderEntry> {
                let reg = RENDER_REGISTRY.get_or_init(|| Mutex::new(Vec::new()));
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
                Render(RenderArgs),
                Run(RunArgs),
                Bench(BenchArgs),
            }

            #[derive(Args)]
            struct RenderArgs {
                /// Run all registered render solutions for the day
                #[arg(long)]
                all: bool,
                /// Name of the render solution to run (if not using --all)
                name: Option<String>,
                /// Input path (use '-' for stdin)
                input: Option<String>,
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
                    let render_entries = crate::__aoc::render_entries_for_day(day);
                    if entries.is_empty() && render_entries.is_empty() {
                        println!("No solutions registered for {}", day);
                    } else {
                        if !entries.is_empty() {
                            println!("Solutions:");
                            for e in entries { println!("  {}", e.name); }
                        }
                        if !render_entries.is_empty() {
                            println!("\nRender:");
                            for e in render_entries { println!("  {}", e.name); }
                        }
                    }
                }
                Commands::Render(args) => {
                    if args.all {
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
                        let entries = crate::__aoc::render_entries_for_day(day);
                        if entries.is_empty() { eprintln!("No render solutions registered for {}", day); std::process::exit(3); }
                        for e in entries { (e.func)(&input); }
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
                        match crate::__aoc::get_render(day, &name) { Some(entry) => { (entry.func)(&input); } None => { eprintln!("No such render solution: {}. Try 'list'.", name); std::process::exit(3); } }
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
