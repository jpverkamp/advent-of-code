use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, ItemFn, Token, parse::Parser, parse_macro_input};

// Function-like macro: aoc::render_image!(width, height, filename, |x, y| { /* return (r, g, b) */ })
pub fn render_image_impl(input: TokenStream) -> TokenStream {
    let parser = syn::punctuated::Punctuated::<Expr, Token![,]>::parse_terminated;
    let args = parser
        .parse(input.into())
        .expect("expected aoc::render_image!(width, height, filename, closure)");

    assert!(
        args.len() == 4,
        "expected 4 arguments to aoc::render_image!"
    );

    let width_expr = &args[0];
    let height_expr = &args[1];
    let filename_expr = &args[2];
    let closure_expr = &args[3];

    let expanded = quote! {
        {
            let width = #width_expr as usize;
            let height = #height_expr as usize;
            let filename = #filename_expr;
            let f = #closure_expr;

            let mut img = ::image::RgbImage::new(width as u32, height as u32);
            for y in 0..height {
                for x in 0..width {
                    let (r, g, b) = f(x, y);
                    img.put_pixel(x as u32, y as u32, ::image::Rgb([r, g, b]));
                }
            }
            img.save(filename).expect("failed to save image");
        }
    };

    TokenStream::from(expanded)
}

// Function-like macro: aoc::render_frame!(width, height, |x, y| { /* return (r, g, b) */ })
pub fn render_frame_impl(input: TokenStream) -> TokenStream {
    let parser = syn::punctuated::Punctuated::<Expr, Token![,]>::parse_terminated;
    let args = parser
        .parse(input.into())
        .expect("expected aoc::render_frame!(width, height, closure)");

    assert!(
        args.len() == 3,
        "expected 3 arguments to aoc::render_frame!"
    );

    let width_expr = &args[0];
    let height_expr = &args[1];
    let closure_expr = &args[2];

    let expanded = quote! {
        {
            let width = #width_expr as usize;
            let height = #height_expr as usize;
            let f = #closure_expr;

            let mut frame = ::image::RgbImage::new(width as u32, height as u32);
            for y in 0..height {
                for x in 0..width {
                    let (r, g, b) = f(x, y);
                    frame.put_pixel(x as u32, y as u32, ::image::Rgb([r, g, b]));
                }
            }

            // Store frame in the shared thread-local
            // This is a special function that all render code can call to access the frames
            __aoc_render_frames_push(frame);
        }
    };

    TokenStream::from(expanded)
}
