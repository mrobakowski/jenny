#![feature(proc_macro)]

extern crate lazy_static;
#[macro_use]
extern crate quote;
extern crate syn;
extern crate proc_macro;
extern crate case;

use case::CaseExt;
use proc_macro::TokenStream;
use quote::Ident;

#[proc_macro_attribute]
pub fn jni(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = syn::parse_item(&item.to_string()).unwrap();
    let jni_func = generate_jni_func(&func);

    let res: quote::Tokens = quote! {
        #func
        #jni_func
    };

    println!("res = {:?}", res);

    res.parse().unwrap()
}

fn generate_jni_func(source: &syn::Item) -> quote::Tokens {
    if let syn::Item { ref ident, node: syn::ItemKind::Fn(ref decl, ..), .. } = *source {
        let name = Ident::new(jni_name(ident.as_ref(), decl.inputs.as_ref()));
        let mod_name = Ident::new(format!("mod_{}", name));
        let args = jni_args(decl.inputs.as_ref());
        let ret = jni_ret(&decl.output);
        let body = jni_body(ident.as_ref(), decl);
        quote! {
            #[allow(non_snake_case)]
            pub mod #mod_name {
                #[no_mangle]
                pub extern "system" fn #name(__jenny_jni_env: jenny::JNIEnv, __jenny_jni_class: jenny::JClass, #(#args),*) -> #ret {
                    #body
                }
            }
        }
    } else {
        panic!("Jenny supports only functions for now")
    }
}

fn jni_name(rust_name: &str, _rust_args: &[syn::FnArg]) -> String {
    // TODO: do something better here, preferably user defined
    let package_name = "rust_jenny";

    // see Table 2-1 from http://docs.oracle.com/javase/7/docs/technotes/guides/jni/spec/design.html
    // for the escape codes
    // TODO: do something user defined if user asks
    let class_name = rust_name.to_camel().replace("_", "_1");

    // TODO: support renaming
    let func_name = rust_name.replace("_", "_1");

    // TODO: figure out how to make signatures work in case of overloaded methods
    // let func_signature = jni_signature(rust_args);

    format!(
        // "Java_{pkg}_{cls}_{func}__{sig}", // TODO: signature
        "Java_{pkg}_{cls}_{func}",
        pkg = package_name,
        cls = class_name,
        func = func_name,
        // sig = func_signature // TODO: signature
    )
}

// TODO: signature
fn jni_signature(rust_args: &[syn::FnArg]) -> String {
    println!("rust_args = {:#?}", rust_args);
    unimplemented!("jni_signature")
}

fn jni_args(rust_args: &[syn::FnArg]) -> Vec<quote::Tokens> {
    rust_args.iter().enumerate().map(|(i, a)| {
        use syn::FnArg::*;
        match (a, syn::Pat::Wild) {
            (&Captured(_, ref typ), _) | (&Ignored(ref typ), _) => {
                let arg_name = Ident::new(format!("__jenny_arg_{}", i));
                quote!(#arg_name: <#typ as jenny::JvmConvertible>::JvmType)
            }
            (&SelfRef(..), _) | (&SelfValue(..), _) => panic!("Self arguments are not yet supported by jenny!")
        }
    }).collect()
}

fn jni_ret(rust_ret: &syn::FunctionRetTy) -> quote::Tokens {
    use syn::FunctionRetTy::*;
    match *rust_ret {
        Default => quote!(()),
        Ty(ref typ) => quote!(<#typ as jenny::JvmConvertible<'static>>::JvmType)
    }
}

fn jni_body(rust_name: &str, rust_args: &syn::FnDecl) -> quote::Tokens {
    let rust_name = Ident::new(rust_name);

    let (arg_conversions, arg_names): (Vec<quote::Tokens>, Vec<_>) = rust_args.inputs.iter().enumerate().map(|(i, arg)| {
        use syn::FnArg::*;
        let name = Ident::new(format!("__jenny_arg_{}", i));
        let typ = match *arg {
            Captured(_, ref t) | Ignored(ref t) => t,
            SelfValue(..) | SelfRef(..) => panic!("Self arguments are not yet supported by jenny!")
        };
        (quote!(let #name = #typ::from_jvm_type(&__jenny_jni_env, #name);), name)
    }).unzip();

    quote! {
        use jenny::JvmConvertible;

        // argument conversions
        #(#arg_conversions)*

        // actual call
        #rust_name(#(#arg_names),*).into_jvm_type(&__jenny_jni_env)
    }
}