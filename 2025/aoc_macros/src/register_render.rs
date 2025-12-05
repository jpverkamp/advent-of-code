use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote};
use syn::Expr;
use syn::parse::Parser;

pub fn register_render_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let parser = syn::punctuated::Punctuated::<Expr, syn::Token![,]>::parse_terminated;
    let args = parser
        .parse(attr.into())
        .expect("expected #[aoc::register_render(scale=N, fps=F)] (all parameters optional)");

    // Parse optional scale parameter (at index 0 if present)
    let scale_lit = if args.len() > 0 {
        // Handle both "4" and "scale = 4" syntax
        match &args[0] {
            Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(lit_int),
                ..
            }) => {
                // Direct integer: "4"
                lit_int.clone()
            }
            Expr::Assign(syn::ExprAssign { right, .. }) => {
                // Assignment: "scale = 4"
                if let Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(lit_int),
                    ..
                }) = right.as_ref()
                {
                    lit_int.clone()
                } else {
                    syn::LitInt::new("1", proc_macro2::Span::call_site())
                }
            }
            _ => syn::LitInt::new("1", proc_macro2::Span::call_site()),
        }
    } else {
        syn::LitInt::new("1", proc_macro2::Span::call_site())
    };

    // Parse optional fps parameter (at index 1 if present)
    let fps_lit = if args.len() > 1 {
        // Handle both "30" and "fps = 30" syntax
        match &args[1] {
            Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(lit_int),
                ..
            }) => {
                // Direct integer: "30"
                lit_int.clone()
            }
            Expr::Assign(syn::ExprAssign { right, .. }) => {
                // Assignment: "fps = 30"
                if let Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(lit_int),
                    ..
                }) = right.as_ref()
                {
                    lit_int.clone()
                } else {
                    syn::LitInt::new("30", proc_macro2::Span::call_site())
                }
            }
            _ => syn::LitInt::new("30", proc_macro2::Span::call_site()),
        }
    } else {
        syn::LitInt::new("30", proc_macro2::Span::call_site())
    };

    let fn_item = syn::parse_macro_input!(item as syn::ItemFn);
    let fn_name = fn_item.sig.ident.clone();
    let fn_vis = &fn_item.vis;
    let fn_sig = &fn_item.sig;
    let fn_body = &fn_item.block;
    let name_str = fn_name.to_string();

    let shim_ident: Ident = format_ident!("__aoc_render_shim_{}", fn_name);
    let entry_ident: Ident =
        format_ident!("__AOC_RENDER_ENTRY_{}", fn_name.to_string().to_uppercase());
    let reg_ident: Ident = format_ident!("__aoc_register_render_{}", fn_name);
    let module_ident: Ident = format_ident!("__aoc_render_module_{}", fn_name);

    let name_lit = name_str;

    let expanded = quote! {
        mod #module_ident {
            use super::*;

            thread_local! {
                static __AOC_RENDER_FRAMES: std::cell::RefCell<Vec<::image::RgbImage>> = std::cell::RefCell::new(Vec::new());
            }

            #[allow(non_snake_case)]
            pub fn __aoc_render_frames_push(frame: ::image::RgbImage) {
                __AOC_RENDER_FRAMES.with(|frames| {
                    log::info!("Rendering frame {}", frames.borrow().len());
                    frames.borrow_mut().push(frame);
                });
            }

            #[allow(non_snake_case)]
            pub fn __aoc_render_frames_take() -> Vec<::image::RgbImage> {
                __AOC_RENDER_FRAMES.with(|frames| {
                    frames.borrow_mut().drain(..).collect()
                })
            }
        }

        #fn_vis #fn_sig {
            use #module_ident::*;

            // Execute the original function to collect frames
            #fn_body

            // Finalize video encoding using ffmpeg via subprocess
            let frames_vec = __aoc_render_frames_take();

            if !frames_vec.is_empty() {
                let first_frame = &frames_vec[0];
                let orig_width = first_frame.width();
                let orig_height = first_frame.height();
                let scale = #scale_lit;

                let temp_dir = std::path::PathBuf::from("/tmp/aoc_render");
                std::fs::create_dir_all(&temp_dir).expect("failed to create temp dir");

                // Save frames without scaling - let ffmpeg handle it
                for (idx, frame) in frames_vec.iter().enumerate() {
                    let frame_path = temp_dir.join(format!("frame_{:04}.png", idx));
                    frame.save(&frame_path).expect("failed to save frame");
                }

                // Use ffmpeg to create video from frames with scaling
                let output_path = format!("aoc2025_{}_{}_render.mp4", crate::__aoc::DAY, stringify!(#fn_name));
                let frame_pattern = temp_dir.join("frame_%04d.png").to_string_lossy().to_string();
                let fps_str = format!("{}", #fps_lit);
                let scale_filter = format!("scale={}:{}", orig_width * scale, orig_height * scale);

                let mut cmd = std::process::Command::new("ffmpeg");
                cmd.arg("-framerate").arg(&fps_str)
                    .arg("-i").arg(&frame_pattern);

                // Add scaling filter if scale != 1
                if scale != 1 {
                    cmd.arg("-vf").arg(format!("{}:flags=neighbor", scale_filter));
                }

                log::info!("Rendering video with ffmpeg");

                cmd.arg("-c:v").arg("libx264")
                    .arg("-pix_fmt").arg("yuv420p")
                    .arg("-y")
                    .arg("-movflags")
                    .arg("+faststart")
                    .arg(&output_path);

                let output = cmd.output().expect("failed to run ffmpeg");

                if !output.status.success() {
                    log::error!("ffmpeg error: {}", String::from_utf8_lossy(&output.stderr));
                } else {
                    log::info!("ffmpeg succeeded");
                    println!("Rendered {}", output_path);
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
        static #entry_ident: crate::__aoc::RenderEntry = crate::__aoc::RenderEntry { day: crate::__aoc::DAY, name: #name_lit, func: #shim_ident };

        #[doc(hidden)]
        #[::ctor::ctor]
        fn #reg_ident() { crate::__aoc::register_render(&#entry_ident); }
    };

    TokenStream::from(expanded)
}
