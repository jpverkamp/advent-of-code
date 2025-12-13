use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::ItemFn;
use syn::parse_macro_input;

pub fn register_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let fn_name = func.sig.ident.clone();
    let name_str = fn_name.to_string();

    let entry_ident: Ident = quote::format_ident!("__AOC_ENTRY_{}", fn_name.to_string().to_uppercase());
    let shim_ident: Ident = quote::format_ident!("__aoc_shim_{}", fn_name);
    let reg_ident: Ident = quote::format_ident!("__aoc_register_{}", fn_name);

    let name_lit = name_str;

    let expanded = quote! {
        #func

        #[doc(hidden)]
        fn #shim_ident(input: &str) -> String {
            let start = std::time::Instant::now();
            let result = #fn_name(input).into();
            let elapsed = start.elapsed();
            log::info!("{} took {:?}", stringify!(#fn_name), elapsed);
            result
        }

        #[doc(hidden)]
        static #entry_ident: crate::__aoc::Entry = crate::__aoc::Entry { day: crate::__aoc::DAY, name: #name_lit, func: #shim_ident };

        #[doc(hidden)]
        #[::ctor::ctor]
        fn #reg_ident() { crate::__aoc::register(&#entry_ident); }
    };

    TokenStream::from(expanded)
}
