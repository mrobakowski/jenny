#![feature(proc_macro)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate quote;
extern crate syn;
extern crate proc_macro;
extern crate case;

use case::CaseExt;
use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn jni(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = syn::parse_item(&item.to_string()).unwrap();
    let jni_func = generate_jni_func(&func);

    let res = quote! {
        #func
        #jni_func
    };

    res.parse().unwrap()
}

fn generate_jni_func(source: &syn::Item) -> quote::Tokens {
    if let syn::Item { ref ident, node: syn::ItemKind::Fn(ref decl, ..), .. } = *source {
        let name = jni_name(ident.as_ref(), decl.inputs.as_ref());
        let mod_name = format!("mod_{}", name);
        let args = jni_args(decl.inputs.as_ref());
        let ret = jni_ret(&decl.output);
        let body = jni_body(ident.as_ref(), decl);
        quote! {
            mod #mod_name {
                #[no_mangle]
                extern "C" fn #name(#(#args),*) -> #ret {
                    #body
                }
            }
        }
    } else {
        panic!("eeeeh")
    }
}

fn jni_name(rust_name: &str, rust_args: &[syn::FnArg]) -> String {
    // TODO: do something better here, preferably user defined
    let package_name = "rust_jenny";

    // see Table 2-1 from http://docs.oracle.com/javase/7/docs/technotes/guides/jni/spec/design.html
    // for the escape codes
    // TODO: do something user defined if user asks
    let class_name = rust_name.to_camel().replace("_", "_1");

    // TODO: support renaming
    let func_name = rust_name.replace("_", "_1");
    let func_signature = jni_signature(rust_args);

    format!("Java_{pkg}_{cls}_{func}__{sig}",
            pkg = package_name,
            cls = class_name,
            func = func_name,
            sig = func_signature)
}

fn jni_signature(rust_args: &[syn::FnArg]) -> String {
    println!("rust_args = {:#?}", rust_args);
    unimplemented!()
}

fn jni_args(rust_args: &[syn::FnArg]) -> Vec<quote::Tokens> {
    unimplemented!()
}

fn jni_ret(rust_ret: &syn::FunctionRetTy) -> quote::Tokens {
    unimplemented!()
}

fn jni_body(rust_name: &str, rust_args: &syn::FnDecl) -> quote::Tokens {
    unimplemented!()
}