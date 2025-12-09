use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::{Expr, Token};

pub fn render_image_impl(input: TokenStream) -> TokenStream {
    let parser = syn::punctuated::Punctuated::<Expr, Token![,]>::parse_terminated;
    let args = parser
        .parse(input.into())
        .expect("expected aoc::render_image!(name, width, height, closure)");

    assert!(
        args.len() == 4,
        "expected 4 arguments to aoc::render_image!(name, width, height, closure)"
    );

    let name_expr = &args[0];
    let width_expr = &args[1];
    let height_expr = &args[2];
    let closure_expr = &args[3];

    let expanded = quote! {
        {
            let width = #width_expr;
            let height = #height_expr;
            let f = #closure_expr;

            let mut img = ::image::RgbImage::new(width as u32, height as u32);
            for y in 0..height {
                for x in 0..width {
                    let (r, g, b) = f(x, y);
                    img.put_pixel(x as u32, y as u32, ::image::Rgb([r, g, b]));
                }
            }

            let filename = format!("aoc2025_{}_{}_render.png", crate::__aoc::DAY, stringify!(#name_expr));
            img.save(&filename).expect("failed to save image");
            println!("Rendered {}", filename);
        }
    };

    TokenStream::from(expanded)
}

pub fn render_svg_impl(input: TokenStream) -> TokenStream {
    let parser = syn::punctuated::Punctuated::<Expr, Token![,]>::parse_terminated;
    let args = parser
        .parse(input.into())
        .expect("expected aoc::render_svg!(name, svg_data)");

    assert!(
        args.len() == 2,
        "expected 2 arguments to aoc::render_svg!(name, svg_data)"
    );

    let name_expr = &args[0];
    let closure_expr = &args[1];

    let expanded = quote! {
        {
            let mut svg_data = #closure_expr.to_string();
            
            let filename = format!("aoc2025_{}_{}_render.svg", crate::__aoc::DAY, stringify!(#name_expr));
            std::fs::write(&filename, svg_data).expect("failed to save SVG image");
            println!("Rendered {}", filename);
        }
    };

    TokenStream::from(expanded)
}