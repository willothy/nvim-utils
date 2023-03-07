#[allow(unused_imports)]
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{AttributeArgs, Error, Ident, Path, Result};

struct Plugin {
    path: Path,
}

impl Plugin {
    fn parse(args: AttributeArgs) -> Result<Self> {
        let mut path = None;

        for arg in args {
            use syn::Meta::Path;
            use syn::NestedMeta::*;
            match arg {
                Meta(Path(p)) => path = Some(p),
                _ => {
                    return Err(Error::new_spanned(arg, "expected `name = \"...\"`"));
                }
            }
        }

        let path = path.ok_or_else(|| Error::new(Span::call_site(), "expected module path"))?;

        Ok(Self { path })
    }
}

#[proc_macro_attribute]
pub fn module(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = syn::parse_macro_input!(attr as AttributeArgs);
    let plugin = match Plugin::parse(attr) {
        Ok(plugin) => plugin,
        Err(err) => return err.to_compile_error().into(),
    };

    let path = plugin
        .path
        .segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect::<Vec<_>>()
        .join("_");

    let func = syn::parse_macro_input!(item as syn::ItemFn);
    let name = func.sig.ident.clone();

    let entry = Ident::new(&format!("luaopen_{path}"), Span::call_site());
    let wrapped = quote! {
        #func

        #[no_mangle]
        unsafe extern "C" fn #entry(state: *mut ::luajit::ffi::LuaState) -> std::os::raw::c_int {
            ::luajit::entry::entry(state, #name)
        }
    };

    wrapped.into()
}
