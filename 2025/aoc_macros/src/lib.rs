use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream, Parser},
    parse_macro_input, Expr, ItemFn, LitStr, Token,
};

mod render;

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

// Function-like macro: aoc::render_image!(width, height, filename, |x, y| { /* return (r, g, b) */ })
#[proc_macro]
pub fn render_image(input: TokenStream) -> TokenStream {
    render::render_image_impl(input)
}

// Function-like macro: aoc::render_frame!(width, height, |x, y| { /* return (r, g, b) */ })
#[proc_macro]
pub fn render_frame(input: TokenStream) -> TokenStream {
    render::render_frame_impl(input)
}

// Attribute macro: #[aoc::register_render(day, name)] or #[aoc::register_render(day, name, scale=N)]
// Registers a render function with the CLI and applies render logic with optional scaling
#[proc_macro_attribute]
pub fn register_render(attr: TokenStream, item: TokenStream) -> TokenStream {
    let parser = syn::punctuated::Punctuated::<Expr, Token![,]>::parse_terminated;
    let args = parser
        .parse(attr.into())
        .expect("expected #[aoc::register_render(day0, name)] or #[aoc::register_render(day0, name, scale=N)]");
    assert!(
        args.len() >= 2,
        "expected at least two arguments to #[aoc::register_render(day0, name)]"
    );

    let day_str = expr_to_string(&args[0]);
    let name_str = expr_to_string(&args[1]);

    // Parse optional scale parameter
    let scale_lit = if args.len() > 2 {
        // Handle both "4" and "scale = 4" syntax
        match &args[2] {
            Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(lit_int), .. }) => {
                // Direct integer: "4"
                lit_int.clone()
            }
            Expr::Assign(syn::ExprAssign { right, .. }) => {
                // Assignment: "scale = 4"
                if let Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(lit_int), .. }) = right.as_ref() {
                    lit_int.clone()
                } else {
                    syn::LitInt::new("1", proc_macro2::Span::call_site())
                }
            }
            _ => syn::LitInt::new("1", proc_macro2::Span::call_site())
        }
    } else {
        syn::LitInt::new("1", proc_macro2::Span::call_site())
    };

    // Parse optional fps parameter
    let fps_lit = if args.len() > 3 {
        // Handle both "30" and "fps = 30" syntax
        match &args[3] {
            Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(lit_int), .. }) => {
                // Direct integer: "30"
                lit_int.clone()
            }
            Expr::Assign(syn::ExprAssign { right, .. }) => {
                // Assignment: "fps = 30"
                if let Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(lit_int), .. }) = right.as_ref() {
                    lit_int.clone()
                } else {
                    syn::LitInt::new("30", proc_macro2::Span::call_site())
                }
            }
            _ => syn::LitInt::new("30", proc_macro2::Span::call_site())
        }
    } else {
        syn::LitInt::new("30", proc_macro2::Span::call_site())
    };

    let fn_item = parse_macro_input!(item as ItemFn);
    let fn_name = fn_item.sig.ident.clone();
    let fn_vis = &fn_item.vis;
    let fn_sig = &fn_item.sig;
    let fn_body = &fn_item.block;

    let shim_ident: Ident = format_ident!("__aoc_render_shim_{}", fn_name);
    let entry_ident: Ident = format_ident!("__AOC_RENDER_ENTRY_{}", fn_name.to_string().to_uppercase());
    let reg_ident: Ident = format_ident!("__aoc_register_render_{}", fn_name);

    let day_lit = day_str;
    let name_lit = name_str;

    let expanded = quote! {
        thread_local! {
            static __AOC_RENDER_FRAMES: std::cell::RefCell<Vec<::image::RgbImage>> = std::cell::RefCell::new(Vec::new());
        }

        #[allow(non_snake_case)]
        fn __aoc_render_frames_push(frame: ::image::RgbImage) {
            __AOC_RENDER_FRAMES.with(|frames| {
                frames.borrow_mut().push(frame);
            });
        }

        #[allow(non_snake_case)]
        fn __aoc_render_frames_take() -> Vec<::image::RgbImage> {
            __AOC_RENDER_FRAMES.with(|frames| {
                frames.borrow_mut().drain(..).collect()
            })
        }

        #fn_vis #fn_sig {
            // Execute the original function to collect frames
            #fn_body

            // Finalize video encoding using ffmpeg via subprocess
            let frames_vec = __aoc_render_frames_take();
            
            if !frames_vec.is_empty() {
                let first_frame = &frames_vec[0];
                let orig_width = first_frame.width() as usize;
                let orig_height = first_frame.height() as usize;
                let scale = #scale_lit as usize;

                let scaled_width = orig_width * scale;
                let scaled_height = orig_height * scale;

                let temp_dir = std::path::PathBuf::from("/tmp/aoc_render");
                std::fs::create_dir_all(&temp_dir).expect("failed to create temp dir");

                for (idx, frame) in frames_vec.iter().enumerate() {
                    let mut scaled_frame = ::image::RgbImage::new(
                        scaled_width as u32,
                        scaled_height as u32
                    );

                    for orig_y in 0..orig_height {
                        for orig_x in 0..orig_width {
                            let pixel = frame.get_pixel(orig_x as u32, orig_y as u32);
                            for dy in 0..scale {
                                for dx in 0..scale {
                                    let new_x = orig_x * scale + dx;
                                    let new_y = orig_y * scale + dy;
                                    scaled_frame.put_pixel(new_x as u32, new_y as u32, *pixel);
                                }
                            }
                        }
                    }

                    let frame_path = temp_dir.join(format!("frame_{:04}.png", idx));
                    scaled_frame.save(&frame_path).expect("failed to save frame");
                }

                // Use ffmpeg to create video from frames
                let output_path = format!("{}.mp4", stringify!(#fn_name));
                let frame_pattern = temp_dir.join("frame_%04d.png").to_string_lossy().to_string();
                let fps_str = format!("{}", #fps_lit);

                let output = std::process::Command::new("ffmpeg")
                    .arg("-framerate").arg(&fps_str)
                    .arg("-i").arg(&frame_pattern)
                    .arg("-c:v").arg("libx264")
                    .arg("-pix_fmt").arg("yuv420p")
                    .arg("-y")
                    .arg(&output_path)
                    .output()
                    .expect("failed to run ffmpeg");

                if !output.status.success() {
                    eprintln!("ffmpeg error: {}", String::from_utf8_lossy(&output.stderr));
                } else {
                    eprintln!("ffmpeg succeeded");
                }

                // Clean up temp frames
                for entry in std::fs::read_dir(&temp_dir).expect("failed to read temp dir") {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if path.extension().map_or(false, |ext| ext == "png") {
                            let _ = std::fs::remove_file(&path);
                        }
                    }
                }
            }
        }

        #[doc(hidden)]
        fn #shim_ident(input: &str) { #fn_name(input); }

        #[doc(hidden)]
        static #entry_ident: crate::__aoc::RenderEntry = crate::__aoc::RenderEntry { day: #day_lit, name: #name_lit, func: #shim_ident };

        #[doc(hidden)]
        #[::ctor::ctor]
        fn #reg_ident() { crate::__aoc::register_render(&#entry_ident); }
    };

    TokenStream::from(expanded)
}
