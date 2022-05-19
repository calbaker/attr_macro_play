use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};

#[proc_macro_attribute]
pub fn api(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut output: TokenStream2 = input.clone().into();
    let mut ast = syn::parse_macro_input!(input as syn::ItemStruct);
    
    let mut impl_block = TokenStream2::new();

    if let syn::Fields::Named(syn::FieldsNamed { mut named, .. }) = ast.fields {
        for field in named.iter_mut() {
            let ident = field.ident.as_ref().unwrap();
            let ftype = field.ty.clone();
            let get_func_name: TokenStream2 = format!("get_{}", ident).parse().unwrap();

            for attr in &mut field.attrs {
                match attr.path.segments[0].ident.to_string().as_str() {
                    "api" => {}
                    _ => {}

                }
    
            }
            let get_block = quote! {
                pub fn #get_func_name(&self) -> &#ftype {
                    &self.#ident
                }
            };
            impl_block.extend::<TokenStream2>(get_block);
        }
    };

    let ident = ast.ident;
    let impl_block = quote! {
        impl #ident {
            #impl_block
        }
    };

    output.extend(impl_block);
    println!("{}", output);
    output.into()
}
