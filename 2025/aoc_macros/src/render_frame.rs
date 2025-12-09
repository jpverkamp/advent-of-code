use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::{Expr, Token};

pub fn render_frame_impl(input: TokenStream) -> TokenStream {
    let parser = syn::punctuated::Punctuated::<Expr, Token![,]>::parse_terminated;
    let args = parser
        .parse(input.into())
        .expect("expected aoc::render_frame!(width, height, closure) or aoc::render_frame!(width, height, closure, force = true)");

    assert!(
        args.len() == 3 || args.len() == 4,
        "expected 3 or 4 arguments to aoc::render_frame!(width, height, closure[, force = true])"
    );

    let width_expr = &args[0];
    let height_expr = &args[1];
    let closure_expr = &args[2];

    // Parse optional force parameter
    let force_check = if args.len() > 3 {
        match &args[3] {
            Expr::Assign(syn::ExprAssign { right, .. }) => {
                // Check if it's a boolean literal
                match right.as_ref() {
                    Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Bool(lit_bool),
                        ..
                    }) => {
                        let val = lit_bool.value;
                        if val {
                            quote! { true }
                        } else {
                            quote! { __aoc_should_render_frame() }
                        }
                    }
                    _ => quote! { __aoc_should_render_frame() },
                }
            }
            _ => quote! { __aoc_should_render_frame() },
        }
    } else {
        quote! { __aoc_should_render_frame() }
    };

    let expanded = quote! {
        {
            if #force_check {
                let width = #width_expr;
                let height = #height_expr;
                let f = #closure_expr;

                let mut frame = ::image::RgbImage::new(width as u32, height as u32);
                for y in 0..height {
                    for x in 0..width {
                        let (r, g, b) = f(x, y);
                        frame.put_pixel(x as u32, y as u32, ::image::Rgb([r, g, b]));
                    }
                }

                // Store frame in the shared thread-local
                __aoc_render_frames_push(frame);
            }
        }
    };

    TokenStream::from(expanded)
}


pub fn render_svg_frame_impl(input: TokenStream) -> TokenStream {
    let parser = syn::punctuated::Punctuated::<Expr, Token![,]>::parse_terminated;
    let args = parser
        .parse(input.into())
        .expect("expected aoc::render_svg_frame!(width, height, svg_data) or aoc::render_svg_frame!(width, height, svg_data, force = true)");

    assert!(
        args.len() == 3 || args.len() == 4,
        "expected 3 or 4 arguments to aoc::render_svg_frame!(width, height, svg_data[, force = true])"
    );

    let width_expr = &args[0];
    let height_expr = &args[1];
    let svg_data_expr = &args[2];

    // Parse optional force parameter
    let force_check = if args.len() > 3 {
        match &args[3] {
            Expr::Assign(syn::ExprAssign { right, .. }) => {
                // Check if it's a boolean literal
                match right.as_ref() {
                    Expr::Lit(syn::ExprLit {
                        lit: syn::Lit::Bool(lit_bool),
                        ..
                    }) => {
                        let val = lit_bool.value;
                        if val {
                            quote! { true }
                        } else {
                            quote! { __aoc_should_render_frame() }
                        }
                    }
                    _ => quote! { __aoc_should_render_frame() },
                }
            }
            _ => quote! { __aoc_should_render_frame() },
        }
    } else {
        quote! { __aoc_should_render_frame() }
    };

    let expanded = quote! {
        {
            if #force_check {
                let width = #width_expr;
                let height = #height_expr;
                let svg_data = #svg_data_expr;

                // Use ImageMagick to rasterize SVG to PNG
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs();
                let temp_dir = std::path::PathBuf::from(format!("/tmp/aoc_render/{}", timestamp));
                std::fs::create_dir_all(&temp_dir).expect("failed to create temp dir");

                let svg_file = temp_dir.join("frame.svg");
                let png_file = temp_dir.join("frame.png");
                
                std::fs::write(&svg_file, &svg_data).expect("failed to write SVG frame");

                // Use ImageMagick's magick command to convert SVG to PNG with dimensions
                let mut cmd = std::process::Command::new("magick");
                cmd.current_dir(&temp_dir)
                    .arg("frame.svg")
                    .arg("-resize")
                    .arg(format!("{}x{}", width, height))
                    .arg("frame.png");

                let output = cmd.output().expect("failed to execute magick");

                if !output.status.success() {
                    log::error!("magick conversion failed: {}", String::from_utf8_lossy(&output.stderr));
                    panic!("failed to rasterize SVG with ImageMagick");
                }

                let frame = ::image::open(&png_file)
                    .expect("failed to load rasterized PNG")
                    .to_rgb8();

                __aoc_render_frames_push(frame);
            }
        }
    };

    TokenStream::from(expanded)
}
