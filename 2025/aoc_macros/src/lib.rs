use proc_macro::TokenStream;

// Place #[aoc::main("dayX")] *once* to generate the main function for dayX.
mod main_macro;

#[proc_macro]
pub fn main(input: TokenStream) -> TokenStream {
    main_macro::main_impl(input)
}

// Function-like macro: aoc::test!("input_path", [solution1, solution2, etc] => "expected", [solution] => "expected", ...)
mod test;

#[proc_macro]
pub fn test(input: TokenStream) -> TokenStream {
    test::test_impl(input)
}

// Place #[aoc::register] to register it as an aoc solution.
mod register;

#[proc_macro_attribute]
pub fn register(_attr: TokenStream, item: TokenStream) -> TokenStream {
    register::register_impl(_attr, item)
}

// Macros for rendering images and frames.
// Place #[aoc::register_render(...)] to register a render function.
// Then use aoc::render_image! and aoc::render_frame! within that function.
// Both take (width, height, f) where f is a closure (x, y) -> (r, g, b).

mod register_render;
mod render_image;
mod render_frame;

#[proc_macro_attribute]
pub fn register_render(_attr: TokenStream, item: TokenStream) -> TokenStream {
    register_render::register_render_impl(_attr, item)
}

#[proc_macro]
pub fn render_image(input: TokenStream) -> TokenStream {
    render_image::render_image_impl(input)
}

#[proc_macro]
pub fn render_svg(input: TokenStream) -> TokenStream {
    render_image::render_svg_impl(input)
}

#[proc_macro]
pub fn render_frame(input: TokenStream) -> TokenStream {
    render_frame::render_frame_impl(input)
}

#[proc_macro]
pub fn render_svg_frame(input: TokenStream) -> TokenStream {
    render_frame::render_svg_frame_impl(input)
}

// Helper function to convert syn::Expr to String

use syn::Expr;
use quote::ToTokens;

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