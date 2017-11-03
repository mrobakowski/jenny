extern crate jni;

pub use jni::JNIEnv;
pub use jni::objects::JClass;
use jni::objects::JString;

pub trait FromJvmType<'jni> {
    type JvmType;
    fn from_jvm_type<'a>(env: &'a JNIEnv<'jni>, jvm_value: Self::JvmType) -> Self;
}

pub trait IntoJvmType<'jni> {
    type JvmType;
    fn into_jvm_type<'a>(self, env: &'a JNIEnv<'jni>) -> Self::JvmType;
}

impl<'jni> IntoJvmType<'jni> for String {
    type JvmType = JString<'jni>;

    fn into_jvm_type<'a>(self, env: &'a JNIEnv<'jni>) -> Self::JvmType {
        env.new_string(self).expect("Couldn't create Java string")
    }
}

impl<'jni> FromJvmType<'jni> for String {
    type JvmType = JString<'jni>;

    fn from_jvm_type<'a>(env: &'a JNIEnv<'jni>, jvm_value: Self::JvmType) -> Self {
        env.get_string(jvm_value).expect("Couldn't get Java string").into()
    }
}

impl<'jni, 's> IntoJvmType<'jni> for &'s str {
    type JvmType = JString<'jni>;

    fn into_jvm_type<'a>(self, env: &'a JNIEnv<'jni>) -> Self::JvmType {
        env.new_string(self).expect("Couldn't create Java string")
    }
}

impl<'jni> FromJvmType<'jni> for bool {
    type JvmType = jni::sys::jboolean;

    fn from_jvm_type<'a>(_env: &'a JNIEnv<'jni>, jvm_value: Self::JvmType) -> Self {
        jvm_value == 1
    }
}

impl<'jni> IntoJvmType<'jni> for bool {
    type JvmType = jni::sys::jboolean;

    fn into_jvm_type<'a>(self, _env: &'a JNIEnv<'jni>) -> Self::JvmType {
        if self { 1 } else { 0 }
    }
}

macro_rules! jvm_primitives {
    ($($typ:ty),*) => {
        $(
            impl<'jni> IntoJvmType<'jni> for $typ {
                type JvmType = $typ;

                fn into_jvm_type<'a>(self, _env: &'a JNIEnv<'jni>) -> Self::JvmType {
                    self
                }
            }

            impl<'jni> FromJvmType<'jni> for $typ {
                type JvmType = $typ;

                fn from_jvm_type<'a>(_env: &'a JNIEnv<'jni>, jvm_value: Self::JvmType) -> Self {
                    jvm_value
                }
            }
        )*
    };
}

// see jni::sys for available types
jvm_primitives!(i32, i64, i8, u8, u16, i16, f32, f64, ());