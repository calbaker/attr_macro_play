use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};

#[proc_macro_attribute]
pub fn api(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = syn::parse_macro_input!(input as syn::ItemStruct);
    // println!("{:?}", ast.to_token_stream());

    let mut impl_block = TokenStream2::new();

    if let syn::Fields::Named(syn::FieldsNamed { named, .. }) = &mut ast.fields {
        for field in named.iter_mut() {
            let ident = field.ident.as_ref().unwrap();
            let ftype = field.ty.clone();
            let get_func_name: TokenStream2 = format!("get_{}", ident).parse().unwrap();

            let mut skip_get = false;

            let keep: Vec<bool> = field
                .attrs
                .iter()
                .map(|x| match x.path.segments[0].ident.to_string().as_str() {
                    "api" => {
                        skip_get = true;
                        false
                    }
                    _ => true,
                })
                .collect();
            let mut iter = keep.iter();
            println!("{:?}", keep);

            field.attrs.retain(|_| *iter.next().unwrap());

            if !skip_get {
                let get_block = quote! {
                    pub fn #get_func_name(&self) -> &#ftype {
                        &self.#ident
                    }
                };
                impl_block.extend::<TokenStream2>(get_block);
            }
        }
    };

    let ident = &ast.ident;
    let impl_block = quote! {
        impl #ident {
            #impl_block
        }
    };

    let mut output: TokenStream2 = ast.to_token_stream();

    output.extend(impl_block);
    // println!(
    //     "
    // final output {}",
    //     "*".to_string().repeat(25)
    // );
    // println!("{}", output);
    output.into()
}
