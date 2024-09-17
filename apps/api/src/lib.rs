use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn authenticated(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let expanded = authenticated_impl(input);
    TokenStream::from(expanded)
}

fn authenticated_impl(input: ItemFn) -> TokenStream2 {
    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;

    quote! {
        #vis #sig {
            use crate::graphql::auth::TokenVerifier;

            let token_verifier = ctx.data::<TokenVerifier>().map_err(|e| {
                tracing::error!("Failed to get token verifier: {:?}", e);
                AppError::InternalError.extend()
            })?;

            let user_id = token_verifier.verify_access_token(ctx).await?;

            #block
        }
    }
}
