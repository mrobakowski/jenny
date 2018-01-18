#![feature(proc_macro)]

extern crate jenny_derive;
extern crate jenny;

use jenny_derive::jni;

#[jni(class = "com.mrobakowski.jenny.HelloWorld")]
fn foo(x: i64, f: f32) -> f64 {
    x as f64 + f as f64
}

#[jni(class = "com.mrobakowski.jenny.HelloWorld", name = "helloWorld")]
fn hello_world() {
    println!("Hello from Rust!")
}

#[jni(class = "com.mrobakowski.jenny.HelloWorld", name = "bestLangName")]
fn best_lang_name() -> &'static str {
    "Rust"
}

#[jni(class = "com.mrobakowski.jenny.HelloWorld", name = "containsRust")]
fn contains_rust<'a>(haystack: &'a str) -> bool {
    haystack.contains("rust")
}

// This is here just as a test to check if the code compiles
#[allow(dead_code)]
fn test() {
    let _ = mod_Java_com_mrobakowski_jenny_HelloWorld_foo::Java_com_mrobakowski_jenny_HelloWorld_foo;
    let _ = mod_Java_com_mrobakowski_jenny_HelloWorld_helloWorld::Java_com_mrobakowski_jenny_HelloWorld_helloWorld;
    let _ = mod_Java_com_mrobakowski_jenny_HelloWorld_bestLangName::Java_com_mrobakowski_jenny_HelloWorld_bestLangName;
    let _ = mod_Java_com_mrobakowski_jenny_HelloWorld_containsRust::Java_com_mrobakowski_jenny_HelloWorld_containsRust;
}

/// The above functions generate the following code
#[cfg(feature = "never compile this plz")]
mod generated_code {
    fn foo(x: i64, f: f32) -> f64 { x as f64 + f as f64 }

    #[allow(non_snake_case, unused_imports)]
    pub mod mod_Java_com_mrobakowski_jenny_HelloWorld_foo {
        #[no_mangle]
        pub extern "system" fn Java_com_mrobakowski_jenny_HelloWorld_foo<'__jenny_env>(
            __jenny_jni_env: jenny::JNIEnv<'__jenny_env>,
            __jenny_jni_class: jenny::JClass,
            __jenny_arg_0: <i64 as jenny::FromJvmValue<'__jenny_env>>::JvmValue,
            __jenny_arg_1: <f32 as jenny::FromJvmValue<'__jenny_env>>::JvmValue,
        ) -> <f64 as jenny::IntoJvmValue<'__jenny_env>>::JvmValue {
            use jenny::{FromJvmValue, BorrowFromJvmValue, IntoJvmValue};
            let __jenny_arg_0 = i64::from_jvm_type(&__jenny_jni_env, __jenny_arg_0);
            let __jenny_arg_1 = f32::from_jvm_type(&__jenny_jni_env, __jenny_arg_1);
            foo(__jenny_arg_0, __jenny_arg_1).into_jvm_type(&__jenny_jni_env)
        }
    }

    fn hello_world() { println!("Hello from Rust!") }

    #[allow(non_snake_case, unused_imports)]
    pub mod mod_Java_com_mrobakowski_jenny_HelloWorld_helloWorld {
        #[no_mangle]
        pub extern "system" fn Java_com_mrobakowski_jenny_HelloWorld_helloWorld<'__jenny_env>(
            __jenny_jni_env: jenny::JNIEnv<'__jenny_env>,
            __jenny_jni_class: jenny::JClass,
        ) -> () {
            use jenny::{FromJvmValue, BorrowFromJvmValue, IntoJvmValue};
            hello_world().into_jvm_type(&__jenny_jni_env)
        }
    }

    fn best_lang_name() -> &'static str { "Rust" }

    #[allow(non_snake_case, unused_imports)]
    pub mod mod_Java_com_mrobakowski_jenny_HelloWorld_bestLangName {
        #[no_mangle]
        pub extern "system" fn Java_com_mrobakowski_jenny_HelloWorld_bestLangName<'__jenny_env>(
            __jenny_jni_env: jenny::JNIEnv<'__jenny_env>,
            __jenny_jni_class: jenny::JClass,
        ) -> <&'static str as jenny::IntoJvmValue<'__jenny_env>>::JvmValue {
            use jenny::{FromJvmValue, BorrowFromJvmValue, IntoJvmValue};
            best_lang_name().into_jvm_type(&__jenny_jni_env)
        }
    }

    fn contains_rust<'a>(haystack: &'a str) -> bool { haystack.contains("rust") }

    #[allow(non_snake_case, unused_imports)]
    pub mod mod_Java_com_mrobakowski_jenny_HelloWorld_containsRust {
        #[no_mangle]
        pub extern "system" fn Java_com_mrobakowski_jenny_HelloWorld_containsRust<'__jenny_env, 'a>(
            __jenny_jni_env: jenny::JNIEnv<'__jenny_env>,
            __jenny_jni_class: jenny::JClass,
            __jenny_arg_0: <<str as jenny::BorrowFromJvmValue<'__jenny_env>>::Impl as jenny::BorrowFromJvmValueImpl<'__jenny_env>>::JvmValue,
        ) -> <bool as jenny::IntoJvmValue<'__jenny_env>>::JvmValue {
            use jenny::{FromJvmValue, BorrowFromJvmValue, IntoJvmValue};
            let __jenny_arg_0 = str::jvm_type_into_tmp(&__jenny_jni_env, __jenny_arg_0);
            contains_rust(<str as BorrowFromJvmValue>::tmp_as_ref(&__jenny_arg_0)).into_jvm_type(&__jenny_jni_env)
        }
    }
}