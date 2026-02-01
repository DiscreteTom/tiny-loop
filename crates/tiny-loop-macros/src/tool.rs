use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, FnArg, ImplItem, ItemFn, ItemImpl, Pat};

struct ArgsStruct {
    name: syn::Ident,
    fields: syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
    tool_name: String,
    tool_description: String,
}

struct ToolAttr {
    name: Option<String>,
}

fn parse_tool_attr(attr: TokenStream) -> ToolAttr {
    if attr.is_empty() {
        return ToolAttr { name: None };
    }

    let mut result = ToolAttr { name: None };

    let parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("name") {
            let value = meta.value()?;
            let s: syn::LitStr = value.parse()?;
            result.name = Some(s.value());
        }
        Ok(())
    });

    let _ = parser.parse(attr);
    result
}

pub fn tool_impl(
    attr: TokenStream,
    item: TokenStream,
    trait_path: proc_macro2::TokenStream,
) -> TokenStream {
    let tool_attr = parse_tool_attr(attr);

    // Try parsing as impl block first
    if let Ok(impl_block) = syn::parse::<ItemImpl>(item.clone()) {
        return tool_impl_block(impl_block, trait_path, tool_attr);
    }

    // Otherwise parse as function
    let input = parse_macro_input!(item as ItemFn);
    tool_impl_fn(input, trait_path, tool_attr)
}

fn tool_impl_block(
    mut impl_block: ItemImpl,
    trait_path: proc_macro2::TokenStream,
    _tool_attr: ToolAttr,
) -> TokenStream {
    let mut args_structs = Vec::new();

    for item in &mut impl_block.items {
        if let ImplItem::Fn(method) = item {
            // Validate return type
            if let Err(err) = validate_return_type(&method.sig) {
                return TokenStream::from(err.to_compile_error());
            }

            // Parse name attribute from method attributes and remove it
            let mut method_name = None;
            method.attrs.retain(|attr| {
                if attr.path().is_ident("name") {
                    if let syn::Meta::NameValue(nv) = &attr.meta {
                        if let syn::Expr::Lit(lit) = &nv.value {
                            if let syn::Lit::Str(s) = &lit.lit {
                                method_name = Some(s.value());
                            }
                        }
                    }
                    false // Remove the name attribute
                } else {
                    true // Keep other attributes
                }
            });

            let args_struct =
                extract_args_struct(&method.sig, &method.attrs, method_name.as_deref());
            let struct_name = &args_struct.name;
            let param_names: Vec<_> = args_struct
                .fields
                .iter()
                .filter_map(|f| f.ident.as_ref().cloned())
                .collect();

            // Modify signature
            let self_param = method.sig.inputs.iter().find_map(|arg| match arg {
                FnArg::Receiver(_) => Some(arg.clone()),
                _ => None,
            });
            method.sig.inputs.clear();
            if let Some(self_param) = self_param {
                method.sig.inputs.push(self_param);
            }
            method
                .sig
                .inputs
                .push(syn::parse_quote!(args: #struct_name));

            // Add destructuring
            let destructure = quote! {
                let #struct_name { #(#param_names),* } = args;
            };
            let block = &method.block;
            method.block = syn::parse_quote!({
                #destructure
                #block
            });

            args_structs.push(args_struct);
        }
    }

    let struct_defs: Vec<_> = args_structs
        .iter()
        .map(|s| {
            let name = &s.name;
            let fields = &s.fields;
            let tool_name = &s.tool_name;
            let tool_description = &s.tool_description;
            quote! {
                #[doc = concat!("Arguments for the `", #tool_name, "` tool.")]
                #[derive(serde::Deserialize, schemars::JsonSchema)]
                pub struct #name {
                    #fields
                }

                impl #trait_path for #name {
                    const TOOL_NAME: &'static str = #tool_name;
                    const TOOL_DESCRIPTION: &'static str = #tool_description;
                }
            }
        })
        .collect();

    let expanded = quote! {
        #(#struct_defs)*
        #impl_block
    };

    TokenStream::from(expanded)
}

fn tool_impl_fn(
    mut input: ItemFn,
    trait_path: proc_macro2::TokenStream,
    tool_attr: ToolAttr,
) -> TokenStream {
    let args_struct = extract_args_struct(&input.sig, &input.attrs, tool_attr.name.as_deref());

    // Validate return type
    if let Err(err) = validate_return_type(&input.sig) {
        return TokenStream::from(err.to_compile_error());
    }

    let struct_name = &args_struct.name;
    let param_names: Vec<_> = args_struct
        .fields
        .iter()
        .filter_map(|f| f.ident.as_ref().cloned())
        .collect();

    // Modify signature
    let self_param = input.sig.inputs.iter().find_map(|arg| match arg {
        FnArg::Receiver(_) => Some(arg.clone()),
        _ => None,
    });
    input.sig.inputs.clear();
    if let Some(self_param) = self_param {
        input.sig.inputs.push(self_param);
    }
    input.sig.inputs.push(syn::parse_quote!(args: #struct_name));

    // Add destructuring
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
    let fields = &args_struct.fields;
    let tool_name = &args_struct.tool_name;
    let tool_description = &args_struct.tool_description;

    let expanded = quote! {
        #[doc = concat!("Arguments for the `", #tool_name, "` tool.")]
        #[derive(serde::Deserialize, schemars::JsonSchema)]
        pub struct #struct_name {
            #fields
        }

        impl #trait_path for #struct_name {
            const TOOL_NAME: &'static str = #tool_name;
            const TOOL_DESCRIPTION: &'static str = #tool_description;
        }

        #(#fn_attrs)*
        #vis #sig #new_block
    };

    TokenStream::from(expanded)
}

fn extract_args_struct(
    sig: &syn::Signature,
    attrs: &[syn::Attribute],
    override_name: Option<&str>,
) -> ArgsStruct {
    let fn_name = &sig.ident;
    let tool_name = override_name.unwrap_or(&fn_name.to_string()).to_string();
    let struct_name = syn::Ident::new(
        &format!("{}Args", to_pascal_case(&tool_name)),
        fn_name.span(),
    );

    let fn_doc = attrs
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

    let fields: syn::punctuated::Punctuated<syn::Field, syn::token::Comma> = sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(pat_type) => {
                let Pat::Ident(ident) = &*pat_type.pat else {
                    return None;
                };
                Some(syn::Field {
                    attrs: pat_type.attrs.clone(),
                    vis: syn::Visibility::Public(syn::token::Pub::default()),
                    mutability: syn::FieldMutability::None,
                    ident: Some(ident.ident.clone()),
                    colon_token: Some(syn::token::Colon::default()),
                    ty: (*pat_type.ty).clone(),
                })
            }
            _ => None,
        })
        .collect();

    ArgsStruct {
        name: struct_name,
        fields,
        tool_name,
        tool_description: fn_doc,
    }
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

fn validate_return_type(sig: &syn::Signature) -> Result<(), syn::Error> {
    use syn::{ReturnType, Type, TypePath};

    match &sig.output {
        ReturnType::Default => Err(syn::Error::new_spanned(
            sig,
            "Tool function must return String, but returns ()",
        )),
        ReturnType::Type(_, ty) => {
            // Check if type is String (std::string::String or any path ending with String)
            if let Type::Path(TypePath { path, .. }) = &**ty {
                if let Some(last_seg) = path.segments.last() {
                    if last_seg.ident == "String" {
                        return Ok(());
                    }
                }
            }
            Err(syn::Error::new_spanned(
                ty,
                "Tool function must return String",
            ))
        }
    }
}
