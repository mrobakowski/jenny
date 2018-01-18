#![feature(specialization, nll)]
extern crate jni;

pub use jni::JNIEnv;
pub use jni::objects::JClass;
use jni::objects::JString;

pub trait FromJvmValue<'jni> {
    type JvmValue;
    fn from_jvm_type<'a>(env: &'a JNIEnv<'jni>, jvm_value: Self::JvmValue) -> Self;
}

pub trait BorrowFromJvmValue<'jni> {
    type Impl: BorrowFromJvmValueImpl<'jni, This=Self>; // see the comment on [BorrowFromJvmValueImpl] as to why this is here

    #[inline]
    fn jvm_type_into_tmp<'a>(
        env: &'a JNIEnv<'jni>,
        jvm_value: <Self::Impl as BorrowFromJvmValueImpl<'jni>>::JvmValue,
    ) -> <Self::Impl as BorrowFromJvmValueImpl<'jni>>::TmpStorage {
        <Self::Impl as BorrowFromJvmValueImpl<'jni>>::jvm_type_into_tmp(env, jvm_value)
    }

    #[inline]
    fn tmp_as_ref<'a>(tmp: &'a <Self::Impl as BorrowFromJvmValueImpl<'jni>>::TmpStorage) -> &'a Self {
        <Self::Impl as BorrowFromJvmValueImpl<'jni>>::tmp_as_ref(tmp)
    }
}

// The trait exists because of specialization. I needed to somehow entangle the default associated types and the default fn impls together.
pub trait BorrowFromJvmValueImpl<'jni> {
    type JvmValue;
    type TmpStorage;
    type This: ? Sized;
    fn jvm_type_into_tmp<'a>(env: &'a JNIEnv<'jni>, jvm_value: Self::JvmValue) -> Self::TmpStorage;
    fn tmp_as_ref<'a>(tmp: &'a Self::TmpStorage) -> &'a Self::This;
}

pub trait IntoJvmValue<'jni> {
    type JvmValue;
    fn into_jvm_type<'a>(self, env: &'a JNIEnv<'jni>) -> Self::JvmValue;
}

impl<'jni> IntoJvmValue<'jni> for String {
    type JvmValue = JString<'jni>;

    #[inline]
    fn into_jvm_type<'a>(self, env: &'a JNIEnv<'jni>) -> Self::JvmValue {
        env.new_string(self).expect("Couldn't create Java string")
    }
}

impl<'jni> FromJvmValue<'jni> for String {
    type JvmValue = JString<'jni>;

    #[inline]
    fn from_jvm_type<'a>(env: &'a JNIEnv<'jni>, jvm_value: Self::JvmValue) -> Self {
        env.get_string(jvm_value).expect("Couldn't get Java string").into()
    }
}

impl<'jni, 's> IntoJvmValue<'jni> for &'s str {
    type JvmValue = JString<'jni>;

    #[inline]
    fn into_jvm_type<'a>(self, env: &'a JNIEnv<'jni>) -> Self::JvmValue {
        env.new_string(self).expect("Couldn't create Java string")
    }
}

impl<'jni, 'a, T> IntoJvmValue<'jni> for &'a T
    where T: ToOwned, <T as ToOwned>::Owned: IntoJvmValue<'jni> {
    type JvmValue = <<T as ToOwned>::Owned as IntoJvmValue<'jni>>::JvmValue;

    #[inline]
    fn into_jvm_type<'b>(self, env: &'b JNIEnv<'jni>) -> Self::JvmValue {
        // TODO: don't copy in certain cases? maybe add an additional trait that takes &self instead of self, and use that if available? (look at the impl for &str above)
        self.to_owned().into_jvm_type(env)
    }
}

pub struct StrBorrowFromJvmValueImpl;

impl<'jni> BorrowFromJvmValueImpl<'jni> for StrBorrowFromJvmValueImpl {
    type JvmValue = JString<'jni>;
    type TmpStorage = String;
    type This = str;

    #[inline]
    fn jvm_type_into_tmp<'a>(env: &'a JNIEnv<'jni>, jvm_value: Self::JvmValue) -> Self::TmpStorage {
        <String as FromJvmValue<'jni>>::from_jvm_type(env, jvm_value)
    }

    #[inline]
    fn tmp_as_ref<'a>(tmp: &'a Self::TmpStorage) -> &'a Self::This {
        use std::borrow::Borrow;
        tmp.borrow()
    }
}

impl<'jni> BorrowFromJvmValue<'jni> for str {
    type Impl = StrBorrowFromJvmValueImpl;
}

use std::marker::PhantomData;

pub struct DirectBorrowFromJvmValueImpl<T>(PhantomData<T>);

impl<'jni, T> BorrowFromJvmValueImpl<'jni> for DirectBorrowFromJvmValueImpl<T>
    where T: FromJvmValue<'jni> {
    type JvmValue = <T as FromJvmValue<'jni>>::JvmValue;
    type TmpStorage = T;
    type This = T;

    #[inline]
    fn jvm_type_into_tmp<'a>(env: &'a JNIEnv<'jni>, jvm_value: Self::JvmValue) -> Self::TmpStorage {
        <T as FromJvmValue<'jni>>::from_jvm_type(env, jvm_value)
    }

    #[inline]
    fn tmp_as_ref<'a>(tmp: &'a Self::TmpStorage) -> &'a Self::This {
        tmp
    }
}

pub struct ToOwnedBorrowFromJvmValueImpl<T>(PhantomData<T>);

impl<'jni, T> BorrowFromJvmValueImpl<'jni> for ToOwnedBorrowFromJvmValueImpl<T>
    where T: ToOwned, <T as ToOwned>::Owned: FromJvmValue<'jni> {
    type JvmValue = <<T as ToOwned>::Owned as FromJvmValue<'jni>>::JvmValue;
    type TmpStorage = <T as ToOwned>::Owned;
    type This = T;

    #[inline]
    fn jvm_type_into_tmp<'a>(env: &'a JNIEnv<'jni>, jvm_value: Self::JvmValue) -> Self::TmpStorage {
        <Self::TmpStorage as FromJvmValue<'jni>>::from_jvm_type(env, jvm_value)
    }

    #[inline]
    fn tmp_as_ref<'a>(tmp: &'a Self::TmpStorage) -> &'a Self::This {
        use std::borrow::Borrow;
        tmp.borrow()
    }
}

impl<'jni, T> BorrowFromJvmValue<'jni> for T
    where T: ToOwned, <T as ToOwned>::Owned: FromJvmValue<'jni> {
    default type Impl = ToOwnedBorrowFromJvmValueImpl<T>;
}


impl<'jni> FromJvmValue<'jni> for bool {
    type JvmValue = jni::sys::jboolean;

    #[inline]
    fn from_jvm_type<'a>(_env: &'a JNIEnv<'jni>, jvm_value: Self::JvmValue) -> Self {
        jvm_value == 1
    }
}

impl<'jni> IntoJvmValue<'jni> for bool {
    type JvmValue = jni::sys::jboolean;

    #[inline]
    fn into_jvm_type<'a>(self, _env: &'a JNIEnv<'jni>) -> Self::JvmValue {
        if self { 1 } else { 0 }
    }
}

macro_rules! jvm_primitives {
    ($($typ:ty),*) => {
        $(
            impl<'jni> IntoJvmValue<'jni> for $typ {
                type JvmValue = $typ;

                fn into_jvm_type<'a>(self, _env: &'a JNIEnv<'jni>) -> Self::JvmValue {
                    self
                }
            }

            impl<'jni> FromJvmValue<'jni> for $typ {
                type JvmValue = $typ;

                fn from_jvm_type<'a>(_env: &'a JNIEnv<'jni>, jvm_value: Self::JvmValue) -> Self {
                    jvm_value
                }
            }
        )*
    };
}

// see jni::sys for available types
jvm_primitives!(
    i32, i64, i8, u8, u16, i16, f32, f64, ()
);

macro_rules! impl_direct_borrow_from_jvm_type {
    ($($typ:ty),*) => {
        $(
            impl<'jni> BorrowFromJvmValue<'jni> for $typ {
                type Impl = DirectBorrowFromJvmValueImpl<$typ>;
            }
        )*
    };
}

impl_direct_borrow_from_jvm_type!(
    i32, i64, i8, u8, u16, i16, f32, f64, (), String, bool
);

macro_rules! assert_impl {
    (@gen < $($generics:tt),+ >; $x:ty, $($t:path),+ $(,)*) => {
        {
            fn assert_impl< $($generics),+, T>() where T: ?Sized $(+ $t)+ {}
            assert_impl::<$x>();
        }
    };
    ($x:ty, $($t:path),+ $(,)*) => {
        {
            fn assert_impl<T>() where T: ?Sized $(+ $t)+ {}
            assert_impl::<$x>();
        }
    };
    ($label:ident; $($xs:tt)+) => {
        #[allow(dead_code, non_snake_case)]
        fn $label() { assert_impl!($($xs)+); }
    };
    ($label:ident < $($generics:tt),+ >; $($xs:tt)+) => {
        #[allow(dead_code, non_snake_case)]
        fn $label<$($generics),+>() { assert_impl!(@gen <$($generics),+>; $($xs)+); }
    };
}

assert_impl!(str<'jni>; str, BorrowFromJvmValue<'jni>);
assert_impl!(refString<'jni>; String, BorrowFromJvmValue<'jni>);