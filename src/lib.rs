extern crate jni;

pub use jni::JNIEnv;
pub use jni::objects::JClass;
use jni::objects::JString;

pub trait JvmConvertible<'a> {
    type JvmType;
    fn into_jvm_type(self, env: &'a JNIEnv) -> Self::JvmType;
    fn from_jvm_type(env: &'a JNIEnv, jvm_value: Self::JvmType) -> Self;
}

impl<'a> JvmConvertible<'a> for String {
    type JvmType = JString<'a>;

    fn into_jvm_type(self, env: &'a JNIEnv) -> Self::JvmType {
        env.new_string(self).expect("Couldn't create Java string")
    }

    fn from_jvm_type(env: &'a JNIEnv, jvm_value: Self::JvmType) -> Self {
        env.get_string(jvm_value).expect("Couldn't get Java string").into()
    }
}

macro_rules! jvm_primitives {
    ($($typ:ty),*) => {
        $(
            impl<'a> JvmConvertible<'a> for $typ {
                type JvmType = $typ;

                fn into_jvm_type(self, _env: &'a JNIEnv) -> Self::JvmType {
                    self
                }

                fn from_jvm_type(_env: &'a JNIEnv, jvm_value: Self::JvmType) -> Self {
                    jvm_value
                }
            }
        )*
    };
}

// see jni::sys for available types
jvm_primitives!(i32, i64, i8, u8, u16, i16, f32, f64, ());