use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, Pat};

pub fn tool_impl(item: TokenStream, trait_path: proc_macro2::TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemFn);

    // Extract function doc comment for tool description
    let fn_doc = input
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("doc"))
        .and_then(|attr| attr.meta.require_name_value().ok())
        .and_then(|nv| match &nv.value {
            syn::Expr::Lit(lit) => match &lit.lit {
                syn::Lit::Str(s) => Some(s.value().trim().to_string()),
                _ => None,
            },
            _ => None,
        })
        .unwrap_or_default();

    // Generate struct name: tool_name -> ToolNameArgs
    let fn_name = &input.sig.ident;
    let struct_name = syn::Ident::new(
        &format!("{}Args", to_pascal_case(&fn_name.to_string())),
        fn_name.span(),
    );

    // Extract parameters with their doc comments and names
    let (fields, param_names): (Vec<_>, Vec<_>) = input
        .sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(pat_type) => {
                let Pat::Ident(ident) = &*pat_type.pat else {
                    return None;
                };
                let ty = &pat_type.ty;
                let attrs = &pat_type.attrs;
                Some((
                    quote! {
                        #(#attrs)*
                        pub #ident: #ty
                    },
                    ident.clone(),
                ))
            }
            _ => None,
        })
        .unzip();

    // Replace function parameters with single args parameter
    input.sig.inputs.clear();
    input.sig.inputs.push(syn::parse_quote!(args: #struct_name));

    // Destructure args at start of function body
    let destructure = quote! {
        let #struct_name { #(#param_names),* } = args;
    };

    let block = &input.block;
    input.block = syn::parse_quote!({
        #destructure
        #block
    });

    let vis = &input.vis;
    let sig = &input.sig;
    let new_block = &input.block;
    let fn_attrs = &input.attrs;

    // Generate args struct, trait impl, and modified function
    let expanded = quote! {
        #[doc = concat!("Arguments for the `", stringify!(#fn_name), "` tool.")]
        #[derive(serde::Deserialize, schemars::JsonSchema)]
        pub struct #struct_name {
            #(#fields),*
        }

        impl #trait_path for #struct_name {
            const TOOL_NAME: &'static str = stringify!(#fn_name);
            const TOOL_DESCRIPTION: &'static str = #fn_doc;
        }

        #(#fn_attrs)*
        #vis #sig #new_block
    };

    TokenStream::from(expanded)
}

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect()
}
