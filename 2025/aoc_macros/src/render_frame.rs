use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::{Expr, Token};

pub fn render_frame_impl(input: TokenStream) -> TokenStream {
    let parser = syn::punctuated::Punctuated::<Expr, Token![,]>::parse_terminated;
    let args = parser
        .parse(input.into())
        .expect("expected aoc::render_frame!(width, height, closure)");

    assert!(
        args.len() == 3,
        "expected 3 arguments to aoc::render_frame!(width, height, closure)"
    );

    let width_expr = &args[0];
    let height_expr = &args[1];
    let closure_expr = &args[2];

    let expanded = quote! {
        {
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
            // This is a special function that all render code can call to access the frames
            __aoc_render_frames_push(frame);
        }
    };

    TokenStream::from(expanded)
}
