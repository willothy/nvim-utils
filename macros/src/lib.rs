use proc_macro::TokenStream;

struct Plugin {

}

#[proc_macro_attribute]
pub fn module(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = syn::parse_macro_input!(attr as syn::AttributeArgs);

    let func = syn::parse_macro_input!(item as syn::ItemFn); 

    let name = func.sig.ident.to_string();

    let entry = format!("luaopen_{}", name);
}
